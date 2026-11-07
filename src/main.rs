#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod locale;
mod onboarding;
mod timer;
mod tray;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::{Duration, Instant};

use chrono::Local;
use slint::winit_030::{WinitWindowAccessor, winit};
use slint::{ComponentHandle, ModelRc, SharedString, Timer, TimerMode, VecModel};
use tomatotask::commands::*;
use tomatotask::db::{
    CreateTaskInput, DbConnection, Project, SessionType, Settings, Task, UpdateTaskInput,
    migrations,
};

use audio::Audio;
use timer::Countdown;

slint::include_modules!();

struct Desktop {
    db: DbConnection,
    settings: Settings,
    locale: serde_json::Value,
    timer: Countdown,
    audio: Audio,
    task_id: Option<i64>,
    focus_tasks: Vec<Task>,
    visible_tasks: Vec<Task>,
    projects: Vec<Project>,
    period: u32,
    window_size: Option<slint::PhysicalSize>,
    window_maximized: bool,
    tray: Option<tray::Tray>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = dirs::data_dir()
        .ok_or("Could not locate the user data directory")?
        .join("com.tomatotask.app");
    std::fs::create_dir_all(&data)?;
    let db = DbConnection::new(data.join("tomatotask.db"))?;
    {
        let conn = db.get_connection();
        let conn = conn.lock().map_err(|e| e.to_string())?;
        migrations::run_migrations(&conn)?;
    }
    let show_welcome = onboarding::pending(&db)?;
    let settings = get_settings(&db)?;
    let state = Rc::new(RefCell::new(Desktop {
        db,
        locale: locale::load(0),
        timer: Countdown::new(&settings),
        settings,
        audio: Audio::default(),
        task_id: None,
        focus_tasks: Vec::new(),
        visible_tasks: Vec::new(),
        projects: Vec::new(),
        period: 7,
        window_size: None,
        window_maximized: false,
        tray: None,
    }));

    let ui = App::new()?;
    ui.set_welcoming(show_welcome);
    state.borrow_mut().load_settings(&ui);
    state.borrow_mut().refresh(&ui)?;
    state.borrow().paint_timer(&ui, Instant::now());

    // The generated Slint callbacks each keep a weak handle to the same window.
    macro_rules! bind {
        ($callback:ident, |$desktop:ident, $view:ident $(, $arg:ident)*| $body:expr) => {{
            let weak = ui.as_weak();
            let state = state.clone();
            ui.$callback(move |$($arg),*| {
                if let Some($view) = weak.upgrade() {
                    let $desktop = &mut *state.borrow_mut();
                    $view.set_error("".into());
                    $view.set_message("".into());
                    let result: Result<(), String> = $body;
                    if let Err(error) = result {
                        $view.set_error(error.into());
                    }
                }
            });
        }};
    }

    bind!(on_finish_welcome, |app, ui, make_task| app
        .finish_welcome(&ui, make_task));

