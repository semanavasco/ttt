use std::time::{Duration, Instant};

pub struct State {
    pub exit: bool,
    pub menu: Menu,
    pub mode: Mode,
}

pub enum Menu {
    Home,
    Running,
    Done,
}

pub enum Mode {
    Clock {
        duration: Duration,
        start: Option<Instant>,
    },
}

impl Default for State {
    fn default() -> Self {
        State {
            exit: false,
            menu: Menu::Home,
            mode: Mode::Clock {
                duration: Duration::from_secs(30),
                start: None,
            },
        }
    }
}
