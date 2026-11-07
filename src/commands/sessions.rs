// Opérations pour la gestion des sessions Pomodoro
use crate::db::{CreateSessionInput, DbConnection, PomodoroSession, SessionType, queries};

/// Crée une nouvelle session Pomodoro
///
/// # Arguments
/// * `task_id` - ID de la tâche associée (optionnel)
/// * `duration_minutes` - Durée de la session en minutes
/// * `session_type` - Type de session (work, short_break, long_break)
/// * `db` - État partagé contenant la connexion à la base de données
pub fn create_session(
    task_id: Option<i64>,
    duration_minutes: i32,
    session_type: SessionType,
    db: &DbConnection,
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
pub fn complete_session(id: i64, db: &DbConnection) -> Result<PomodoroSession, String> {
    let conn = db.get_connection();
    let mut conn = conn.lock().map_err(|e| e.to_string())?;
    let transaction = conn.transaction().map_err(|e| e.to_string())?;
    let session = queries::get_session_by_id(&transaction, id).map_err(|e| e.to_string())?;
    if session.completed_at.is_some() {
        return Ok(session);
    }
    let session = queries::complete_session(&transaction, id).map_err(|e| e.to_string())?;
    if session.session_type == SessionType::Work
        && let Some(task_id) = session.task_id
    {
        queries::increment_completed_pomodoros(&transaction, task_id).map_err(|e| e.to_string())?;
    }
    transaction.commit().map_err(|e| e.to_string())?;
    Ok(session)
}

/// Marque une session comme interrompue
///
/// # Arguments
/// * `id` - ID de la session
/// * `db` - État partagé contenant la connexion à la base de données
pub fn interrupt_session(id: i64, db: &DbConnection) -> Result<PomodoroSession, String> {
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
pub fn get_sessions_by_date_range(
    start_date: String,
    end_date: String,
    db: &DbConnection,
) -> Result<Vec<PomodoroSession>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::get_sessions_by_date_range(&conn, &start_date, &end_date).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{create_task, get_task};
    use crate::db::{CreateTaskInput, migrations};

    #[test]
    fn retrying_completion_does_not_count_a_second_pomodoro() {
        let db = DbConnection::new_in_memory().unwrap();
        migrations::run_migrations(&db.get_connection().lock().unwrap()).unwrap();
        let task = create_task(
            CreateTaskInput {
                title: "Focus".into(),
                description: None,
                project_id: None,
                estimated_pomodoros: 2,
            },
            &db,
        )
        .unwrap();
        let session = create_session(Some(task.id), 25, SessionType::Work, &db).unwrap();
        complete_session(session.id, &db).unwrap();
        complete_session(session.id, &db).unwrap();
        assert_eq!(get_task(task.id, &db).unwrap().completed_pomodoros, 1);
    }
}
