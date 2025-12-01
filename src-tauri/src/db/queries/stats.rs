use crate::db::models::{DailyFocusTime, ProjectDistribution};
use rusqlite::{Connection, Result};

/// Récupère le temps de focus quotidien pour les 7 derniers jours
pub fn get_daily_focus_time(conn: &Connection) -> Result<Vec<DailyFocusTime>> {
    let mut stmt = conn.prepare(
        "WITH RECURSIVE dates(date) AS (
            SELECT date('now', '-6 days')
            UNION ALL
            SELECT date(date, '+1 day')
            FROM dates
            WHERE date <= date('now')
        )
        SELECT 
            d.date,
            COALESCE(SUM(s.duration_minutes), 0) as minutes
        FROM dates d
        LEFT JOIN pomodoro_sessions s ON date(s.started_at) = d.date 
            AND s.completed_at IS NOT NULL 
            AND s.session_type = 'work'
        GROUP BY d.date
        ORDER BY d.date ASC"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(DailyFocusTime {
            date: row.get(0)?,
            minutes: row.get(1)?,
        })
    })?;

    rows.collect()
}

/// Récupère la distribution du temps par projet
pub fn get_project_distribution(conn: &Connection) -> Result<Vec<ProjectDistribution>> {
    // D'abord, calculer le temps total pour les pourcentages
    let total_minutes: i32 = conn.query_row(
        "SELECT COALESCE(SUM(duration_minutes), 0)
         FROM pomodoro_sessions
         WHERE completed_at IS NOT NULL 
         AND session_type = 'work'",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    if total_minutes == 0 {
        return Ok(Vec::new());
    }

    let mut stmt = conn.prepare(
        "SELECT 
            COALESCE(p.name, 'No Project') as project_name,
            p.color,
            SUM(s.duration_minutes) as minutes
        FROM pomodoro_sessions s
        LEFT JOIN tasks t ON s.task_id = t.id
        LEFT JOIN projects p ON t.project_id = p.id
        WHERE s.completed_at IS NOT NULL 
        AND s.session_type = 'work'
        GROUP BY p.id
        ORDER BY minutes DESC"
    )?;

    let rows = stmt.query_map([], |row| {
        let minutes: i32 = row.get(2)?;
        let percentage = (minutes as f32 / total_minutes as f32) * 100.0;
        
        Ok(ProjectDistribution {
            project_name: row.get(0)?,
            color: row.get(1)?,
            minutes,
            percentage,
        })
    })?;

    rows.collect()
}

/// Récupère le temps de focus quotidien pour une période donnée
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `days` - Nombre de jours à récupérer
pub fn get_focus_history(conn: &Connection, days: i32) -> Result<Vec<DailyFocusTime>> {
    let mut stmt = conn.prepare(
        "WITH RECURSIVE dates(date) AS (
            SELECT date('now', '-' || ?1 || ' days')
            UNION ALL
            SELECT date(date, '+1 day')
            FROM dates
            WHERE date <= date('now')
        )
        SELECT 
            d.date,
            COALESCE(SUM(s.duration_minutes), 0) as minutes
        FROM dates d
        LEFT JOIN pomodoro_sessions s ON date(s.started_at) = d.date 
            AND s.completed_at IS NOT NULL 
            AND s.session_type = 'work'
        GROUP BY d.date
        ORDER BY d.date ASC"
    )?;

    // Note: SQLite expects string concatenation for interval in some versions, 
    // but parameter binding for calculation is safer.
    // However, the query above uses string concatenation for the interval: '-' || ?1 || ' days'
    // We need to pass `days - 1` because if we want 365 days, we start 364 days ago.
    let days_offset = days - 1;

    let rows = stmt.query_map([days_offset.to_string()], |row| {
        Ok(DailyFocusTime {
            date: row.get(0)?,
            minutes: row.get(1)?,
        })
    })?;

    rows.collect()
}
