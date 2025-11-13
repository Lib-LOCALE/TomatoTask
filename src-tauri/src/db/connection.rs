// Gestionnaire de connexion SQLite
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Structure encapsulant la connexion SQLite thread-safe
pub struct DbConnection {
    conn: Arc<Mutex<Connection>>,
}

impl DbConnection {
    /// Crée une nouvelle connexion à la base de données
    ///
    /// # Arguments
    /// * `db_path` - Chemin vers le fichier de base de données
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Active les clés étrangères (désactivées par défaut dans SQLite)
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Crée une connexion en mémoire (pour les tests)
    #[allow(dead_code)]
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Obtient un lock sur la connexion pour exécuter des opérations
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }
}

impl Clone for DbConnection {
    fn clone(&self) -> Self {
        Self {
            conn: Arc::clone(&self.conn),
        }
    }
}
