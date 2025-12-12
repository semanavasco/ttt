use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{Resource, config::Config};

pub struct State {
    pub exit: bool,
    pub menu: Menu,
    pub mode: Mode,
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
            Mode::Clock { duration, .. } => Mode::Clock {
                duration,
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

pub enum Menu {
    Home,
    Running,
    Done,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum Mode {
    Clock {
        #[serde(default = "default_clock_duration", with = "duration_as_secs")]
        duration: Duration,
        #[serde(skip_serializing, skip_deserializing)]
        start: Option<Instant>,
        #[serde(skip_serializing, skip_deserializing)]
        target_words: Vec<String>,
        #[serde(skip_serializing, skip_deserializing)]
        typed_words: Vec<String>,
    },
}

pub fn default_clock_duration() -> Duration {
    Duration::from_secs(30)
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Clock {
            duration: default_clock_duration(),
            start: None,
            target_words: Vec::new(),
            typed_words: Vec::new(),
        }
    }
}

mod duration_as_secs {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(seconds))
    }
}
