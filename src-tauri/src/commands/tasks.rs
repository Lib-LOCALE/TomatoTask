// Commandes Tauri pour la gestion des tâches
use crate::db::{queries, CreateTaskInput, DbConnection, Task, UpdateTaskInput};
use tauri::State;

/// Récupère toutes les tâches
///
/// # Arguments
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn get_tasks(db: State<DbConnection>) -> Result<Vec<Task>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::get_all_tasks(&conn).map_err(|e| e.to_string())
}

/// Récupère une tâche spécifique par son ID
///
/// # Arguments
/// * `id` - ID de la tâche
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn get_task(id: i64, db: State<DbConnection>) -> Result<Task, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::get_task_by_id(&conn, id).map_err(|e| e.to_string())
}

/// Crée une nouvelle tâche
///
/// # Arguments
/// * `input` - Données de la tâche à créer
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn create_task(input: CreateTaskInput, db: State<DbConnection>) -> Result<Task, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::create_task(&conn, &input).map_err(|e| e.to_string())
}

/// Met à jour une tâche existante
///
/// # Arguments
/// * `id` - ID de la tâche à mettre à jour
/// * `input` - Nouvelles données de la tâche
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn update_task(
    id: i64,
    input: UpdateTaskInput,
    db: State<DbConnection>,
) -> Result<Task, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::update_task(&conn, id, &input).map_err(|e| e.to_string())
}

/// Supprime une tâche
///
/// # Arguments
/// * `id` - ID de la tâche à supprimer
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn delete_task(id: i64, db: State<DbConnection>) -> Result<(), String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::delete_task(&conn, id).map_err(|e| e.to_string())
}

/// Bascule le statut de complétion d'une tâche
///
/// # Arguments
/// * `id` - ID de la tâche
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn toggle_task_completion(id: i64, db: State<DbConnection>) -> Result<Task, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::toggle_task_completion(&conn, id).map_err(|e| e.to_string())
}

/// Récupère les tâches d'un projet spécifique
///
/// # Arguments
/// * `project_id` - ID du projet
/// * `db` - État partagé contenant la connexion à la base de données
#[tauri::command]
pub fn get_tasks_by_project(project_id: i64, db: State<DbConnection>) -> Result<Vec<Task>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;

    queries::get_tasks_by_project(&conn, project_id).map_err(|e| e.to_string())
}
