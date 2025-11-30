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

/// Met à jour un projet existant
///
/// # Arguments
/// * `id` - ID du projet à mettre à jour
/// * `name` - Nouveau nom du projet
/// * `color` - Nouvelle couleur du projet
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn update_project(
    id: i64,
    name: String,
    color: String,
    db: State<DbConnection>,
) -> Result<Project, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::update_project(&conn, id, &name, &color).map_err(|e| e.to_string())
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

/// Réordonne les projets
///
/// # Arguments
/// * `project_ids` - Liste des IDs de projets dans le nouvel ordre
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn reorder_projects(project_ids: Vec<i64>, db: State<DbConnection>) -> Result<(), String> {
    let conn = db.get_connection();
    let mut conn = conn.lock().map_err(|e| e.to_string())?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    for (index, id) in project_ids.iter().enumerate() {
        tx.execute(
            "UPDATE projects SET position = ?1 WHERE id = ?2",
            (index as i32, id),
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}
