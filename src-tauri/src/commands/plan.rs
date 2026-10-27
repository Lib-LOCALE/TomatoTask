use crate::db::{queries, DbConnection, Task};
use chrono::NaiveDate;
use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
struct PlannedTask {
    #[serde(flatten)]
    task: Task,
    #[serde(rename = "carriedOver")]
    carried_over: bool,
}

#[derive(Serialize)]
pub struct DayPlan {
    date: String,
    tasks: Vec<PlannedTask>,
    planned_pomodoros: i64,
    completed_pomodoros: i64,
}

fn validate_date(date: &str) -> Result<NaiveDate, String> {
    let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|e| e.to_string())?;
    if parsed.format("%Y-%m-%d").to_string() != date {
        return Err("Date attendue au format AAAA-MM-JJ".into());
    }
    Ok(parsed)
}

fn set_plan(conn: &Connection, id: i64, date: Option<&str>) -> Result<Task, String> {
    if let Some(date) = date {
        validate_date(date)?;
    }
    let changed = conn.execute("UPDATE tasks SET planned_for = ?1, plan_position = CASE WHEN ?1 IS NULL THEN NULL ELSE (SELECT COALESCE(MAX(plan_position), -1) + 1 FROM tasks WHERE planned_for = ?1 AND id != ?2) END, updated_at = datetime('now') WHERE id = ?2", (date, id)).map_err(|e| e.to_string())?;
    if changed == 0 {
        return Err("Tâche introuvable".into());
    }
    queries::get_task_by_id(conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn plan_task(id: i64, date: String, db: State<DbConnection>) -> Result<Task, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;
    set_plan(&conn, id, Some(&date))
}
#[tauri::command]
pub fn unplan_task(id: i64, db: State<DbConnection>) -> Result<Task, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;
    set_plan(&conn, id, None)
}

fn reorder(conn: &mut Connection, date: &str, ids: &[i64], today: NaiveDate) -> Result<(), String> {
    let parsed = validate_date(date)?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let mut expected = Vec::new();
    {
        let mut stmt = tx.prepare("SELECT id FROM tasks WHERE planned_for = ?1 OR (?2 AND planned_for < ?1 AND is_completed = 0)").map_err(|e| e.to_string())?;
        for id in stmt
            .query_map((date, parsed == today), |r| r.get::<_, i64>(0))
            .map_err(|e| e.to_string())?
        {
            expected.push(id.map_err(|e| e.to_string())?);
        }
    }
    let mut supplied = ids.to_vec();
    expected.sort_unstable();
    supplied.sort_unstable();
    if supplied != expected {
        return Err("La liste doit contenir toutes les tâches du plan une seule fois".into());
    }
    for (position, id) in ids.iter().enumerate() {
        tx.execute(
            "UPDATE tasks SET plan_position = ?1 WHERE id = ?2",
            (position as i64, id),
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}
#[tauri::command]
pub fn reorder_plan(
    date: String,
    task_ids: Vec<i64>,
    db: State<DbConnection>,
) -> Result<(), String> {
    let conn = db.get_connection();
    let mut conn = conn.lock().map_err(|e| e.to_string())?;
    reorder(
        &mut conn,
        &date,
        &task_ids,
        chrono::Local::now().date_naive(),
    )
}

fn day_plan(conn: &Connection, date: &str, today: NaiveDate) -> Result<DayPlan, String> {
    let parsed = validate_date(date)?;
    let mut stmt = conn.prepare("SELECT id, planned_for FROM tasks WHERE planned_for = ?1 OR (?2 AND planned_for < ?1 AND is_completed = 0) ORDER BY plan_position IS NULL, plan_position, planned_for, id").map_err(|e| e.to_string())?;
    let entries = stmt
        .query_map((date, parsed == today), |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| e.to_string())?;
    let mut tasks = Vec::new();
    for entry in entries {
        let (id, planned_for) = entry.map_err(|e| e.to_string())?;
        tasks.push(PlannedTask {
            task: queries::get_task_by_id(conn, id).map_err(|e| e.to_string())?,
            carried_over: planned_for != date,
        });
    }
    let planned_pomodoros = tasks
        .iter()
        .map(|t| i64::from(t.task.estimated_pomodoros))
        .sum();
    let completed_pomodoros = conn.query_row("SELECT COUNT(*) FROM pomodoro_sessions WHERE session_type = 'work' AND interrupted = 0 AND completed_at IS NOT NULL AND date(completed_at, 'localtime') = ?1", [date], |row| row.get(0)).optional().map_err(|e| e.to_string())?.unwrap_or(0);
    Ok(DayPlan {
        date: date.into(),
        tasks,
        planned_pomodoros,
        completed_pomodoros,
    })
}
#[tauri::command]
pub fn get_day_plan(date: String, db: State<DbConnection>) -> Result<DayPlan, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;
    day_plan(&conn, &date, chrono::Local::now().date_naive())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn planification_report_et_ordre() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::db::migrations::run_migrations(&conn).unwrap();
        crate::db::migrations::run_migrations(&conn).unwrap();
        conn.execute_batch("INSERT INTO tasks(title, estimated_pomodoros) VALUES ('a', 2), ('b', 3), ('c', 1); INSERT INTO pomodoro_sessions(started_at, completed_at, duration_minutes, session_type) VALUES ('2026-01-02 09:00:00', '2026-01-02 09:25:00', 25, 'work');").unwrap();
        set_plan(&conn, 1, Some("2026-01-01")).unwrap();
        set_plan(&conn, 2, Some("2026-01-02")).unwrap();
        set_plan(&conn, 3, Some("2026-01-01")).unwrap();
        conn.execute("UPDATE tasks SET is_completed = 1 WHERE id = 3", [])
            .unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 1, 2).unwrap();
        let plan = day_plan(&conn, "2026-01-02", today).unwrap();
        assert_eq!((plan.tasks.len(), plan.planned_pomodoros), (2, 5));
        let expected: i64 = conn.query_row("SELECT COUNT(*) FROM pomodoro_sessions WHERE date(completed_at, 'localtime') = '2026-01-02'", [], |r| r.get(0)).unwrap();
        assert_eq!(plan.completed_pomodoros, expected);
        assert!(plan.tasks.iter().any(|t| t.task.id == 1 && t.carried_over));
        assert!(day_plan(&conn, "2026-01-02", today.succ_opt().unwrap())
            .unwrap()
            .tasks
            .iter()
            .all(|t| !t.carried_over));
        assert!(reorder(&mut conn, "2026-01-02", &[2, 2], today).is_err());
        reorder(&mut conn, "2026-01-02", &[2, 1], today).unwrap();
        assert_eq!(
            day_plan(&conn, "2026-01-02", today).unwrap().tasks[0]
                .task
                .id,
            2
        );
        assert!(set_plan(&conn, 1, Some("2026-02-30")).is_err());
        assert!(set_plan(&conn, 99, None).is_err());
        assert_eq!(set_plan(&conn, 1, None).unwrap().planned_for, None);
    }
}
