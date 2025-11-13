// Requêtes SQL pour la gestion des sessions Pomodoro
use crate::db::models::{CreateSessionInput, PomodoroSession, SessionType};
use rusqlite::{Connection, Result, params};

/// Crée une nouvelle session Pomodoro
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `input` - Données de la session à créer
pub fn create_session(conn: &Connection, input: &CreateSessionInput) -> Result<PomodoroSession> {
    conn.execute(
        "INSERT INTO pomodoro_sessions (task_id, started_at, duration_minutes, session_type)
         VALUES (?1, datetime('now'), ?2, ?3)",
        params![
            &input.task_id,
            &input.duration_minutes,
            input.session_type.as_str(),
        ],
    )?;

    let session_id = conn.last_insert_rowid();
    get_session_by_id(conn, session_id)
}

/// Marque une session comme complétée
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `session_id` - ID de la session à compléter
pub fn complete_session(conn: &Connection, session_id: i64) -> Result<PomodoroSession> {
    conn.execute(
        "UPDATE pomodoro_sessions
         SET completed_at = datetime('now'), interrupted = 0
         WHERE id = ?1",
        [session_id],
    )?;

    get_session_by_id(conn, session_id)
}

/// Marque une session comme interrompue
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `session_id` - ID de la session à interrompre
pub fn interrupt_session(conn: &Connection, session_id: i64) -> Result<PomodoroSession> {
    conn.execute(
        "UPDATE pomodoro_sessions
         SET completed_at = datetime('now'), interrupted = 1
         WHERE id = ?1",
        [session_id],
    )?;

    get_session_by_id(conn, session_id)
}

/// Récupère une session par son ID
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `session_id` - ID de la session
fn get_session_by_id(conn: &Connection, session_id: i64) -> Result<PomodoroSession> {
    conn.query_row(
        "SELECT id, task_id, started_at, completed_at, duration_minutes, session_type, interrupted
         FROM pomodoro_sessions
         WHERE id = ?1",
        [session_id],
        |row| {
            let session_type_str: String = row.get(5)?;
            let session_type = SessionType::from_str(&session_type_str)
                .map_err(|e| rusqlite::Error::InvalidQuery)?;

            Ok(PomodoroSession {
                id: row.get(0)?,
                task_id: row.get(1)?,
                started_at: row.get(2)?,
                completed_at: row.get(3)?,
                duration_minutes: row.get(4)?,
                session_type,
                interrupted: row.get::<_, i32>(6)? != 0,
            })
        },
    )
}

/// Récupère toutes les sessions d'une plage de dates
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `start_date` - Date de début (format ISO: YYYY-MM-DD)
/// * `end_date` - Date de fin (format ISO: YYYY-MM-DD)
pub fn get_sessions_by_date_range(
    conn: &Connection,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<PomodoroSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, started_at, completed_at, duration_minutes, session_type, interrupted
         FROM pomodoro_sessions
         WHERE DATE(started_at) BETWEEN ?1 AND ?2
         ORDER BY started_at DESC",
    )?;

    let sessions = stmt.query_map(params![start_date, end_date], |row| {
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
            interrupted: row.get::<_, i32>(6)? != 0,
        })
    })?;

    sessions.collect()
}

/// Compte les sessions complétées pour une date donnée
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `date` - Date cible (format ISO: YYYY-MM-DD)
pub fn count_completed_sessions_by_date(conn: &Connection, date: &str) -> Result<i32> {
    conn.query_row(
        "SELECT COUNT(*)
         FROM pomodoro_sessions
         WHERE DATE(started_at) = ?1
           AND completed_at IS NOT NULL
           AND interrupted = 0
           AND session_type = 'work'",
        [date],
        |row| row.get(0),
    )
}

/// Calcule le total de minutes de focus pour une date
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `date` - Date cible (format ISO: YYYY-MM-DD)
pub fn calculate_focus_minutes_by_date(conn: &Connection, date: &str) -> Result<i32> {
    let result: Option<i32> = conn.query_row(
        "SELECT SUM(duration_minutes)
         FROM pomodoro_sessions
         WHERE DATE(started_at) = ?1
           AND completed_at IS NOT NULL
           AND interrupted = 0
           AND session_type = 'work'",
        [date],
        |row| row.get(0),
    )?;

    Ok(result.unwrap_or(0))
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
    fn test_create_and_complete_session() {
        let conn = setup_test_db();
        let input = CreateSessionInput {
            task_id: None,
            duration_minutes: 25,
            session_type: SessionType::Work,
        };

        let session = create_session(&conn, &input).unwrap();
        assert_eq!(session.duration_minutes, 25);
        assert_eq!(session.session_type, SessionType::Work);
        assert!(session.completed_at.is_none());

        let completed = complete_session(&conn, session.id).unwrap();
        assert!(completed.completed_at.is_some());
        assert!(!completed.interrupted);
    }

    #[test]
    fn test_interrupt_session() {
        let conn = setup_test_db();
        let input = CreateSessionInput {
            task_id: None,
            duration_minutes: 25,
            session_type: SessionType::Work,
        };

        let session = create_session(&conn, &input).unwrap();
        let interrupted = interrupt_session(&conn, session.id).unwrap();

        assert!(interrupted.completed_at.is_some());
        assert!(interrupted.interrupted);
    }
}
