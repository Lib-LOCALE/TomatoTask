use std::time::{Duration, Instant};
use tomatotask::db::{SessionType, Settings};

pub struct Countdown {
    pub kind: SessionType,
    pub id: Option<i64>,
    pub remaining: Duration,
    pub deadline: Option<Instant>,
    pub work_sessions: u32,
    total: Duration,
}

impl Countdown {
    pub fn new(settings: &Settings) -> Self {
        let mut timer = Self {
            kind: SessionType::Work,
            id: None,
            remaining: Duration::ZERO,
            deadline: None,
            work_sessions: 0,
            total: Duration::ZERO,
        };
        timer.reset(SessionType::Work, settings);
        timer
    }

    pub fn minutes(&self, settings: &Settings) -> i32 {
        match self.kind {
            SessionType::Work => settings.work_duration,
            SessionType::ShortBreak => settings.short_break_duration,
            SessionType::LongBreak => settings.long_break_duration,
        }
        .clamp(1, 180)
    }

    pub fn reset(&mut self, kind: SessionType, settings: &Settings) {
        self.kind = kind;
        self.id = None;
        self.deadline = None;
        self.total = Duration::from_secs(self.minutes(settings) as u64 * 60);
        self.remaining = self.total;
    }

    pub fn start(&mut self, id: i64, now: Instant) {
        self.id = Some(id);
        self.deadline = Some(now + self.remaining);
    }

    pub fn pause(&mut self, now: Instant) {
        self.remaining = self.left(now);
        self.deadline = None;
    }

    pub fn left(&self, now: Instant) -> Duration {
        self.deadline
            .map_or(self.remaining, |end| end.saturating_duration_since(now))
    }

    pub fn progress(&self, now: Instant) -> f32 {
        1.0 - self.left(now).as_secs_f32() / self.total.as_secs_f32()
    }

    pub fn next(&mut self, settings: &Settings) -> bool {
        let (kind, automatic) = if self.kind == SessionType::Work {
            self.work_sessions += 1;
            let cycle = settings.pomodoros_until_long_break.max(1) as u32;
            let kind = if self.work_sessions.is_multiple_of(cycle) {
                SessionType::LongBreak
            } else {
                SessionType::ShortBreak
            };
            (kind, settings.auto_start_breaks)
        } else {
            (SessionType::Work, settings.auto_start_pomodoros)
        };
        self.reset(kind, settings);
        automatic
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tomatotask::{
        commands::get_settings,
        db::{DbConnection, migrations},
    };

    #[test]
    fn pause_resume_and_long_break() {
        let db = DbConnection::new_in_memory().unwrap();
        migrations::run_migrations(&db.get_connection().lock().unwrap()).unwrap();
        let mut prefs = get_settings(&db).unwrap();
        prefs.work_duration = 1;
        prefs.pomodoros_until_long_break = 2;
        let mut timer = Countdown::new(&prefs);
        let now = Instant::now();
        timer.start(1, now);
        timer.pause(now + Duration::from_secs(45));
        assert_eq!(timer.left(now + Duration::from_secs(70)).as_secs(), 15);
        timer.start(1, now + Duration::from_secs(70));
        assert!(timer.left(now + Duration::from_secs(85)).is_zero());
        timer.next(&prefs);
        assert_eq!(timer.kind, SessionType::ShortBreak);
        timer.next(&prefs);
        assert_eq!(timer.kind, SessionType::Work);
        timer.next(&prefs);
        assert_eq!(timer.kind, SessionType::LongBreak);
    }
}
