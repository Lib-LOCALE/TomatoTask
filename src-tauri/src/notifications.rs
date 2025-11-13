// Gestion des notifications système
// NOTE: Temporairement désactivé en attendant la configuration correcte du plugin
use tauri::AppHandle;

/// Envoie une notification système (stub temporaire)
pub fn send_notification(_app: &AppHandle, _title: &str, _body: &str) {
    // TODO: Réactiver avec le bon plugin de notification Tauri 2
    // Pour l'instant, cette fonction ne fait rien pour permettre la compilation
}

/// Envoie une notification de session Pomodoro terminée
pub fn notify_work_complete(app: &AppHandle) {
    send_notification(
        app,
        "Work Session Complete!",
        "Time for a break. Good job! 🍅",
    );
}

/// Envoie une notification de pause terminée
pub fn notify_break_complete(app: &AppHandle) {
    send_notification(
        app,
        "Break Complete!",
        "Ready to focus? Let's start another session! 💪",
    );
}

/// Envoie une notification de tâche terminée
pub fn notify_task_complete(app: &AppHandle, task_title: &str) {
    send_notification(
        app,
        "Task Completed!",
        &format!("✅ {}", task_title),
    );
}

/// Envoie une notification personnalisée depuis le frontend (commande désactivée)
#[allow(dead_code)]
#[tauri::command]
pub fn send_custom_notification(
    app: AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    send_notification(&app, &title, &body);
    Ok(())
}
