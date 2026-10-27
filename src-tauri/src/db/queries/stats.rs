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
            WHERE date < date('now')
        )
        SELECT 
            d.date,
            COALESCE(SUM(s.duration_minutes), 0) as minutes
        FROM dates d
        LEFT JOIN pomodoro_sessions s ON date(s.started_at) = d.date 
            AND s.completed_at IS NOT NULL 
            AND s.session_type = 'work'
            AND s.interrupted = 0
        GROUP BY d.date
        ORDER BY d.date ASC",
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
         AND session_type = 'work'
         AND interrupted = 0",
        [],
        |row| row.get(0),
    )?;

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
            AND s.interrupted = 0
        GROUP BY p.id
        ORDER BY minutes DESC",
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
    if days <= 0 {
        return Ok(Vec::new());
    }
    let mut stmt = conn.prepare(
        "WITH RECURSIVE dates(date) AS (
            SELECT date('now', '-' || ?1 || ' days')
            UNION ALL
            SELECT date(date, '+1 day')
            FROM dates
            WHERE date < date('now')
        )
        SELECT 
            d.date,
            COALESCE(SUM(s.duration_minutes), 0) as minutes
        FROM dates d
        LEFT JOIN pomodoro_sessions s ON date(s.started_at) = d.date 
            AND s.completed_at IS NOT NULL 
            AND s.session_type = 'work'
            AND s.interrupted = 0
        GROUP BY d.date
        ORDER BY d.date ASC",
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

#[derive(Debug, serde::Serialize)]
pub struct FocusStreak {
    pub current_days: u32,
    pub best_days: u32,
    pub total_pomodoros: u32,
    pub today_pomodoros: u32,
}

pub fn get_focus_streak(conn: &Connection, today: chrono::NaiveDate) -> Result<FocusStreak> {
    let mut stmt = conn.prepare(
        "SELECT date(completed_at, 'localtime'), COUNT(*) FROM pomodoro_sessions
         WHERE completed_at IS NOT NULL AND interrupted = 0 AND session_type = 'work'
         GROUP BY 1 ORDER BY 1",
    )?;
    let days = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
    })?;
    let mut result = FocusStreak {
        current_days: 0,
        best_days: 0,
        total_pomodoros: 0,
        today_pomodoros: 0,
    };
    let mut previous = None;
    let mut run = 0;
    for entry in days {
        let (day, count) = entry?;
        let day = chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d")
            .map_err(|_| rusqlite::Error::InvalidQuery)?;
        result.total_pomodoros = result
            .total_pomodoros
            .checked_add(count)
            .ok_or(rusqlite::Error::InvalidQuery)?;
        run = if previous == day.pred_opt() {
            run + 1
        } else {
            1
        };
        result.best_days = result.best_days.max(run);
        if day == today {
            result.today_pomodoros = count;
        }
        if day == today || Some(day) == today.pred_opt() {
            result.current_days = run;
        }
        previous = Some(day);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;

    #[test]
    fn statistiques_excluent_interruptions_et_ne_depassent_pas_aujourdhui() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn.execute_batch("INSERT INTO pomodoro_sessions (started_at, completed_at, duration_minutes, session_type, interrupted) VALUES (datetime('now'), datetime('now'), 25, 'work', 0), (datetime('now'), datetime('now'), 40, 'work', 1), (datetime('now'), datetime('now'), 5, 'short_break', 0)").unwrap();
        let daily = get_daily_focus_time(&conn).unwrap();
        assert_eq!(daily.len(), 7);
        assert_eq!(daily.last().unwrap().minutes, 25);
        let history = get_focus_history(&conn, 2).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history.last().unwrap().minutes, 25);
        assert_eq!(get_project_distribution(&conn).unwrap()[0].minutes, 25);
        assert!(get_focus_history(&conn, 0).unwrap().is_empty());
    }

    #[test]
    fn serie_compte_les_jours_locaux_et_ignore_les_interruptions() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn.execute_batch("INSERT INTO pomodoro_sessions (started_at, completed_at, duration_minutes, session_type, interrupted) VALUES (datetime('now', '-2 days'), datetime('now', '-2 days'), 25, 'work', 0), (datetime('now', '-1 day'), datetime('now', '-1 day'), 25, 'work', 0), (datetime('now'), datetime('now'), 25, 'work', 0), (datetime('now'), datetime('now'), 25, 'work', 1)").unwrap();
        let today = chrono::Local::now().date_naive();
        let result = get_focus_streak(&conn, today).unwrap();
        assert_eq!(
            (
                result.current_days,
                result.best_days,
                result.total_pomodoros,
                result.today_pomodoros
            ),
            (3, 3, 3, 1)
        );
        let tomorrow = today.succ_opt().unwrap();
        assert_eq!(get_focus_streak(&conn, tomorrow).unwrap().current_days, 3);
        assert_eq!(
            get_focus_streak(&conn, tomorrow.succ_opt().unwrap())
                .unwrap()
                .current_days,
            0
        );
        assert_eq!(
            serde_json::to_value(result).unwrap(),
            serde_json::json!({"current_days":3,"best_days":3,"total_pomodoros":3,"today_pomodoros":1})
        );
    }
}
