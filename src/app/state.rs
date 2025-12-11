use std::time::{Duration, Instant};

use rand::seq::SliceRandom;

use crate::{
    Resource,
    config::{Config, DefaultMode},
};

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
        target_words: Vec<String>,
        typed_words: Vec<String>,
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
                target_words: Vec::new(),
                typed_words: Vec::new(),
            },
        }
    }
}

impl State {
    pub fn from_config(config: &Config) -> Self {
        let mode = match config.defaults.mode {
            DefaultMode::Clock { duration } => Mode::Clock {
                duration: Duration::from_secs(duration),
                start: None,
                target_words: {
                    let bytes = Resource::get(&config.defaults.text)
                        .map(|f| f.data.into_owned())
                        .unwrap();

                    let text: Vec<&str> = str::from_utf8(&bytes).unwrap().lines().collect();

                    let mut rng = rand::rng();

                    let mut words: Vec<String> = text
                        .iter()
                        .cycle()
                        .take(config.defaults.words as usize)
                        .map(ToOwned::to_owned)
                        .map(String::from)
                        .collect();

                    words.shuffle(&mut rng);

                    words
                },
                typed_words: Vec::new(),
            },
        };

        State {
            exit: false,
            menu: Menu::Home,
            mode,
        }
    }
}
