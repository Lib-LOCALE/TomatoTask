// Point d'entrée principal de l'application Tauri
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod tray;
mod notifications;

use db::{DbConnection, migrations};
use std::path::PathBuf;
use tauri::Manager;

/// Initialise et démarre l'application Tauri
///
/// Cette fonction:
/// 1. Configure le chemin de la base de données
/// 2. Exécute les migrations de schéma
/// 3. Initialise la connexion DB comme état partagé
/// 4. Enregistre toutes les commandes Tauri
/// 5. Configure le système de logs en mode debug
#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Obtient le répertoire de données de l'application
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Crée le répertoire s'il n'existe pas
            std::fs::create_dir_all(&app_dir)
                .expect("Failed to create app data directory");

            // Chemin complet vers la base de données
            let db_path: PathBuf = app_dir.join("tomatotask.db");

            // Initialise la connexion à la base de données
            let db = DbConnection::new(db_path)
                .expect("Failed to open database connection");

            // Exécute les migrations de schéma
            let conn = db.get_connection();
            let conn = conn.lock().expect("Failed to lock database connection");
            migrations::run_migrations(&conn)
                .expect("Failed to run database migrations");
            drop(conn); // Libère le lock

            // Enregistre la connexion DB comme état partagé
            app.manage(db);

            // Active les logs détaillés en mode debug
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        // Enregistre toutes les commandes Tauri disponibles au frontend
        .invoke_handler(tauri::generate_handler![
            // Commandes de gestion des tâches
            commands::get_tasks,
            commands::get_task,
            commands::create_task,
            commands::update_task,
            commands::delete_task,
            commands::toggle_task_completion,
            commands::get_tasks_by_project,
            // Commandes de gestion des projets
            commands::get_projects,
            commands::create_project,
            commands::update_project,
            commands::delete_project,
            // Commandes de gestion des sessions Pomodoro
            commands::create_session,
            commands::complete_session,
            commands::interrupt_session,
            commands::get_sessions_by_date_range,
            // Commandes de gestion des paramètres
            commands::get_settings,
            commands::update_settings,
            // Commandes de résumé/analytics
            commands::get_daily_summary,
            commands::get_weekly_summary,
            // Commandes de notifications (temporairement désactivées)
            // notifications::send_custom_notification,
            // Commandes de system tray
            tray::toggle_window,
            tray::show_window,
            tray::hide_window,
            tray::quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
