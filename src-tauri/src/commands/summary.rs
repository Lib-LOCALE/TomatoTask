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

    // Compte les tâches complétées pour cette date
    let completed_tasks_count: i32 = conn
        .query_row(
            "SELECT COUNT(*)
             FROM tasks
             WHERE DATE(completed_at) = ?1",
            [&date],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Compte les Pomodoros complétés (sessions de travail non interrompues)
    let completed_pomodoros_count =
        queries::count_completed_sessions_by_date(&conn, &date).map_err(|e| e.to_string())?;

    // Calcule le total de minutes de focus
    let total_focus_minutes =
        queries::calculate_focus_minutes_by_date(&conn, &date).map_err(|e| e.to_string())?;

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
        let completed_tasks_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM tasks WHERE DATE(completed_at) = ?1",
                [&date],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        let completed_pomodoros_count =
            queries::count_completed_sessions_by_date(&conn, &date).map_err(|e| e.to_string())?;

        let total_focus_minutes =
            queries::calculate_focus_minutes_by_date(&conn, &date).map_err(|e| e.to_string())?;

        summaries.push(DailySummary {
            date,
            completed_tasks_count,
            completed_pomodoros_count,
            total_focus_minutes,
        });
    }

    Ok(summaries)
}
