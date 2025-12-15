pub mod clock;
pub mod util;

use std::time::Duration;

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};
use serde::{Deserialize, Serialize};

use crate::{app::modes::clock::Clock, config::Config};

pub trait Handler {
    fn initialize(&mut self, config: &Config);
    fn handle_input(&mut self, key: KeyEvent);
    fn is_complete(&self) -> bool;
    fn handle_complete(&mut self);
    fn get_stats(&self) -> GameStats;
    fn reset(&mut self);
}

pub trait Renderer {
    fn render_home(&self, area: Rect, buf: &mut Buffer);
    fn render_running(&self, area: Rect, buf: &mut Buffer);
    fn render_complete(&self, area: Rect, buf: &mut Buffer);
}

pub trait GameMode: Handler + Renderer {}
impl<T: Handler + Renderer> GameMode for T {}

pub struct GameStats {
    wpm: f64,
    accuracy: f64,
    duration: f64,
}

impl GameStats {
    fn wpm(&self) -> f64 {
        self.wpm
    }

    fn accuracy(&self) -> f64 {
        self.accuracy
    }

    fn duration(&self) -> f64 {
        self.duration
    }
}

pub const AVAILABLE_MODES: &[&str] = &["clock"];

pub fn create_mode(mode: &Mode) -> Box<dyn GameMode> {
    match mode {
        Mode::Clock { duration } => Box::new(Clock::new(*duration)),
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum Mode {
    Clock {
        #[serde(default = "default_clock_duration", with = "duration_as_secs")]
        duration: Duration,
    },
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Clock {
            duration: default_clock_duration(),
        }
    }
}

impl Mode {
    pub fn from_string(mode: &str) -> Option<Self> {
        match mode {
            "clock" => Some(Mode::Clock {
                duration: default_clock_duration(),
            }),
            _ => None,
        }
    }
}

pub fn default_clock_duration() -> Duration {
    Duration::from_secs(30)
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