    bind!(on_toggle, |app, ui| app.toggle(&ui));
    bind!(on_stop, |app, ui| app.stop(&ui));
    bind!(on_kind, |app, ui, index| {
        if app.timer.id.is_none() {
            let kind = match index {
                1 => SessionType::ShortBreak,
                2 => SessionType::LongBreak,
                _ => SessionType::Work,
            };
            app.timer.reset(kind, &app.settings);
            app.paint_timer(&ui, Instant::now());
        }
        Ok(())
    });
    bind!(on_choose_task, |app, ui, index| {
        app.task_id = option_at(&app.focus_tasks, index - 1).map(|task| task.id);
        ui.set_focus_task_index(index);
        app.paint_timer(&ui, Instant::now());
        Ok(())
    });
    bind!(on_filter_tasks, |app, ui, index| {
        ui.set_filter(index);
        ui.set_selected_task(-1);
        ui.set_editing_task(false);
        ui.set_confirm_task(false);
        app.refresh_tasks(&ui)
    });
    bind!(on_filter_project, |app, ui, index| {
        ui.set_project_filter(index);
        ui.set_selected_task(-1);
        ui.set_editing_task(false);
        ui.set_confirm_task(false);
        app.refresh_tasks(&ui)
    });
    bind!(on_task_action, |app, ui, action, index| app
        .task_action(&ui, &action, index));
    bind!(on_project_action, |app, ui, action, index| app
        .project_action(&ui, &action, index));
    bind!(on_save_settings, |app, ui, prefs| app
        .save_settings(&ui, prefs));
    bind!(on_review, |app, ui, index| {
        app.period = [7, 30, 90][index.clamp(0, 2) as usize];
        app.refresh_review(&ui)
    });
    let weak = ui.as_weak();
    let backup_state = state.clone();
    ui.on_backup(move |import| {
        let Some(ui) = weak.upgrade() else {
            return;
        };
        if ui.get_backup_busy() {
            return;
        }
        ui.set_backup_busy(true);
        ui.set_error("".into());
        ui.set_message("".into());
        let dialog = ui
            .window()
            .with_winit_window(|window| {
                rfd::AsyncFileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_parent(window)
            })
            .unwrap_or_else(|| rfd::AsyncFileDialog::new().add_filter("JSON", &["json"]));
        let weak = weak.clone();
        let state = backup_state.clone();
        slint::spawn_local(async move {
            let file = if import {
                dialog.pick_file().await
            } else {
                dialog
                    .set_file_name("tomatotask-backup.json")
                    .save_file()
                    .await
            };
            let Some(ui) = weak.upgrade() else {
                return;
            };
            ui.set_backup_busy(false);
            if let Some(file) = file {
                let mut app = state.borrow_mut();
                if let Err(error) = app.backup(&ui, import, file.path()) {
                    let key = if import {
                        "chrome.settings.importError"
                    } else {
                        "chrome.settings.exportError"
                    };
                    ui.set_error(format!("{} {error}", app.text(key)).into());
                }
            }
        })
        .expect("backup dialog runs on the UI thread");
    });
    bind!(on_ambient, |app, _ui, index| app.audio.ambient(index));
    bind!(on_set_focus_mode, |_app, ui, enabled| {
        ui.window().set_fullscreen(enabled);
        ui.set_focus_mode(enabled);
        Ok(())
    });
    bind!(on_compact, |app, ui, enabled| {
        use winit::window::WindowLevel;
        if enabled {
            app.window_size = Some(ui.window().size());
            app.window_maximized = ui.window().is_maximized();
            ui.window().set_maximized(false);
            ui.set_mini(true);
            ui.window().with_winit_window(|window| {
                window.set_window_level(WindowLevel::AlwaysOnTop);
                window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(300, 180)));
            });
            ui.window().set_size(slint::LogicalSize::new(320.0, 190.0));
        } else {
            ui.set_mini(false);
            ui.window().with_winit_window(|window| {
                window.set_window_level(WindowLevel::Normal);
            });
            if let Some(size) = app.window_size.take() {
                ui.window().set_size(size);
            }
            ui.window().set_maximized(app.window_maximized);
        }
        Ok(())
    });

    let ticker = Timer::default();
    let weak = ui.as_weak();
    let ticking_state = state.clone();
    ticker.start(TimerMode::Repeated, Duration::from_millis(200), move || {
        if let Some(ui) = weak.upgrade() {
            let mut app = ticking_state.borrow_mut();
            if let Err(error) = app.handle_tray(&ui) {
                ui.set_error(error.into());
            }
            app.tick(&ui);
        }
    });
    let weak = ui.as_weak();
    let closing_state = state.clone();
    ui.window().on_close_requested(move || {
        let mut app = closing_state.borrow_mut();
        if let Some(ui) = weak.upgrade()
            && let Err(error) = app.stop(&ui)
        {
            ui.set_error(error.into());
            return slint::CloseRequestResponse::KeepWindowShown;
        }
        slint::CloseRequestResponse::HideWindow
    });
    ui.show()?;
    let weak = ui.as_weak();
    Timer::single_shot(Duration::ZERO, move || {
        if let Some(ui) = weak.upgrade() {
            let mut app = state.borrow_mut();
            match tray::Tray::new(&app.locale, ui.get_welcoming()) {
                Ok(tray) => app.tray = Some(tray),
                Err(error) => eprintln!("Could not start the TomatoTask tray: {error}"),
            }
        }
    });
    ui.window().with_winit_window(|window| {
        ui.global::<Theme>()
            .set_system_dark(window.theme() == Some(winit::window::Theme::Dark));
    });
    let weak = ui.as_weak();
    ui.window().on_winit_window_event(move |_, event| {
        if let winit::event::WindowEvent::ThemeChanged(theme) = event
            && let Some(ui) = weak.upgrade()
        {
            ui.global::<Theme>()
                .set_system_dark(*theme == winit::window::Theme::Dark);
        }
        slint::winit_030::EventResult::Propagate
    });
    slint::run_event_loop()?;
    Ok(())
}

fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
    ModelRc::new(VecModel::from(items))
}

fn option_at<T>(items: &[T], index: i32) -> Option<&T> {
    usize::try_from(index)
        .ok()
        .and_then(|index| items.get(index))
}

fn parse_color(value: &str) -> Option<slint::Color> {
    let hex = value.strip_prefix('#').filter(|hex| hex.len() == 6)?;
    let rgb = u32::from_str_radix(hex, 16).ok()?;
    Some(slint::Color::from_rgb_u8(
        (rgb >> 16) as u8,
        (rgb >> 8) as u8,
        rgb as u8,
    ))
}

fn index_of(value: &str, choices: &[&str]) -> i32 {
    choices.iter().position(|item| *item == value).unwrap_or(0) as i32
}

const THEMES: [&str; 3] = ["light", "dark", "system"];
const SOUNDS: [&str; 2] = ["default", "none"];

impl Desktop {
    fn text(&self, key: &str) -> SharedString {
        locale::text(&self.locale, key).into()
    }

    fn format(&self, key: &str, values: &[(&str, String)]) -> SharedString {
        let mut text = locale::text(&self.locale, key).to_owned();
        for (name, value) in values {
            text = text.replace(&format!("{{{name}}}"), value);
        }
        text.into()
    }

    fn finish_welcome(&mut self, ui: &App, make_task: bool) -> Result<(), String> {
        let title = ui.get_welcome_task();
        if make_task && !onboarding::valid_title(title.as_str()) {
            return Err(self.text("welcome-invalid").to_string());
        }
        let task = onboarding::finish(&self.db, make_task.then_some(title.as_str()))?;
        ui.set_welcoming(false);
        if let Some(tray) = &self.tray {
            tray.enable_timer();
        }
        if let Some(task) = task {
            self.task_id = Some(task.id);
            self.refresh_tasks(ui)?;
            self.paint_timer(ui, Instant::now());
        }
        Ok(())
    }

    fn handle_tray(&mut self, ui: &App) -> Result<(), String> {
        for action in tray::actions() {
            match action {
                tray::Action::Open => {
                    ui.show().map_err(|err| err.to_string())?;
                    ui.window().set_minimized(false);
                    ui.window()
                        .with_winit_window(|window| window.focus_window());
                }
                tray::Action::Toggle if !ui.get_welcoming() => self.toggle(ui)?,
                tray::Action::Toggle => {}
                tray::Action::Quit => {
                    self.stop(ui)?;
                    return slint::quit_event_loop().map_err(|err| err.to_string());
                }
            }
        }
        Ok(())
    }

    fn load_settings(&mut self, ui: &App) {
        let language = index_of(&self.settings.language, &locale::LANGUAGES);
        self.locale = locale::load(language as usize);
        if let Some(tray) = &self.tray {
            tray.localize(&self.locale);
        }
        ui.set_labels(locale::labels(&self.locale));
        ui.set_prefs(Preferences {
            work: self.settings.work_duration,
            short_break: self.settings.short_break_duration,
            long_break: self.settings.long_break_duration,
            cycle: self.settings.pomodoros_until_long_break,
            language,
            theme: index_of(&self.settings.theme, &THEMES),
            auto_break: self.settings.auto_start_breaks,
            auto_work: self.settings.auto_start_pomodoros,
            sound: index_of(&self.settings.notification_sound, &SOUNDS),
        });
        ui.global::<Theme>().set_mode(ui.get_prefs().theme);
    }

