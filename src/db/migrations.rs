// Gestion des migrations de schéma de base de données
use rusqlite::{Connection, Result};

/// Exécute toutes les migrations nécessaires
///
/// Cette fonction vérifie la version actuelle du schéma et applique
/// toutes les migrations manquantes dans l'ordre
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Crée la table de suivi des migrations si elle n'existe pas
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    let current_version = get_schema_version(conn)?;

    // Applique chaque migration dans l'ordre
    if current_version < 1 {
        apply_migration_001(conn)?;
        set_schema_version(conn, 1)?;
    }

    if current_version < 2 {
        apply_migration_002(conn)?;
        set_schema_version(conn, 2)?;
    }

    if current_version < 3 {
        conn.execute_batch("BEGIN; ALTER TABLE tasks ADD COLUMN planned_for TEXT; ALTER TABLE tasks ADD COLUMN plan_position INTEGER; INSERT INTO schema_version (version) VALUES (3); COMMIT;")?;
    }

    if current_version < 4 {
        conn.execute_batch("BEGIN;
            CREATE TABLE settings_v4 (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                work_duration INTEGER DEFAULT 25 CHECK (work_duration BETWEEN 1 AND 180),
                short_break_duration INTEGER DEFAULT 5 CHECK (short_break_duration BETWEEN 1 AND 60),
                long_break_duration INTEGER DEFAULT 15 CHECK (long_break_duration BETWEEN 1 AND 60),
                pomodoros_until_long_break INTEGER DEFAULT 4 CHECK (pomodoros_until_long_break BETWEEN 1 AND 10),
                language TEXT DEFAULT 'en' CHECK (language IN ('en', 'fr', 'es', 'it', 'de')),
                theme TEXT DEFAULT 'light' CHECK (theme IN ('light', 'dark', 'system')),
                notification_sound TEXT DEFAULT 'default',
                auto_start_breaks INTEGER DEFAULT 0,
                auto_start_pomodoros INTEGER DEFAULT 0,
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            INSERT INTO settings_v4 SELECT * FROM settings;
            DROP TABLE settings;
            ALTER TABLE settings_v4 RENAME TO settings;
            INSERT INTO schema_version (version) VALUES (4);
            COMMIT;")?;
    }

    Ok(())
}

/// Obtient la version actuelle du schéma
fn get_schema_version(conn: &Connection) -> Result<i32> {
    conn.query_row("SELECT MAX(version) FROM schema_version", [], |row| {
        row.get::<_, Option<i32>>(0)
            .map(|version| version.unwrap_or(0))
    })
}

/// Enregistre une nouvelle version de schéma
fn set_schema_version(conn: &Connection, version: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        [version],
    )?;
    Ok(())
}

/// Migration 001: Schéma initial complet
///
/// Crée toutes les tables nécessaires pour TomatoTask:
/// - projects: Organisation des tâches par projet
/// - tasks: Tâches avec estimation Pomodoro
/// - pomodoro_sessions: Historique des sessions Pomodoro
/// - settings: Préférences utilisateur (singleton)
fn apply_migration_001(conn: &Connection) -> Result<()> {
    // Table des projets
    conn.execute(
        "CREATE TABLE projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            color TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Table des tâches
    conn.execute(
        "CREATE TABLE tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            project_id INTEGER,
            estimated_pomodoros INTEGER DEFAULT 0,
            completed_pomodoros INTEGER DEFAULT 0,
            is_completed INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            completed_at TEXT,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // Table des sessions Pomodoro
    conn.execute(
        "CREATE TABLE pomodoro_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER,
            started_at TEXT NOT NULL,
            completed_at TEXT,
            duration_minutes INTEGER NOT NULL,
            session_type TEXT NOT NULL,
            interrupted INTEGER DEFAULT 0,
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL,
            CHECK (session_type IN ('work', 'short_break', 'long_break'))
        )",
        [],
    )?;

    // Table des paramètres (singleton)
    conn.execute(
        "CREATE TABLE settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            work_duration INTEGER DEFAULT 25,
            short_break_duration INTEGER DEFAULT 5,
            long_break_duration INTEGER DEFAULT 15,
            pomodoros_until_long_break INTEGER DEFAULT 4,
            language TEXT DEFAULT 'en',
            theme TEXT DEFAULT 'light',
            notification_sound TEXT DEFAULT 'default',
            auto_start_breaks INTEGER DEFAULT 0,
            auto_start_pomodoros INTEGER DEFAULT 0,
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            CHECK (work_duration BETWEEN 1 AND 180),
            CHECK (short_break_duration BETWEEN 1 AND 60),
            CHECK (long_break_duration BETWEEN 1 AND 60),
            CHECK (pomodoros_until_long_break BETWEEN 1 AND 10),
            CHECK (language IN ('en', 'fr', 'es', 'it', 'de')),
            CHECK (theme IN ('light', 'dark'))
        )",
        [],
    )?;

    // Index pour optimiser les requêtes fréquentes
    conn.execute("CREATE INDEX idx_tasks_project ON tasks(project_id)", [])?;

    conn.execute(
        "CREATE INDEX idx_tasks_completed ON tasks(is_completed)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX idx_sessions_task ON pomodoro_sessions(task_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX idx_sessions_date ON pomodoro_sessions(started_at)",
        [],
    )?;

    // Insère les paramètres par défaut
    conn.execute("INSERT INTO settings (id) VALUES (1)", [])?;

    Ok(())
}

