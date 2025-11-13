// Requêtes SQL pour la gestion des paramètres
use crate::db::models::Settings;
use rusqlite::{Connection, Result, params};

/// Récupère les paramètres de l'application
///
/// Il n'y a qu'un seul enregistrement de paramètres (singleton avec id=1)
///
/// # Arguments
/// * `conn` - Connexion à la base de données
pub fn get_settings(conn: &Connection) -> Result<Settings> {
    conn.query_row(
        "SELECT work_duration, short_break_duration, long_break_duration,
                pomodoros_until_long_break, language, theme, notification_sound,
                auto_start_breaks, auto_start_pomodoros
         FROM settings
         WHERE id = 1",
        [],
        |row| {
            Ok(Settings {
                work_duration: row.get(0)?,
                short_break_duration: row.get(1)?,
                long_break_duration: row.get(2)?,
                pomodoros_until_long_break: row.get(3)?,
                language: row.get(4)?,
                theme: row.get(5)?,
                notification_sound: row.get(6)?,
                auto_start_breaks: row.get::<_, i32>(7)? != 0,
                auto_start_pomodoros: row.get::<_, i32>(8)? != 0,
            })
        },
    )
}

/// Met à jour les paramètres de l'application
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `settings` - Nouveaux paramètres à enregistrer
pub fn update_settings(conn: &Connection, settings: &Settings) -> Result<Settings> {
    conn.execute(
        "UPDATE settings
         SET work_duration = ?1,
             short_break_duration = ?2,
             long_break_duration = ?3,
             pomodoros_until_long_break = ?4,
             language = ?5,
             theme = ?6,
             notification_sound = ?7,
             auto_start_breaks = ?8,
             auto_start_pomodoros = ?9,
             updated_at = datetime('now')
         WHERE id = 1",
        params![
            &settings.work_duration,
            &settings.short_break_duration,
            &settings.long_break_duration,
            &settings.pomodoros_until_long_break,
            &settings.language,
            &settings.theme,
            &settings.notification_sound,
            if settings.auto_start_breaks { 1 } else { 0 },
            if settings.auto_start_pomodoros { 1 } else { 0 },
        ],
    )?;

    get_settings(conn)
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
    fn test_get_default_settings() {
        let conn = setup_test_db();
        let settings = get_settings(&conn).unwrap();

        assert_eq!(settings.work_duration, 25);
        assert_eq!(settings.short_break_duration, 5);
        assert_eq!(settings.long_break_duration, 15);
        assert_eq!(settings.language, "en");
        assert_eq!(settings.theme, "light");
    }

    #[test]
    fn test_update_settings() {
        let conn = setup_test_db();

        let mut settings = get_settings(&conn).unwrap();
        settings.work_duration = 30;
        settings.language = "fr".to_string();
        settings.theme = "dark".to_string();

        let updated = update_settings(&conn, &settings).unwrap();
        assert_eq!(updated.work_duration, 30);
        assert_eq!(updated.language, "fr");
        assert_eq!(updated.theme, "dark");
    }
}