    fn save_settings(&mut self, ui: &App, prefs: Preferences) -> Result<(), String> {
        let settings = Settings {
            work_duration: prefs.work.clamp(1, 180),
            short_break_duration: prefs.short_break.clamp(1, 60),
            long_break_duration: prefs.long_break.clamp(1, 60),
            pomodoros_until_long_break: prefs.cycle.clamp(1, 10),
            language: locale::LANGUAGES[prefs.language.clamp(0, 4) as usize].into(),
            theme: THEMES[prefs.theme.clamp(0, 2) as usize].into(),
            notification_sound: SOUNDS[prefs.sound.clamp(0, 1) as usize].into(),
            auto_start_breaks: prefs.auto_break,
            auto_start_pomodoros: prefs.auto_work,
        };
        self.settings = update_settings(settings, &self.db)?;
        self.load_settings(ui);
        if self.timer.id.is_none() {
            self.timer.reset(self.timer.kind.clone(), &self.settings);
        }
        self.refresh(ui)?;
        ui.set_message(self.text("settings.saved"));
        Ok(())
    }

    fn refresh(&mut self, ui: &App) -> Result<(), String> {
        self.projects = get_projects(&self.db)?;
        let projects = std::iter::once(self.text("projects.noProject"))
            .chain(self.projects.iter().map(|p| p.name.as_str().into()))
            .collect();
        ui.set_projects(model(projects));
        ui.set_project_filters(model(
            std::iter::once(self.text("all-projects"))
                .chain(std::iter::once(self.text("projects.noProject")))
                .chain(
                    self.projects
                        .iter()
                        .map(|project| project.name.as_str().into()),
                )
                .collect(),
        ));
        ui.set_project_filter(0);
        ui.set_selected_project(-1);
        self.refresh_tasks(ui)?;
        self.refresh_review(ui)?;
        self.paint_timer(ui, Instant::now());
        Ok(())
    }

    fn refresh_tasks(&mut self, ui: &App) -> Result<(), String> {
        let selected_task_id =
            option_at(&self.visible_tasks, ui.get_selected_task()).map(|task| task.id);
        let today = Local::now().date_naive().to_string();
        let plan = get_day_plan(today.clone(), &self.db)?;
        let tasks = get_tasks(&self.db)?;
        self.focus_tasks = tasks
            .iter()
            .filter(|task| {
                !task.is_completed || Some(task.id) == self.task_id && self.timer.id.is_some()
            })
            .cloned()
            .collect();
        let selected = self
            .task_id
            .and_then(|id| self.focus_tasks.iter().position(|task| task.id == id));
        if selected.is_none() && self.timer.id.is_none() {
            self.task_id = None;
        }
        let focus_tasks = std::iter::once(self.text("tasks.selectTask"))
            .chain(
                self.focus_tasks
                    .iter()
                    .map(|task| task.title.as_str().into()),
            )
            .collect();
        ui.set_focus_tasks(model(focus_tasks));
        ui.set_focus_task_index(selected.map_or(0, |index| index as i32 + 1));

        self.visible_tasks = match ui.get_filter() {
            1 => tasks.into_iter().filter(|t| !t.is_completed).collect(),
            2 => tasks.into_iter().filter(|t| t.is_completed).collect(),
            3 => plan.tasks.iter().map(|entry| entry.task.clone()).collect(),
            _ => tasks,
        };
        if ui.get_project_filter() > 0 {
            let project_id = option_at(&self.projects, ui.get_project_filter() - 2).map(|p| p.id);
            self.visible_tasks
                .retain(|task| task.project_id == project_id);
        }
        let items = self
            .visible_tasks
            .iter()
            .map(|task| {
                let project = self.projects.iter().find(|p| Some(p.id) == task.project_id);
                let mut details =
                    format!("{}/{}", task.completed_pomodoros, task.estimated_pomodoros);
                if let Some(project) = project {
                    details.push_str(&format!(" · {}", project.name));
                }
                let planned = plan.tasks.iter().any(|entry| entry.task.id == task.id);
                if planned && task.planned_for.as_deref() != Some(&today) {
                    details.push_str(&format!(" · {}", self.text("plan.carried")));
                }
                TaskRow {
                    title: task.title.as_str().into(),
                    details: details.into(),
                    done: task.is_completed,
                    planned,
                    pigment: project
                        .and_then(|p| p.color.as_deref())
                        .and_then(parse_color)
                        .unwrap_or_default(),
                }
            })
            .collect();
        ui.set_tasks(model(items));
        if let Some(id) = selected_task_id {
            let selected = self.visible_tasks.iter().position(|task| task.id == id);
            ui.set_selected_task(selected.map_or(-1, |index| index as i32));
            if selected.is_none() {
                ui.set_editing_task(false);
                ui.set_confirm_task(false);
            }
        }
        ui.set_today_count(plan.completed_pomodoros as i32);
        ui.set_plan_count(plan.planned_pomodoros as i32);
        ui.set_total_count(get_focus_streak(&self.db)?.total_pomodoros as i32);
        Ok(())
    }

