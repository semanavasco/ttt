//! # Game Modes Module
//!
//! This module defines the traits and types for the different test modes.
//!
//! It utilizes a trait-based system where each mode must implement both
//! logic handling ([`Handler`]) and visual rendering ([`Renderer`]). These are
//! unified under the [`GameMode`] trait object used by the main application.

pub mod clock;
pub mod util;
pub mod words;

use std::time::Duration;

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{
        State,
        events::Action,
        modes::{clock::Clock, words::Words},
        ui::SELECTED_STYLE,
    },
    config::Config,
};

/// A list of mode identifiers used for configuration and CLI parsing.
pub const AVAILABLE_MODES: &[&str] = &["clock", "words"];

/// Factory function to create a new boxed [`GameMode`] based on a [`Mode`]
/// configuration.
pub fn create_mode(mode: &Mode) -> Box<dyn GameMode> {
    match mode {
        Mode::Clock { duration } => Box::new(Clock::new(*duration)),
        Mode::Words { count } => Box::new(Words::new(*count)),
    }
}

/// Configuration and serialized representation of [`GameMode`]s.
///
/// This enum defines the specific parameters for each mode (e.g., time limits or word counts).
/// It serves two primary roles:
/// 1. **In-memory State**: Determines which mode logic the [`App`][crate::app::App] should instantiate.
/// 2. **Configuration**: Defines the schema for the application's config file via Serde.
///
/// # Serialization
/// Uses an internally tagged representation (`tag = "mode"`).
///
/// ## Example:
/// ```toml
/// [defaults]
/// mode = "clock"
/// duration = 30
/// ```
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum Mode {
    Clock {
        #[serde(default = "default_clock_duration", with = "duration_as_secs")]
        duration: Duration,
    },

    Words {
        #[serde(default = "default_words_count")]
        count: usize,
    },
}

impl Mode {
    pub fn from_string(mode: &str) -> Option<Self> {
        match mode {
            "clock" => Some(Mode::Clock {
                duration: default_clock_duration(),
            }),
            "words" => Some(Mode::Words {
                count: default_words_count(),
            }),
            _ => None,
        }
    }
}

/// Logic handler for a game mode.
///
/// Responsibilities include initialization and processing user input
/// to return application-level actions.
pub trait Handler {
    /// Performs one-time setup using the application's configuration.
    fn initialize(&mut self, config: &Config);

    /// Processes keyboard input specific to the current mode.
    ///
    /// Returns an [`Action`] if the mode needs to trigger a global state change
    /// (e.g., exiting to the menu or finishing the test).
    fn handle_input(&mut self, key: KeyEvent) -> Action;
}

/// Rendering engine for a game mode.
///
/// This trait uses a "Template Method" pattern. The `render_body` and `render_footer`
/// methods provide a default dispatch mechanism that calls specific methods based
/// on the current [`State`].
pub trait Renderer {
    /// Dispatches rendering of the main content based on the current application [`State`].
    fn render_body(&self, area: Rect, buf: &mut Buffer, state: &State) {
        match state {
            State::Home => self.render_home_body(area, buf),
            State::Running => self.render_running_body(area, buf),
            State::Complete => self.render_complete_body(area, buf),
        }
    }

    /// Renders the body for the [`State::Home`] screen.
    fn render_home_body(&self, area: Rect, buf: &mut Buffer);

    /// Renders the body for the [`State::Running`] screen.
    fn render_running_body(&self, area: Rect, buf: &mut Buffer);

    /// Renders the body for the [`State::Complete`] screen.
    fn render_complete_body(&self, area: Rect, buf: &mut Buffer);

    /// Dispatches rendering of the footer based on the current application [`State`].
    fn render_footer(&self, area: Rect, buf: &mut Buffer, state: &State) {
        match state {
            State::Home => self.render_home_footer(area, buf),
            State::Running => self.render_running_footer(area, buf),
            State::Complete => self.render_complete_footer(area, buf),
        }
    }

