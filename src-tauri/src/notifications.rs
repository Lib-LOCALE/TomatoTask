// Gestion des notifications système
use tauri::{AppHandle, Manager};

/// Envoie une notification système
pub fn send_notification(app: &AppHandle, title: &str, body: &str) {
    use tauri::api::notification::Notification;

    // Crée et envoie la notification
    let _ = Notification::new(&app.config().tauri.bundle.identifier)
        .title(title)
        .body(body)
        .icon("icons/icon.png")
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
