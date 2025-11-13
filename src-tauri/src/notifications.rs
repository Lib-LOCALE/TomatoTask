// Gestion des notifications système
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

/// Envoie une notification système
pub fn send_notification(app: &AppHandle, title: &str, body: &str) {
    // Crée et envoie la notification avec le nouveau plugin
    let _ = app
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show();
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

/// Envoie une notification personnalisée depuis le frontend
#[tauri::command]
pub fn send_custom_notification(
    app: AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    send_notification(&app, &title, &body);
    Ok(())
}