    /// Renders the footer for the [`State::Home`] screen.
    fn render_home_footer(&self, area: Rect, buf: &mut Buffer) {
        let text = vec![
            Span::from(" Quit "),
            Span::from("(ESC)").style(SELECTED_STYLE),
            Span::from(" | Navigate Options "),
            Span::from("(<- | ->)").style(SELECTED_STYLE),
            Span::from(" | Select "),
            Span::from("(ENTER/SPACE)").style(SELECTED_STYLE),
            Span::from(" | Press any key to start your typing session... "),
        ];
        Paragraph::new(Line::from(text)).render(area, buf);
    }

    /// Renders the footer for the [`State::Running`] screen.
    fn render_running_footer(&self, area: Rect, buf: &mut Buffer) {
        let text = vec![
            Span::from(" Restart "),
            Span::from("(TAB)").style(SELECTED_STYLE),
            Span::from(" | Quit "),
            Span::from("(ESC)").style(SELECTED_STYLE),
        ];
        Paragraph::new(Line::from(text)).render(area, buf);
    }

    /// Renders the footer for the [`State::Complete`] screen.
    fn render_complete_footer(&self, area: Rect, buf: &mut Buffer) {
        let text = vec![
            Span::from(" Restart "),
            Span::from("(TAB)").style(SELECTED_STYLE),
            Span::from(" | Quit "),
            Span::from("(ESC)").style(SELECTED_STYLE),
        ];
        Paragraph::new(Line::from(text)).render(area, buf);
    }
}

/// A marker trait combining [`Handler`] and [`Renderer`].
///
/// Any type implementing both traits automatically implements `GameMode`,
/// allowing it to be used as a trait object within the main [`App`][crate::app::App].
pub trait GameMode: Handler + Renderer {}
impl<T: Handler + Renderer> GameMode for T {}

/// Statistics captured during a typing test session.
///
/// This struct provides a standardized way for game modes to report performance
/// metrics like Words Per Minute (WPM), accuracy percentage, and total elapsed time.
pub struct GameStats {
    wpm: f64,
    accuracy: f64,
    duration: f64,
}

impl GameStats {
    /// Creates a new statistics container.
    pub fn new(wpm: f64, accuracy: f64, duration: f64) -> Self {
        Self {
            wpm,
            accuracy,
            duration,
        }
    }

    /// Returns the average Words Per Minute achieved.
    pub fn wpm(&self) -> f64 {
        self.wpm
    }

    /// Returns the accuracy percentage (0.0 to 100.0).
    pub fn accuracy(&self) -> f64 {
        self.accuracy
    }

    /// Returns the total duration of the test in seconds.
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// Calculates statistics based on the test results.
    ///
    /// # Arguments
    /// * `duration` - The total time elapsed.
    /// * `typed_words` - The list of words typed by the user.
    /// * `target_words` - The list of expected words.
    pub fn calculate(duration: Duration, typed_words: &[String], target_words: &[String]) -> Self {
        let duration_mins = duration.as_secs_f64() / 60.0;

        if typed_words.is_empty() || duration_mins == 0.0 {
            return Self::new(0.0, 0.0, duration.as_secs_f64());
        }

        let mut total_chars = 0;
        let mut correct_chars = 0;

        for (i, typed) in typed_words.iter().enumerate() {
            if let Some(target) = target_words.get(i) {
                total_chars += typed.len();

                let min_len = typed.len().min(target.len());
                for j in 0..min_len {
                    if typed.chars().nth(j) == target.chars().nth(j) {
                        correct_chars += 1;
                    }
                }

                if i < typed_words.len() - 1 {
                    total_chars += 1;
                    if typed == target {
                        correct_chars += 1;
                    }
                }
            }
        }

        let accuracy = if total_chars > 0 {
            (correct_chars as f64 / total_chars as f64) * 100.0
        } else {
            0.0
        };

        let gross_wpm = (total_chars as f64 / 5.0) / duration_mins;
        let wpm = gross_wpm * (accuracy / 100.0);

        Self::new(wpm, accuracy, duration.as_secs_f64())
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Clock {
            duration: default_clock_duration(),
        }
    }
}

pub fn default_clock_duration() -> Duration {
    Duration::from_secs(30)
}

pub fn default_words_count() -> usize {
    50
}

/// [`Duration`] serializer as a simple integer representing seconds for serde.
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
