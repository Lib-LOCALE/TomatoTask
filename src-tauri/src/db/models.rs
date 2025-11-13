// Modèles de données Rust pour la base de données
use serde::{Deserialize, Serialize};

/// Représente un projet pour organiser les tâches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Input pour créer un nouveau projet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub color: Option<String>,
}

/// Représente une tâche à accomplir
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<i64>,
    pub estimated_pomodoros: i32,
    pub completed_pomodoros: i32,
    pub is_completed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

/// Input pour créer une nouvelle tâche
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<i64>,
    pub estimated_pomodoros: i32,
}

/// Input pour mettre à jour une tâche existante
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<i64>,
    pub estimated_pomodoros: i32,
}

/// Représente une session Pomodoro complétée
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroSession {
    pub id: i64,
    pub task_id: Option<i64>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub duration_minutes: i32,
    pub session_type: SessionType,
    pub interrupted: bool,
}

/// Type de session Pomodoro
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionType {
    /// Convertit le type de session en string pour la base de données
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionType::Work => "work",
            SessionType::ShortBreak => "short_break",
            SessionType::LongBreak => "long_break",
        }
    }

    /// Parse une string depuis la base de données
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "work" => Ok(SessionType::Work),
            "short_break" => Ok(SessionType::ShortBreak),
            "long_break" => Ok(SessionType::LongBreak),
            _ => Err(format!("Invalid session type: {}", s)),
        }
    }
}

/// Input pour créer une nouvelle session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionInput {
    pub task_id: Option<i64>,
    pub duration_minutes: i32,
    pub session_type: SessionType,
}

/// Paramètres de configuration de l'application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub work_duration: i32,
    pub short_break_duration: i32,
    pub long_break_duration: i32,
    pub pomodoros_until_long_break: i32,
    pub language: String,
    pub theme: String,
    pub notification_sound: String,
    pub auto_start_breaks: bool,
    pub auto_start_pomodoros: bool,
}

/// Résumé quotidien de productivité
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySummary {
    pub date: String,
    pub completed_tasks_count: i32,
    pub completed_pomodoros_count: i32,
    pub total_focus_minutes: i32,
}
