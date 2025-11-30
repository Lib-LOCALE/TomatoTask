use crate::db::DbConnection;
use crate::db::models::{DailyFocusTime, ProjectDistribution};
use crate::db::queries::stats;
use tauri::State;

/// Commande pour obtenir le temps de focus quotidien
#[tauri::command]
pub fn get_daily_focus_time(db: State<DbConnection>) -> Result<Vec<DailyFocusTime>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;
    
    stats::get_daily_focus_time(&conn).map_err(|e| e.to_string())
}

/// Commande pour obtenir la distribution des projets
#[tauri::command]
pub fn get_project_distribution(db: State<DbConnection>) -> Result<Vec<ProjectDistribution>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;
    
    stats::get_project_distribution(&conn).map_err(|e| e.to_string())
}

/// Commande pour obtenir l'historique de focus sur une période
#[tauri::command]
pub fn get_focus_history(days: i32, db: State<DbConnection>) -> Result<Vec<DailyFocusTime>, String> {
    let conn = db.get_connection();
    let conn = conn.lock().map_err(|e| e.to_string())?;
    
    stats::get_focus_history(&conn, days).map_err(|e| e.to_string())
}
