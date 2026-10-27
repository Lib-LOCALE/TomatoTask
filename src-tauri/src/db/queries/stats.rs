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

#[derive(Debug, serde::Serialize)]
pub struct InsightDay {
    date: String,
    pomodoros: u32,
    focus_minutes: i64,
}
#[derive(Debug, serde::Serialize)]
pub struct InsightTask {
    task_id: i64,
    title: String,
    pomodoros: u32,
}
#[derive(Debug, serde::Serialize)]
pub struct InsightProject {
    project_id: i64,
    name: String,
    color: Option<String>,
    pomodoros: u32,
}
#[derive(Debug, serde::Serialize)]
pub struct Insights {
    total_focus_minutes: i64,
    total_pomodoros: u32,
    daily: Vec<InsightDay>,
    by_hour: [u32; 24],
    best_hour: Option<u8>,
    by_weekday: [u32; 7],
    completion_rate: f64,
    previous_period_pomodoros: u32,
    top_tasks: Vec<InsightTask>,
    top_projects: Vec<InsightProject>,
}

pub fn get_insights(conn: &Connection, days: u32, today: chrono::NaiveDate) -> Result<Insights> {
    use chrono::{Datelike, Duration};
    let mut result = Insights {
        total_focus_minutes: 0,
        total_pomodoros: 0,
        daily: Vec::new(),
        by_hour: [0; 24],
        best_hour: None,
        by_weekday: [0; 7],
        completion_rate: 0.0,
        previous_period_pomodoros: 0,
        top_tasks: Vec::new(),
        top_projects: Vec::new(),
    };
    if days == 0 {
        return Ok(result);
    }
    if days > 36_600 {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let start = today
        .checked_sub_signed(Duration::days(i64::from(days) - 1))
        .ok_or(rusqlite::Error::InvalidQuery)?;
    let previous = start
        .checked_sub_signed(Duration::days(i64::from(days)))
        .ok_or(rusqlite::Error::InvalidQuery)?;
    for offset in 0..days {
        let date = start
            .checked_add_signed(Duration::days(i64::from(offset)))
            .ok_or(rusqlite::Error::InvalidQuery)?;
        result.daily.push(InsightDay {
            date: date.to_string(),
            pomodoros: 0,
            focus_minutes: 0,
        });
    }
    let mut stmt = conn.prepare("SELECT date(s.started_at, 'localtime'), CAST(strftime('%H', s.started_at, 'localtime') AS INTEGER), s.completed_at IS NOT NULL AND s.interrupted = 0, s.duration_minutes, t.id, t.title, p.id, p.name, p.color FROM pomodoro_sessions s LEFT JOIN tasks t ON t.id = s.task_id LEFT JOIN projects p ON p.id = t.project_id WHERE s.session_type = 'work' AND date(s.started_at, 'localtime') BETWEEN ?1 AND ?2")?;
    let mut rows = stmt.query((previous.to_string(), today.to_string()))?;
    let mut started = 0_u32;
    let mut tasks = std::collections::HashMap::<i64, InsightTask>::new();
    let mut projects = std::collections::HashMap::<i64, InsightProject>::new();
    while let Some(row) = rows.next()? {
        let date: String = row.get(0)?;
        let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map_err(|_| rusqlite::Error::InvalidQuery)?;
        let completed: bool = row.get(2)?;
        if date < start {
            if completed {
                result.previous_period_pomodoros += 1;
            }
            continue;
        }
        started += 1;
        if !completed {
            continue;
        }
        let hour: u8 = row.get(1)?;
        let minutes: i64 = row.get(3)?;
        let index = (date - start).num_days() as usize;
        result.daily[index].pomodoros += 1;
        result.daily[index].focus_minutes += minutes;
        result.total_pomodoros += 1;
        result.total_focus_minutes += minutes;
        result.by_hour[usize::from(hour)] += 1;
        result.by_weekday[date.weekday().num_days_from_monday() as usize] += 1;
        if let Some(id) = row.get::<_, Option<i64>>(4)? {
            let title: String = row.get(5)?;
            tasks
                .entry(id)
                .or_insert(InsightTask {
                    task_id: id,
                    title,
                    pomodoros: 0,
                })
                .pomodoros += 1;
        }
        if let Some(id) = row.get::<_, Option<i64>>(6)? {
            let name: String = row.get(7)?;
            let color: Option<String> = row.get(8)?;
            projects
                .entry(id)
                .or_insert(InsightProject {
                    project_id: id,
                    name,
                    color,
                    pomodoros: 0,
                })
                .pomodoros += 1;
        }
    }
    if started > 0 {
        result.completion_rate = f64::from(result.total_pomodoros) / f64::from(started);
    }
    result.best_hour = result
        .by_hour
        .iter()
        .enumerate()
        .filter(|(_, n)| **n > 0)
        .max_by_key(|(hour, n)| (**n, std::cmp::Reverse(*hour)))
        .map(|(hour, _)| hour as u8);
    result.top_tasks = tasks.into_values().collect();
    result
        .top_tasks
        .sort_by_key(|t| (std::cmp::Reverse(t.pomodoros), t.task_id));
    result.top_tasks.truncate(5);
    result.top_projects = projects.into_values().collect();
    result
        .top_projects
        .sort_by_key(|p| (std::cmp::Reverse(p.pomodoros), p.project_id));
    Ok(result)
}

#[cfg(test)]
mod insight_tests {
    use super::*;
    #[test]
    fn heures_locales_autour_du_changement_d_heure() {
        let conn = Connection::open_in_memory().unwrap();
        crate::db::migrations::run_migrations(&conn).unwrap();
        conn.execute_batch("INSERT INTO pomodoro_sessions(started_at, completed_at, duration_minutes, session_type) VALUES ('2026-03-29 00:30:00', '2026-03-29 00:55:00', 25, 'work'), ('2026-03-29 01:30:00', '2026-03-29 01:55:00', 25, 'work')").unwrap();
        let today = chrono::NaiveDate::from_ymd_opt(2026, 3, 29).unwrap();
        let result = get_insights(&conn, 1, today).unwrap();
        if std::env::var("TZ").as_deref() == Ok("Europe/Paris") {
            assert_eq!(result.by_hour[1], 1);
            assert_eq!(result.by_hour[2], 0);
            assert_eq!(result.by_hour[3], 1);
        }
    }

    #[test]
    fn periodes_vides_et_sessions_de_travail() {
        let conn = Connection::open_in_memory().unwrap();
        crate::db::migrations::run_migrations(&conn).unwrap();
        conn.execute_batch("INSERT INTO projects(name, color) VALUES ('projet', '#fff'); INSERT INTO tasks(title, project_id) VALUES ('tâche', 1); INSERT INTO pomodoro_sessions(task_id, started_at, completed_at, duration_minutes, session_type, interrupted) VALUES (1, '2026-01-02 12:00:00', '2026-01-02 12:25:00', 25, 'work', 0), (1, '2026-01-02 13:00:00', NULL, 25, 'work', 0), (NULL, '2025-12-31 12:00:00', '2025-12-31 12:25:00', 25, 'work', 0), (1, '2026-01-02 14:00:00', '2026-01-02 14:05:00', 5, 'short_break', 0)").unwrap();
        let today = chrono::NaiveDate::from_ymd_opt(2026, 1, 2).unwrap();
        let empty = get_insights(&conn, 0, today).unwrap();
        assert!(empty.daily.is_empty());
        assert_eq!(empty.best_hour, None);
        let stats = get_insights(&conn, 2, today).unwrap();
        assert_eq!(stats.daily.len(), 2);
        assert_eq!(stats.daily[0].pomodoros, 0);
        assert_eq!(
            (
                stats.total_pomodoros,
                stats.total_focus_minutes,
                stats.previous_period_pomodoros
            ),
            (1, 25, 1)
        );
        assert_eq!(stats.completion_rate, 0.5);
        assert_eq!(stats.top_tasks[0].task_id, 1);
        assert_eq!(stats.top_projects[0].project_id, 1);
        assert_eq!(stats.by_hour.iter().sum::<u32>(), 1);
        assert_eq!(stats.by_weekday.iter().sum::<u32>(), 1);
    }
}
