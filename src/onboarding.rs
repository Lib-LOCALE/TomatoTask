use tomatotask::db::{CreateTaskInput, DbConnection, Task, queries};

pub fn pending(db: &DbConnection) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|err| err.to_string())?;
    Ok(conn.query_row(
        "SELECT NOT completed FROM onboarding WHERE id = 1",
        [],
        |row| row.get(0),
    )?)
}

pub fn valid_title(title: &str) -> bool {
    let title = title.trim();
    !title.is_empty() && title.chars().count() <= 200
}

pub fn finish(db: &DbConnection, title: Option<&str>) -> Result<Option<Task>, String> {
    let conn = db.get_connection();
    let mut conn = conn.lock().map_err(|err| err.to_string())?;
    let tx = conn.transaction().map_err(|err| err.to_string())?;
    let changed = tx
        .execute(
            "UPDATE onboarding SET completed = 1 WHERE id = 1 AND completed = 0",
            [],
        )
        .map_err(|err| err.to_string())?;
    if changed == 0 {
        return Ok(None);
    }
    let task = title
        .map(|title| {
            let title = title.trim();
            if !valid_title(title) {
                return Err("Enter a task name of 1–200 characters.".into());
            }
            queries::create_task(
                &tx,
                &CreateTaskInput {
                    title: title.into(),
                    description: None,
                    project_id: None,
                    estimated_pomodoros: 1,
                },
            )
            .map_err(|err| err.to_string())
        })
        .transpose()?;
    tx.commit().map_err(|err| err.to_string())?;
    Ok(task)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tomatotask::db::migrations::run_migrations;

    #[test]
    fn invalid_title_rolls_back_and_retries_do_not_create_duplicates() {
        let db = DbConnection::new_in_memory().unwrap();
        run_migrations(&db.get_connection().lock().unwrap()).unwrap();
        assert!(pending(&db).unwrap());
        assert!(finish(&db, Some(" ")).is_err());
        assert!(pending(&db).unwrap());
        assert_eq!(
            finish(&db, Some("  Read  ")).unwrap().unwrap().title,
            "Read"
        );
        assert!(!pending(&db).unwrap());
        assert!(finish(&db, Some("Duplicate")).unwrap().is_none());
        let conn = db.get_connection();
        assert_eq!(
            queries::get_all_tasks(&conn.lock().unwrap()).unwrap().len(),
            1
        );
    }

    #[test]
    fn upgrading_a_database_with_tasks_skips_the_welcome() {
        let db = DbConnection::new_in_memory().unwrap();
        let conn = db.get_connection();
        {
            let conn = conn.lock().unwrap();
            run_migrations(&conn).unwrap();
            conn.execute_batch(
                "DROP TABLE onboarding;
                DELETE FROM schema_version WHERE version = 5;
                INSERT INTO tasks (title) VALUES ('Existing task');",
            )
            .unwrap();
            run_migrations(&conn).unwrap();
        }
        assert!(!pending(&db).unwrap());
    }
}