/// Migration 002: Ajout de la colonne position pour le tri
///
/// Ajoute une colonne position aux tables projects et tasks
/// Initialise la position avec l'ID pour maintenir l'ordre de création par défaut
fn apply_migration_002(conn: &Connection) -> Result<()> {
    // Ajout de la colonne position à la table projects
    conn.execute(
        "ALTER TABLE projects ADD COLUMN position INTEGER DEFAULT 0",
        [],
    )?;

    // Initialise la position avec l'ID pour les projets existants
    conn.execute("UPDATE projects SET position = id", [])?;

    // Ajout de la colonne position à la table tasks
    conn.execute(
        "ALTER TABLE tasks ADD COLUMN position INTEGER DEFAULT 0",
        [],
    )?;

    // Initialise la position avec l'ID pour les tâches existantes
    conn.execute("UPDATE tasks SET position = id", [])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migrations_run_successfully() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();

        // Vérifie que toutes les tables existent
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<String>, _>>()
            .unwrap();

        assert!(tables.contains(&"projects".to_string()));
        assert!(tables.contains(&"tasks".to_string()));
        assert!(tables.contains(&"pomodoro_sessions".to_string()));
        assert!(tables.contains(&"settings".to_string()));
    }

    #[test]
    fn migration_plan_sur_base_deja_en_version_deux() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("CREATE TABLE schema_version (version INTEGER PRIMARY KEY, applied_at TEXT DEFAULT (datetime('now'))); INSERT INTO schema_version(version) VALUES (1), (2);").unwrap();
        apply_migration_001(&conn).unwrap();
        apply_migration_002(&conn).unwrap();
        conn.execute("INSERT INTO tasks(title) VALUES ('ancien')", [])
            .unwrap();
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();
        let values: (Option<String>, Option<i64>) = conn
            .query_row(
                "SELECT planned_for, plan_position FROM tasks WHERE id = 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(values, (None, None));
    }

    #[test]
    fn migration_theme_systeme_preserve_les_parametres() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("CREATE TABLE schema_version (version INTEGER PRIMARY KEY, applied_at TEXT DEFAULT (datetime('now'))); INSERT INTO schema_version(version) VALUES (1), (2), (3);").unwrap();
        apply_migration_001(&conn).unwrap();
        apply_migration_002(&conn).unwrap();
        conn.execute(
            "UPDATE settings SET work_duration = 42, theme = 'dark' WHERE id = 1",
            [],
        )
        .unwrap();
        run_migrations(&conn).unwrap();
        conn.execute("UPDATE settings SET theme = 'system' WHERE id = 1", [])
            .unwrap();
        let values: (i32, String) = conn
            .query_row(
                "SELECT work_duration, theme FROM settings WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(values, (42, "system".to_string()));
    }

    #[test]
    fn test_default_settings_created() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();

        let work_duration: i32 = conn
            .query_row(
                "SELECT work_duration FROM settings WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(work_duration, 25);
    }
}