    fn refresh_review(&self, ui: &App) -> Result<(), String> {
        let stats = get_insights(self.period, &self.db)?;
        let streak = get_focus_streak(&self.db)?;
        let mut lines = vec![
            self.format(
                "insights.completed",
                &[
                    ("count", stats.total_pomodoros.to_string()),
                    ("days", self.period.to_string()),
                ],
            ),
            self.format(
                "insights.streak",
                &[
                    ("current", streak.current_days.to_string()),
                    ("best", streak.best_days.to_string()),
                ],
            ),
            self.format(
                "insights.completion",
                &[("rate", format!("{:.0}", stats.completion_rate * 100.0))],
            ),
        ];
        let delta = i64::from(stats.total_pomodoros) - i64::from(stats.previous_period_pomodoros);
        lines.push(self.format(
            if delta > 0 {
                "insights.more"
            } else if delta < 0 {
                "insights.less"
            } else {
                "insights.same"
            },
            &[("count", delta.abs().to_string())],
        ));
        if let Some(hour) = stats.best_hour {
            lines.push(self.format(
                "insights.bestHour",
                &[
                    ("hour", format!("{hour:02}")),
                    ("end", format!("{:02}", (hour + 1) % 24)),
                ],
            ));
        }
        for (label, entries) in [
            (
                "insights.tasks",
                stats
                    .top_tasks
                    .iter()
                    .map(|row| format!("{} ({})", row.title, row.pomodoros))
                    .collect::<Vec<_>>(),
            ),
            (
                "insights.projects",
                stats
                    .top_projects
                    .iter()
                    .map(|row| format!("{} ({})", row.name, row.pomodoros))
                    .collect(),
            ),
        ] {
            if !entries.is_empty() {
                lines.push(format!("{} · {}", self.text(label), entries.join(" · ")).into());
            }
        }
        ui.set_review_lines(model(lines));
        let peak = stats
            .daily
            .iter()
            .map(|day| day.focus_minutes)
            .max()
            .unwrap_or(1)
            .max(1);
        ui.set_peak_minutes(peak as i32);
        ui.set_days(model(
            stats
                .daily
                .into_iter()
                .map(|row| DayRow {
                    date: row.date.into(),
                    minutes: row.focus_minutes as i32,
                    count: row.pomodoros as i32,
                })
                .collect(),
        ));
        Ok(())
    }

