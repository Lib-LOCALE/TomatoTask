// Opérations pour la gestion des paramètres
use crate::db::{DbConnection, Settings, queries};

/// Récupère les paramètres de l'application
///
/// # Arguments
/// * `db` - État partagé contenant la connexion à la base de données
pub fn get_settings(db: &DbConnection) -> Result<Settings, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::get_settings(&conn).map_err(|e| e.to_string())
}

/// Met à jour les paramètres de l'application
///
/// # Arguments
/// * `settings` - Nouveaux paramètres
/// * `db` - État partagé contenant la connexion à la base de données
pub fn update_settings(settings: Settings, db: &DbConnection) -> Result<Settings, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::update_settings(&conn, &settings).map_err(|e| e.to_string())
}
