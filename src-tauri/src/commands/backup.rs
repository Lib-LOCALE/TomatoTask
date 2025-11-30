use crate::db::DbConnection;
use crate::db::models::{Project, Task, PomodoroSession, Settings, SessionType};
use serde::{Deserialize, Serialize};
use tauri::State;
use rusqlite::OptionalExtension;

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
        .prepare("SELECT id, title, description, project_id, estimated_pomodoros, completed_pomodoros, is_completed, created_at, updated_at, completed_at, position FROM tasks")
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
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // 3. Fetch Sessions
    let mut stmt = conn
        .prepare("SELECT id, task_id, started_at, completed_at, duration_minutes, session_type, interrupted FROM sessions")
        .map_err(|e| e.to_string())?;
    
    let sessions = stmt
        .query_map([], |row| {
            let session_type_str: String = row.get(5)?;
            let session_type = SessionType::from_str(&session_type_str).unwrap_or(SessionType::Work);

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

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // 1. Clear existing data
    tx.execute("DELETE FROM sessions", []).map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM tasks", []).map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM projects", []).map_err(|e| e.to_string())?;
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
            "INSERT INTO tasks (id, title, description, project_id, estimated_pomodoros, completed_pomodoros, is_completed, created_at, updated_at, completed_at, position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            (task.id, task.title, task.description, task.project_id, task.estimated_pomodoros, task.completed_pomodoros, task.is_completed, task.created_at, task.updated_at, task.completed_at, task.position),
        ).map_err(|e| e.to_string())?;
    }

    // 4. Insert Sessions
    for session in data.sessions {
        tx.execute(
            "INSERT INTO sessions (id, task_id, started_at, completed_at, duration_minutes, session_type, interrupted) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (session.id, session.task_id, session.started_at, session.completed_at, session.duration_minutes, session.session_type.as_str(), session.interrupted),
        ).map_err(|e| e.to_string())?;
    }

    // 5. Update Settings
    if let Some(settings) = data.settings {
        // Check if settings exist
        let count: i32 = tx.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0)).unwrap_or(0);
        
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
