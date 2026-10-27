use crate::db::models::{PomodoroSession, Project, SessionType, Settings, Task};
use crate::db::DbConnection;
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupData {
    pub version: i32,
    pub timestamp: String,
    pub projects: Vec<Project>,
    pub tasks: Vec<Task>,
    pub sessions: Vec<PomodoroSession>,
    pub settings: Option<Settings>,
}

#[tauri::command]
pub fn export_data(db: State<DbConnection>) -> Result<BackupData, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    export_backup(&conn)
}

fn export_backup(conn: &rusqlite::Connection) -> Result<BackupData, String> {
    // 1. Fetch Projects
    let mut stmt = conn
        .prepare("SELECT id, name, color, created_at, updated_at, position FROM projects")
        .map_err(|e| e.to_string())?;

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                position: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // 2. Fetch Tasks
    let mut stmt = conn
        .prepare("SELECT id, title, description, project_id, estimated_pomodoros, completed_pomodoros, is_completed, created_at, updated_at, completed_at, position, planned_for, plan_position FROM tasks")
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                project_id: row.get(3)?,
                estimated_pomodoros: row.get(4)?,
                completed_pomodoros: row.get(5)?,
                is_completed: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                completed_at: row.get(9)?,
                position: row.get(10)?,
                planned_for: row.get(11)?,
                plan_position: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // 3. Fetch Sessions
    let mut stmt = conn
        .prepare("SELECT id, task_id, started_at, completed_at, duration_minutes, session_type, interrupted FROM pomodoro_sessions")
        .map_err(|e| e.to_string())?;

    let sessions = stmt
        .query_map([], |row| {
            let session_type_str: String = row.get(5)?;
            let session_type = SessionType::from_str(&session_type_str)
                .map_err(|_| rusqlite::Error::InvalidQuery)?;

            Ok(PomodoroSession {
                id: row.get(0)?,
                task_id: row.get(1)?,
                started_at: row.get(2)?,
                completed_at: row.get(3)?,
                duration_minutes: row.get(4)?,
                session_type,
                interrupted: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // 4. Fetch Settings
    let mut stmt = conn
        .prepare("SELECT work_duration, short_break_duration, long_break_duration, pomodoros_until_long_break, language, theme, notification_sound, auto_start_breaks, auto_start_pomodoros FROM settings LIMIT 1")
        .map_err(|e| e.to_string())?;

    let settings = stmt
        .query_row([], |row| {
            Ok(Settings {
                work_duration: row.get(0)?,
                short_break_duration: row.get(1)?,
                long_break_duration: row.get(2)?,
                pomodoros_until_long_break: row.get(3)?,
                language: row.get(4)?,
                theme: row.get(5)?,
                notification_sound: row.get(6)?,
                auto_start_breaks: row.get(7)?,
                auto_start_pomodoros: row.get(8)?,
            })
        })
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(BackupData {
        version: 1,
        timestamp: chrono::Utc::now().to_rfc3339(),
        projects,
        tasks,
        sessions,
        settings,
    })
}

#[tauri::command]
pub fn import_data(db: State<DbConnection>, data: BackupData) -> Result<(), String> {
    let conn = db.get_connection();
    let mut conn = conn.lock().map_err(|e| e.to_string())?;

    import_backup(&mut conn, data)
}

fn import_backup(conn: &mut rusqlite::Connection, data: BackupData) -> Result<(), String> {
    if data.version != 1
        || data
            .projects
            .iter()
            .any(|p| p.id <= 0 || p.name.trim().is_empty())
        || data.tasks.iter().any(|t| {
            t.id <= 0
                || t.title.trim().is_empty()
                || t.estimated_pomodoros < 0
                || t.completed_pomodoros < 0
        })
        || data
            .sessions
            .iter()
            .any(|s| s.id <= 0 || s.duration_minutes <= 0)
    {
        return Err("Sauvegarde invalide ou version non prise en charge".into());
    }
    let timestamps = data
        .projects
        .iter()
        .flat_map(|p| [Some(&p.created_at), Some(&p.updated_at)])
        .chain(data.tasks.iter().flat_map(|t| {
            [
                Some(&t.created_at),
                Some(&t.updated_at),
                t.completed_at.as_ref(),
            ]
        }))
        .chain(
            data.sessions
                .iter()
                .flat_map(|s| [Some(&s.started_at), s.completed_at.as_ref()]),
        );
    for timestamp in timestamps.flatten() {
        let valid: bool = conn
            .query_row("SELECT datetime(?1) IS NOT NULL", [timestamp], |row| {
                row.get(0)
            })
            .map_err(|e| e.to_string())?;
        if !valid {
            return Err("Date invalide dans la sauvegarde".into());
        }
    }
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // 1. Clear existing data
    tx.execute("DELETE FROM pomodoro_sessions", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM tasks", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM projects", [])
        .map_err(|e| e.to_string())?;
    // We don't delete settings, we just update them if present

    // 2. Insert Projects
    for project in data.projects {
        tx.execute(
            "INSERT INTO projects (id, name, color, created_at, updated_at, position) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (project.id, project.name, project.color, project.created_at, project.updated_at, project.position),
        ).map_err(|e| e.to_string())?;
    }

    // 3. Insert Tasks
    for task in data.tasks {
        tx.execute(
            "INSERT INTO tasks (id, title, description, project_id, estimated_pomodoros, completed_pomodoros, is_completed, created_at, updated_at, completed_at, position, planned_for, plan_position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (task.id, task.title, task.description, task.project_id, task.estimated_pomodoros, task.completed_pomodoros, task.is_completed, task.created_at, task.updated_at, task.completed_at, task.position, task.planned_for, task.plan_position),
        ).map_err(|e| e.to_string())?;
    }

    // 4. Insert Sessions
    for session in data.sessions {
        tx.execute(
            "INSERT INTO pomodoro_sessions (id, task_id, started_at, completed_at, duration_minutes, session_type, interrupted) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (session.id, session.task_id, session.started_at, session.completed_at, session.duration_minutes, session.session_type.as_str(), session.interrupted),
        ).map_err(|e| e.to_string())?;
    }

    // 5. Update Settings
    if let Some(settings) = data.settings {
        // Check if settings exist
        let count: i32 = tx
            .query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))
            .map_err(|e| e.to_string())?;

        if count > 0 {
            tx.execute(
                "UPDATE settings SET work_duration = ?1, short_break_duration = ?2, long_break_duration = ?3, pomodoros_until_long_break = ?4, language = ?5, theme = ?6, notification_sound = ?7, auto_start_breaks = ?8, auto_start_pomodoros = ?9",
                (settings.work_duration, settings.short_break_duration, settings.long_break_duration, settings.pomodoros_until_long_break, settings.language, settings.theme, settings.notification_sound, settings.auto_start_breaks, settings.auto_start_pomodoros),
            ).map_err(|e| e.to_string())?;
        } else {
            tx.execute(
                "INSERT INTO settings (work_duration, short_break_duration, long_break_duration, pomodoros_until_long_break, language, theme, notification_sound, auto_start_breaks, auto_start_pomodoros) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (settings.work_duration, settings.short_break_duration, settings.long_break_duration, settings.pomodoros_until_long_break, settings.language, settings.theme, settings.notification_sound, settings.auto_start_breaks, settings.auto_start_pomodoros),
            ).map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;

    #[test]
    fn sauvegarde_restaure_sessions_et_projets() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys = ON").unwrap();
        run_migrations(&conn).unwrap();
        conn.execute_batch("INSERT INTO projects (name) VALUES ('projet'); INSERT INTO tasks (title, project_id, planned_for, plan_position) VALUES ('tache', 1, '2026-01-01', 3); INSERT INTO pomodoro_sessions (task_id, started_at, completed_at, duration_minutes, session_type) VALUES (1, '2026-01-01 12:00:00', '2026-01-01 12:25:00', 25, 'work')").unwrap();
        let backup = export_backup(&conn).unwrap();
        import_backup(&mut conn, backup).unwrap();
        let restored = export_backup(&conn).unwrap();
        assert_eq!(restored.projects.len(), 1);
        assert_eq!(restored.tasks[0].project_id, Some(1));
        assert_eq!(restored.tasks[0].planned_for.as_deref(), Some("2026-01-01"));
        assert_eq!(restored.tasks[0].plan_position, Some(3));
        assert_eq!(restored.sessions[0].task_id, Some(1));
    }

    #[test]
    fn import_invalide_ne_supprime_pas_les_donnees() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys = ON").unwrap();
        run_migrations(&conn).unwrap();
        conn.execute("INSERT INTO projects (name) VALUES ('existant')", [])
            .unwrap();
        let backup = BackupData {
            version: 1,
            timestamp: String::new(),
            projects: vec![],
            tasks: vec![],
            sessions: vec![PomodoroSession {
                id: 1,
                task_id: Some(999),
                started_at: "2026-01-01 12:00:00".into(),
                completed_at: None,
                duration_minutes: 25,
                session_type: SessionType::Work,
                interrupted: false,
            }],
            settings: None,
        };
        assert!(import_backup(&mut conn, backup).is_err());
        assert_eq!(
            conn.query_row("SELECT name FROM projects", [], |row| row
                .get::<_, String>(0))
                .unwrap(),
            "existant"
        );
    }
}
