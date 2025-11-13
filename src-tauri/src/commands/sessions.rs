// Commandes Tauri pour la gestion des sessions Pomodoro
use crate::db::{queries, CreateSessionInput, DbConnection, PomodoroSession, SessionType};
use tauri::State;

/// Crée une nouvelle session Pomodoro
///
/// # Arguments
/// * `task_id` - ID de la tâche associée (optionnel)
/// * `duration_minutes` - Durée de la session en minutes
/// * `session_type` - Type de session (work, short_break, long_break)
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn create_session(
    task_id: Option<i64>,
    duration_minutes: i32,
    session_type: SessionType,
    db: State<DbConnection>,
) -> Result<PomodoroSession, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    let input = CreateSessionInput {
        task_id,
        duration_minutes,
        session_type,
    };

    queries::create_session(&conn, &input).map_err(|e| e.to_string())
}

/// Marque une session comme complétée
///
/// Si la session est associée à une tâche, incrémente le compteur de Pomodoros
///
/// # Arguments
/// * `id` - ID de la session
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn complete_session(id: i64, db: State<DbConnection>) -> Result<PomodoroSession, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    // Complète la session
    let session = queries::complete_session(&conn, id).map_err(|e| e.to_string())?;

    // Si la session est de type "work" et associée à une tâche, incrémente le compteur
    if session.session_type == SessionType::Work {
        if let Some(task_id) = session.task_id {
            queries::increment_completed_pomodoros(&conn, task_id)
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(session)
}

/// Marque une session comme interrompue
///
/// # Arguments
/// * `id` - ID de la session
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn interrupt_session(id: i64, db: State<DbConnection>) -> Result<PomodoroSession, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::interrupt_session(&conn, id).map_err(|e| e.to_string())
}

/// Récupère les sessions d'une plage de dates
///
/// # Arguments
/// * `start_date` - Date de début (format ISO: YYYY-MM-DD)
/// * `end_date` - Date de fin (format ISO: YYYY-MM-DD)
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn get_sessions_by_date_range(
    start_date: String,
    end_date: String,
    db: State<DbConnection>,
) -> Result<Vec<PomodoroSession>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::get_sessions_by_date_range(&conn, &start_date, &end_date)
        .map_err(|e| e.to_string())
}
