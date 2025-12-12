use crate::{
    app::modes::{GameMode, create_mode},
    config::Config,
};

pub struct State {
    pub exit: bool,
    pub menu: Menu,
    pub mode: Box<dyn GameMode>,
}

impl State {
    pub fn from_config(config: &Config) -> Self {
        let mut mode = create_mode(&config.defaults.mode);

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