    fn task_action(&mut self, ui: &App, action: &str, index: i32) -> Result<(), String> {
        let task = option_at(&self.visible_tasks, index);
        match action {
            "new" | "edit" => {
                ui.set_selected_task(index);
                ui.set_confirm_task(false);
                ui.set_task_title(task.map_or("", |task| &task.title).into());
                ui.set_task_description(
                    task.and_then(|task| task.description.as_deref())
                        .unwrap_or("")
                        .into(),
                );
                ui.set_task_estimate(task.map_or(1, |task| task.estimated_pomodoros));
                let project = task.and_then(|task| {
                    self.projects
                        .iter()
                        .position(|p| Some(p.id) == task.project_id)
                });
                ui.set_task_project(project.map_or_else(
                    || {
                        if task.is_none() {
                            (ui.get_project_filter() - 1).max(0)
                        } else {
                            0
                        }
                    },
                    |index| index as i32 + 1,
                ));
                ui.set_editing_task(true);
                return Ok(());
            }
            "save" => {
                let value = ui.get_task_title();
                let title = value.trim();
                if title.is_empty() {
                    return Err(self.text("task-title").to_string());
                }
                let description = ui.get_task_description().trim().to_owned();
                let description = (!description.is_empty()).then_some(description);
                let project_id =
                    option_at(&self.projects, ui.get_task_project() - 1).map(|project| project.id);
                let estimated_pomodoros = ui.get_task_estimate().clamp(0, 20);
                if let Some(task) = task {
                    update_task(
                        task.id,
                        UpdateTaskInput {
                            title: title.into(),
                            description,
                            project_id,
                            estimated_pomodoros,
                        },
                        &self.db,
                    )?;
                } else {
                    create_task(
                        CreateTaskInput {
                            title: title.into(),
                            description,
                            project_id,
                            estimated_pomodoros,
                        },
                        &self.db,
                    )?;
                }
                ui.set_editing_task(false);
            }
            action => {
                let Some(task) = task else {
                    return Ok(());
                };
                match action {
                    "delete" => delete_task(task.id, &self.db)?,
                    "toggle" => {
                        toggle_task_completion(task.id, &self.db)?;
                    }
                    "plan" => {
                        plan_task(task.id, Local::now().date_naive().to_string(), &self.db)?;
                    }
                    "unplan" => {
                        unplan_task(task.id, &self.db)?;
                    }
                    "up" | "down" => {
                        let mut ids = self
                            .visible_tasks
                            .iter()
                            .map(|task| task.id)
                            .collect::<Vec<_>>();
                        let target = if action == "up" { index - 1 } else { index + 1 };
                        if option_at(&ids, target).is_some() {
                            ids.swap(index as usize, target as usize);
                            if ui.get_filter() == 3 {
                                reorder_plan(Local::now().date_naive().to_string(), ids, &self.db)?;
                            } else if ui.get_filter() == 0 {
                                reorder_tasks(ids, &self.db)?;
                            }
                        }
                    }
                    _ => return Ok(()),
                }
            }
        }
        self.refresh_tasks(ui)
    }

    fn project_action(&mut self, ui: &App, action: &str, index: i32) -> Result<(), String> {
        let project = option_at(&self.projects, index - 1);
        match action {
            "new" | "edit" => {
                ui.set_selected_project(index);
                ui.set_confirm_project(false);
                ui.set_project_name(project.map_or("", |p| &p.name).into());
                ui.set_project_color(
                    project
                        .and_then(|p| p.color.as_deref())
                        .unwrap_or("#1d3f83")
                        .into(),
                );
                ui.set_editing_project(true);
                return Ok(());
            }
            "save" => {
                let name = ui.get_project_name().trim().to_owned();
                let color = ui.get_project_color().trim().to_owned();
                if name.is_empty() || name.chars().count() > 50 {
                    return Err(self.text("project-name").to_string());
                }
                if parse_color(&color).is_none() {
                    return Err(format!("{} · #1d3f83", self.text("color")));
                }
                if let Some(project) = project {
                    update_project(project.id, name, color, &self.db)?;
                } else {
                    create_project(name, Some(color), &self.db)?;
                }
            }
            "up" | "down" => {
                let index = index - 1;
                let target = if action == "up" { index - 1 } else { index + 1 };
                let mut ids = self
                    .projects
                    .iter()
                    .map(|project| project.id)
                    .collect::<Vec<_>>();
                if option_at(&ids, index).is_some() && option_at(&ids, target).is_some() {
                    ids.swap(index as usize, target as usize);
                    reorder_projects(ids, &self.db)?;
                    self.refresh(ui)?;
                    ui.set_selected_project(target + 1);
                }
                return Ok(());
            }
            "delete" => {
                if let Some(project) = project {
                    delete_project(project.id, &self.db)?;
                }
            }
            _ => return Ok(()),
        }
        ui.set_editing_project(false);
        self.refresh(ui)
    }

