use serde_json::Value;
use slint::Image;
use std::error::Error;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

use crate::locale;

const OPEN: &str = "tomatotask.open";
const TOGGLE: &str = "tomatotask.toggle";
const QUIT: &str = "tomatotask.quit";

pub enum Action {
    Open,
    Toggle,
    Quit,
}

pub struct Tray {
    _icon: TrayIcon,
    open: MenuItem,
    toggle: MenuItem,
    quit: MenuItem,
}

impl Tray {
    pub fn new(locale: &Value, welcoming: bool) -> Result<Self, Box<dyn Error>> {
        let open = MenuItem::with_id(OPEN, locale::text(locale, "tray-open"), true, None);
        let toggle = MenuItem::with_id(
            TOGGLE,
            locale::text(locale, "tray-toggle"),
            !welcoming,
            None,
        );
        let quit = MenuItem::with_id(QUIT, locale::text(locale, "tray-quit"), true, None);
        let menu = Menu::with_items(&[&open, &toggle, &PredefinedMenuItem::separator(), &quit])?;
        let image = Image::load_from_data(include_bytes!("../assets/icon.png"), Some("png"))?;
        let pixels = image.to_rgba8().ok_or("Could not decode the tray icon")?;
        let icon = Icon::from_rgba(pixels.as_bytes().to_vec(), pixels.width(), pixels.height())?;
        let icon = TrayIconBuilder::new()
            .with_title("TomatoTask")
            .with_tooltip("TomatoTask")
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .build()?;
        Ok(Self {
            _icon: icon,
            open,
            toggle,
            quit,
        })
    }

    pub fn localize(&self, locale: &Value) {
        self.open.set_text(locale::text(locale, "tray-open"));
        self.toggle.set_text(locale::text(locale, "tray-toggle"));
        self.quit.set_text(locale::text(locale, "tray-quit"));
    }

    pub fn enable_timer(&self) {
        self.toggle.set_enabled(true);
    }
}

pub fn actions() -> impl Iterator<Item = Action> {
    MenuEvent::receiver()
        .try_iter()
        .filter_map(|event| match event.id.as_ref() {
            OPEN => Some(Action::Open),
            TOGGLE => Some(Action::Toggle),
            QUIT => Some(Action::Quit),
            _ => None,
        })
}
