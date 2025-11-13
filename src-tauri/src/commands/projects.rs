// Commandes Tauri pour la gestion des projets
use crate::db::{queries, CreateProjectInput, DbConnection, Project};
use tauri::State;

/// Récupère tous les projets
///
/// # Arguments
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn get_projects(db: State<DbConnection>) -> Result<Vec<Project>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::get_all_projects(&conn).map_err(|e| e.to_string())
}

/// Crée un nouveau projet
///
/// # Arguments
/// * `name` - Nom du projet
/// * `color` - Couleur du projet (optionnel)
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn create_project(
    name: String,
    color: Option<String>,
    db: State<DbConnection>,
) -> Result<Project, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    let input = CreateProjectInput { name, color };
    queries::create_project(&conn, &input).map_err(|e| e.to_string())
}

/// Supprime un projet
///
/// # Arguments
/// * `id` - ID du projet à supprimer
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn delete_project(id: i64, db: State<DbConnection>) -> Result<(), String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::delete_project(&conn, id).map_err(|e| e.to_string())
}
