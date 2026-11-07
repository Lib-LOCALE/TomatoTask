use crate::Labels;
use serde_json::Value;
use slint::SharedString;

pub const LANGUAGES: [&str; 5] = ["en", "fr", "de", "es", "it"];

pub fn load(index: usize) -> Value {
    let json = [
        include_str!("../assets/en.json"),
        include_str!("../assets/fr.json"),
        include_str!("../assets/de.json"),
        include_str!("../assets/es.json"),
        include_str!("../assets/it.json"),
    ][index];
    serde_json::from_str(json).expect("bundled translations")
}

pub fn text<'a>(locale: &'a Value, key: &'a str) -> &'a str {
    locale[key].as_str().unwrap_or(key)
}

pub fn labels(locale: &Value) -> Labels {
    Labels {
        timer: SharedString::from(text(locale, "timer")),
        mini: SharedString::from(text(locale, "mini")),
        work: SharedString::from(text(locale, "work")),
        short_break: SharedString::from(text(locale, "short-break")),
        long_break: SharedString::from(text(locale, "long-break")),
        start: SharedString::from(text(locale, "start")),
        pause: SharedString::from(text(locale, "pause")),
        resume: SharedString::from(text(locale, "resume")),
        stop: SharedString::from(text(locale, "stop")),
        focus: SharedString::from(text(locale, "focus")),
        exit: SharedString::from(text(locale, "exit")),
        fresco: SharedString::from(text(locale, "fresco")),
        today: SharedString::from(text(locale, "today")),
        plan: SharedString::from(text(locale, "plan")),
        plan_today: SharedString::from(text(locale, "plan-today")),
        unplan: SharedString::from(text(locale, "unplan")),
        new_task: SharedString::from(text(locale, "new-task")),
        edit: SharedString::from(text(locale, "edit")),
        delete: SharedString::from(text(locale, "delete")),
        cancel: SharedString::from(text(locale, "cancel")),
        save: SharedString::from(text(locale, "save")),
        description: SharedString::from(text(locale, "description")),
        task_title: SharedString::from(text(locale, "task-title")),
        delete_task_confirm: SharedString::from(text(locale, "delete-task-confirm")),
        delete_project_confirm: SharedString::from(text(locale, "delete-project-confirm")),
        filter_all: SharedString::from(text(locale, "filter-all")),
        filter_active: SharedString::from(text(locale, "filter-active")),
        filter_completed: SharedString::from(text(locale, "filter-completed")),
        estimate: SharedString::from(text(locale, "estimate")),
        no_tasks: SharedString::from(text(locale, "no-tasks")),
        project: SharedString::from(text(locale, "project")),
        project_list: SharedString::from(text(locale, "project-list")),
        new_project: SharedString::from(text(locale, "new-project")),
        project_name: SharedString::from(text(locale, "project-name")),
        no_projects: SharedString::from(text(locale, "no-projects")),
        color: SharedString::from(text(locale, "color")),
        review: SharedString::from(text(locale, "review")),
        period: SharedString::from(text(locale, "period")),
        settings: SharedString::from(text(locale, "settings")),
        cycle: SharedString::from(text(locale, "cycle")),
        language: SharedString::from(text(locale, "language")),
        theme: SharedString::from(text(locale, "theme")),
        light: SharedString::from(text(locale, "light")),
        dark: SharedString::from(text(locale, "dark")),
        system: SharedString::from(text(locale, "system")),
        auto_break: SharedString::from(text(locale, "auto-break")),
        auto_work: SharedString::from(text(locale, "auto-work")),
        end_sound: SharedString::from(text(locale, "end-sound")),
        no_sound: SharedString::from(text(locale, "no-sound")),
        sounds: SharedString::from(text(locale, "sounds")),
        birds: SharedString::from(text(locale, "birds")),
        forest: SharedString::from(text(locale, "forest")),
        sea: SharedString::from(text(locale, "sea")),
        storm: SharedString::from(text(locale, "storm")),
        export: SharedString::from(text(locale, "export")),
        import: SharedString::from(text(locale, "import")),
        up: SharedString::from(text(locale, "up")),
        down: SharedString::from(text(locale, "down")),
        all_projects: SharedString::from(text(locale, "all-projects")),
        about: SharedString::from(text(locale, "about")),
        overwrite: SharedString::from(text(locale, "overwrite")),
    }
}