    fn backup(&mut self, ui: &App, import: bool, path: &std::path::Path) -> Result<(), String> {
        if import {
            if self.timer.id.is_some() {
                return Ok(());
            }
            let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
            let data = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            import_data(&self.db, data)?;
            self.settings = get_settings(&self.db)?;
            self.timer = Countdown::new(&self.settings);
            self.task_id = None;
            self.load_settings(ui);
            self.refresh(ui)?;
        } else {
            let data = export_data(&self.db)?;
            let bytes = serde_json::to_vec_pretty(&data).map_err(|e| e.to_string())?;
            std::fs::write(path, bytes).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn paint_timer(&self, ui: &App, now: Instant) {
        let left = self.timer.left(now).as_millis().div_ceil(1000);
        let clock: SharedString = format!("{:02}:{:02}", left / 60, left % 60).into();
        if ui.get_clock() != clock {
            ui.set_clock(clock);
        }
        ui.set_progress(self.timer.progress(now));
        ui.set_active(self.timer.id.is_some());
        ui.set_running(self.timer.deadline.is_some());
        ui.set_session_kind(match self.timer.kind {
            SessionType::Work => 0,
            SessionType::ShortBreak => 1,
            SessionType::LongBreak => 2,
        });
        let task = self
            .focus_tasks
            .iter()
            .find(|task| Some(task.id) == self.task_id);
        ui.set_current_task(task.map_or_else(
            || self.text("focus.noTask"),
            |task| task.title.as_str().into(),
        ));
    }

    fn start(&mut self, now: Instant) -> Result<(), String> {
        let id = match self.timer.id {
            Some(id) => id,
            None => {
                let task = if self.timer.kind == SessionType::Work {
                    self.task_id
                } else {
                    None
                };
                create_session(
                    task,
                    self.timer.minutes(&self.settings),
                    self.timer.kind.clone(),
                    &self.db,
                )?
                .id
            }
        };
        self.timer.start(id, now);
        Ok(())
    }

    fn toggle(&mut self, ui: &App) -> Result<(), String> {
        let now = Instant::now();
        if self.timer.deadline.is_some() {
            self.timer.pause(now);
        } else {
            self.start(now)?;
        }
        self.paint_timer(ui, now);
        Ok(())
    }

    fn stop(&mut self, ui: &App) -> Result<(), String> {
        if let Some(id) = self.timer.id {
            interrupt_session(id, &self.db)?;
            self.timer.reset(self.timer.kind.clone(), &self.settings);
        }
        self.paint_timer(ui, Instant::now());
        Ok(())
    }

    fn tick(&mut self, ui: &App) {
        let now = Instant::now();
        if self.timer.deadline.is_none() {
            return;
        }
        if self.timer.left(now).is_zero()
            && let Some(id) = self.timer.id
        {
            self.timer.pause(now);
            let result = complete_session(id, &self.db).and_then(|_| {
                let was_work = self.timer.kind == SessionType::Work;
                let auto = self.timer.next(&self.settings);
                self.refresh_tasks(ui)?;
                self.refresh_review(ui)?;
                let message = self.text(if was_work {
                    "timer.completed"
                } else {
                    "chrome.notification.break"
                });
                ui.set_message(message.clone());
                let message = message.to_string();
                std::thread::spawn(move || {
                    let _ = notify_rust::Notification::new()
                        .appname("TomatoTask")
                        .summary("TomatoTask")
                        .body(message.as_str())
                        .show();
                });
                if auto {
                    self.start(now)?;
                }
                if self.settings.notification_sound != "none" {
                    self.audio.finish()?;
                }
                Ok(())
            });
            if let Err(error) = result {
                ui.set_error(error.into());
            }
        }
        self.paint_timer(ui, now);
    }
}
