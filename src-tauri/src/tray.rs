// Gestion du system tray (icône système)
// Note: Pour Tauri 2, le tray est principalement configuré via tauri.conf.json
// Ces commandes permettent au frontend de contrôler la fenêtre depuis le tray

use tauri::{AppHandle, Manager};

/// Commande Tauri pour basculer la visibilité de la fenêtre
#[tauri::command]
pub fn toggle_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
        Ok(())
    } else {
        Err("Window not found".to_string())
    }
}

/// Commande Tauri pour afficher la fenêtre
#[tauri::command]
pub fn show_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
        Ok(())
    } else {
        Err("Window not found".to_string())
    }
}

/// Commande Tauri pour cacher la fenêtre
#[tauri::command]
pub fn hide_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
        Ok(())
    } else {
        Err("Window not found".to_string())
    }
}

/// Commande Tauri pour quitter l'application
#[tauri::command]
pub fn quit_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}
