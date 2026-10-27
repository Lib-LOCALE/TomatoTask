// Commandes Tauri pour les résumés et analytics
use crate::db::{queries, DailySummary, DbConnection};
use tauri::State;

/// Récupère le résumé quotidien pour une date donnée
///
/// # Arguments
/// * `date` - Date cible (format ISO: YYYY-MM-DD)
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn get_daily_summary(date: String, db: State<DbConnection>) -> Result<DailySummary, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    daily_summary(&conn, date).map_err(|e| e.to_string())
}

fn daily_summary(conn: &rusqlite::Connection, date: String) -> rusqlite::Result<DailySummary> {
    // Compte les tâches complétées pour cette date
    let completed_tasks_count: i32 = conn.query_row(
        "SELECT COUNT(*)
             FROM tasks
             WHERE DATE(completed_at) = ?1",
        [&date],
        |row| row.get(0),
    )?;

    // Compte les Pomodoros complétés (sessions de travail non interrompues)
    let completed_pomodoros_count = queries::count_completed_sessions_by_date(conn, &date)?;

    // Calcule le total de minutes de focus
    let total_focus_minutes = queries::calculate_focus_minutes_by_date(conn, &date)?;

    Ok(DailySummary {
        date,
        completed_tasks_count,
        completed_pomodoros_count,
        total_focus_minutes,
    })
}

/// Récupère les résumés quotidiens pour une plage de dates (hebdomadaire)
///
/// # Arguments
/// * `start_date` - Date de début (format ISO: YYYY-MM-DD)
/// * `end_date` - Date de fin (format ISO: YYYY-MM-DD)
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn get_weekly_summary(
    start_date: String,
    end_date: String,
    db: State<DbConnection>,
) -> Result<Vec<DailySummary>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    // Génère une liste de dates dans la plage
    let mut summaries = Vec::new();

    // Pour simplifier, on utilise SQLite pour générer les dates
    let mut stmt = conn
        .prepare(
            "WITH RECURSIVE dates(date) AS (
                SELECT DATE(?1)
                UNION ALL
                SELECT DATE(date, '+1 day')
                FROM dates
                WHERE date < DATE(?2)
            )
            SELECT date FROM dates",
        )
        .map_err(|e| e.to_string())?;

    let dates: Vec<String> = stmt
        .query_map([&start_date, &end_date], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;

    // Pour chaque date, calcule le résumé
    for date in dates {
        summaries.push(daily_summary(&conn, date).map_err(|e| e.to_string())?);
    }

    Ok(summaries)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resume_ignore_pauses_et_interruptions() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::migrations::run_migrations(&conn).unwrap();
        conn.execute_batch("INSERT INTO tasks (title, is_completed, completed_at) VALUES ('fait', 1, '2026-01-02 10:00:00'); INSERT INTO pomodoro_sessions (started_at, completed_at, duration_minutes, session_type, interrupted) VALUES ('2026-01-02 10:00:00', '2026-01-02 10:25:00', 25, 'work', 0), ('2026-01-02 11:00:00', '2026-01-02 11:25:00', 25, 'work', 1)").unwrap();
        let summary = daily_summary(&conn, "2026-01-02".into()).unwrap();
        assert_eq!(
            (
                summary.completed_tasks_count,
                summary.completed_pomodoros_count,
                summary.total_focus_minutes
            ),
            (1, 1, 25)
        );
    }
}
