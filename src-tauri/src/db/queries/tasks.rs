// Requêtes SQL pour la gestion des tâches
use crate::db::models::{CreateTaskInput, Task, UpdateTaskInput};
use rusqlite::{Connection, Result, params};

/// Récupère toutes les tâches
///
/// # Arguments
/// * `conn` - Connexion à la base de données
///
/// # Retourne
/// Un vecteur de toutes les tâches triées par date de création (plus récent en premier)
pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, project_id, estimated_pomodoros,
                completed_pomodoros, is_completed, created_at, updated_at, completed_at
         FROM tasks
         ORDER BY created_at DESC",
    )?;

    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            project_id: row.get(3)?,
            estimated_pomodoros: row.get(4)?,
            completed_pomodoros: row.get(5)?,
            is_completed: row.get::<_, i32>(6)? != 0, // Convertit SQLite integer en bool
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            completed_at: row.get(9)?,
        })
    })?;

    tasks.collect()
}

/// Récupère une tâche spécifique par son ID
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `task_id` - ID de la tâche à récupérer
pub fn get_task_by_id(conn: &Connection, task_id: i64) -> Result<Task> {
    conn.query_row(
        "SELECT id, title, description, project_id, estimated_pomodoros,
                completed_pomodoros, is_completed, created_at, updated_at, completed_at
         FROM tasks
         WHERE id = ?1",
        [task_id],
        |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                project_id: row.get(3)?,
                estimated_pomodoros: row.get(4)?,
                completed_pomodoros: row.get(5)?,
                is_completed: row.get::<_, i32>(6)? != 0,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                completed_at: row.get(9)?,
            })
        },
    )
}

/// Crée une nouvelle tâche
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `input` - Données de la tâche à créer
pub fn create_task(conn: &Connection, input: &CreateTaskInput) -> Result<Task> {
    conn.execute(
        "INSERT INTO tasks (title, description, project_id, estimated_pomodoros)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            &input.title,
            &input.description,
            &input.project_id,
            &input.estimated_pomodoros,
        ],
    )?;

    let task_id = conn.last_insert_rowid();
    get_task_by_id(conn, task_id)
}

/// Met à jour une tâche existante
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `task_id` - ID de la tâche à mettre à jour
/// * `input` - Nouvelles données de la tâche
pub fn update_task(
    conn: &Connection,
    task_id: i64,
    input: &UpdateTaskInput,
) -> Result<Task> {
    conn.execute(
        "UPDATE tasks
         SET title = ?1, description = ?2, project_id = ?3,
             estimated_pomodoros = ?4, updated_at = datetime('now')
         WHERE id = ?5",
        params![
            &input.title,
            &input.description,
            &input.project_id,
            &input.estimated_pomodoros,
            task_id,
        ],
    )?;

    get_task_by_id(conn, task_id)
}

/// Supprime une tâche
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `task_id` - ID de la tâche à supprimer
pub fn delete_task(conn: &Connection, task_id: i64) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [task_id])?;
    Ok(())
}

/// Bascule le statut de complétion d'une tâche
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `task_id` - ID de la tâche à basculer
pub fn toggle_task_completion(conn: &Connection, task_id: i64) -> Result<Task> {
    // Récupère l'état actuel
    let task = get_task_by_id(conn, task_id)?;
    let new_status = !task.is_completed;

    // Met à jour le statut et la date de complétion
    conn.execute(
        "UPDATE tasks
         SET is_completed = ?1,
             completed_at = CASE WHEN ?1 = 1 THEN datetime('now') ELSE NULL END,
             updated_at = datetime('now')
         WHERE id = ?2",
        params![if new_status { 1 } else { 0 }, task_id],
    )?;

    get_task_by_id(conn, task_id)
}

/// Incrémente le compteur de Pomodoros complétés pour une tâche
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `task_id` - ID de la tâche
pub fn increment_completed_pomodoros(conn: &Connection, task_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE tasks
         SET completed_pomodoros = completed_pomodoros + 1,
             updated_at = datetime('now')
         WHERE id = ?1",
        [task_id],
    )?;
    Ok(())
}

/// Récupère les tâches par projet
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `project_id` - ID du projet
pub fn get_tasks_by_project(conn: &Connection, project_id: i64) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, project_id, estimated_pomodoros,
                completed_pomodoros, is_completed, created_at, updated_at, completed_at
         FROM tasks
         WHERE project_id = ?1
         ORDER BY created_at DESC",
    )?;

    let tasks = stmt.query_map([project_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            project_id: row.get(3)?,
            estimated_pomodoros: row.get(4)?,
            completed_pomodoros: row.get(5)?,
            is_completed: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            completed_at: row.get(9)?,
        })
    })?;

    tasks.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_and_get_task() {
        let conn = setup_test_db();
        let input = CreateTaskInput {
            title: "Test Task".to_string(),
            description: Some("Test description".to_string()),
            project_id: None,
            estimated_pomodoros: 3,
        };

        let task = create_task(&conn, &input).unwrap();
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.estimated_pomodoros, 3);

        let retrieved = get_task_by_id(&conn, task.id).unwrap();
        assert_eq!(retrieved.title, task.title);
    }

    #[test]
    fn test_toggle_completion() {
        let conn = setup_test_db();
        let input = CreateTaskInput {
            title: "Toggle Test".to_string(),
            description: None,
            project_id: None,
            estimated_pomodoros: 1,
        };

        let task = create_task(&conn, &input).unwrap();
        assert!(!task.is_completed);

        let toggled = toggle_task_completion(&conn, task.id).unwrap();
        assert!(toggled.is_completed);
        assert!(toggled.completed_at.is_some());
    }
}
