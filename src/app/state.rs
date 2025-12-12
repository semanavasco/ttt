use crate::{
    app::modes::{GameMode, Handler, clock::Clock},
    config::{Config, Mode},
};

pub struct State {
    pub exit: bool,
    pub menu: Menu,
    pub mode: Box<dyn GameMode>,
}

impl State {
    pub fn from_config(config: &Config) -> Self {
        let mut mode = match config.defaults.mode {
            Mode::Clock { duration } => Box::new(Clock::new(duration)),
        };

        mode.initialize(config);

        State {
            exit: false,
            menu: Menu::Home,
            mode,
        }
    }
}

pub enum Menu {
    Home,
    Running,
    Completed,
}
