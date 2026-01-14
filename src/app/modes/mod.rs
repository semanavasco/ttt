//! # Game Modes Module
//!
//! This module defines the traits and types for the different test modes.
//!
//! Game modes provide data to the global renderer and handle mode-specific input.
//! Global controls (ESC, TAB, arrows, ...) are handled by the application layer
//! and thus not delegated to the mode.
//!
//! Check [crate::app::events] for more details.

pub mod clock;
pub mod util;
pub mod words;
pub mod zen;

use std::time::Duration;

use anyhow::Result;
use clap::Subcommand;
use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, VariantNames};

use crate::{
    app::{
        State,
        events::Action,
        modes::{clock::Clock, words::Words, zen::Zen},
        ui::char::StyledChar,
    },
    config::Config,
};

/// Factory function to create a new boxed [`GameMode`] based on a [`Mode`] configuration.
pub fn create_mode(mode: &Mode) -> Box<dyn GameMode> {
    match mode {
        Mode::Clock { duration, text } => {
            Box::new(Clock::new(Duration::from_secs(*duration), text))
        }
        Mode::Words { count, text } => Box::new(Words::new(*count, text)),
        Mode::Zen => Box::new(Zen::new()),
    }
}

/// Configuration and serialized representation of [`GameMode`]s.
///
/// This enum serves three roles:
/// 1. **CLI Subcommands**: Via `clap::Subcommand`, each variant becomes a CLI subcommand
///    with its fields as arguments (e.g., `ttt clock -d 60 -t english`).
/// 2. **Configuration Schema**: Via `serde`, defines the structure for `config.toml`.
/// 3. **Mode Factory Input**: Used by [`create_mode`] to instantiate the appropriate [`GameMode`].
///
/// # Config File Example
///
/// ```toml
/// [defaults]
/// mode = "clock"
/// text = "english"
/// duration = 30
/// ```
///
/// # CLI Usage
///
/// ```bash
/// ttt clock -d 60 -t spanish
/// ttt words -c 100
/// ```
#[derive(Serialize, Deserialize, Subcommand, Display, EnumIter, VariantNames, Clone)]
#[strum(serialize_all = "lowercase")]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum Mode {
    /// Timer-based game mode.
    Clock {
        /// The text to use for the typing test.
        #[arg(short, long, default_value_t = default_text())]
        #[serde(default = "default_text")]
        text: String,

        /// The duration of the typing test.
        #[arg(short, long, default_value_t = default_clock_duration())]
        #[serde(default = "default_clock_duration")]
        duration: u64,
    },

    /// Word-count-based game mode.
    Words {
        /// The text to use for the typing test.
        #[arg(short, long, default_value_t = default_text())]
        #[serde(default = "default_text")]
        text: String,

        /// The amount of words to type.
        #[arg(short, long, default_value_t = default_words_count())]
        #[serde(default = "default_words_count")]
        count: usize,
    },

    /// Free-typing mode with no target text.
    Zen,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Clock {
            duration: default_clock_duration(),
            text: default_text(),
        }
    }
}

impl Mode {
    /// Returns a default Mode for a given mode name string.
    pub fn default_for(name: &str) -> Self {
        match name {
            "clock" => Mode::Clock {
                duration: default_clock_duration(),
                text: default_text(),
            },
            "words" => Mode::Words {
                count: default_words_count(),
                text: default_text(),
            },
            "zen" => Mode::Zen,
            _ => Mode::default(),
        }
    }

    /// Returns the mode name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Clock { .. } => "clock",
            Mode::Words { .. } => "words",
            Mode::Zen => "zen",
        }
    }
}

pub fn default_clock_duration() -> u64 {
    30
}

pub fn default_words_count() -> usize {
    50
}

pub fn default_text() -> String {
    "english".to_string()
}

/// Represents a selectable option in the options bar.
pub struct OptionItem {
    pub label: String,
    pub is_active: bool,
    pub is_focused: bool,
    pub is_editing: bool,
}

/// Represents a group of related options.
pub struct OptionGroup {
    pub items: Vec<OptionItem>,
}

/// Direction for option adjustment.
#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

/// Hint for footer keybinds display.
pub struct FooterHint {
    pub key: &'static str,
    pub description: &'static str,
    pub state: Vec<State>,
}

impl FooterHint {
    pub fn new(key: &'static str, description: &'static str, state: Vec<State>) -> Self {
        FooterHint {
            key,
            description,
            state,
        }
    }
}

/// Logic handler for a game mode.
///
/// This trait defines how a game mode manages its internal state and responds
/// to user input. Implementations handle:
/// - **Initialization**: Loading configuration and generating initial word sets.
/// - **Input Processing**: Handling typing, backspace, and mode-specific shortcuts.
/// - **Lifecycle**: Tracking completion conditions and reset behavior.
///
/// Global controls (ESC, TAB, arrow navigation) are handled by the application
/// layer before input reaches the mode.
pub trait Handler {
    /// Performs one-time setup using the application's configuration.
    fn initialize(&mut self, config: &Config) -> Result<()>;

    /// Processes mode-specific keyboard input (typing, backspace, etc.).
    /// Global keys (ESC, TAB, arrows, ...) are handled before this is called.
    fn handle_input(&mut self, key: KeyEvent) -> Action;

    /// Resets the mode to initial state.
    fn reset(&mut self) -> Result<()>;

    /// Returns true if the mode has completed (e.g., timer expired, all words typed).
    fn is_complete(&self) -> bool;

    /// Called when transitioning to Complete state.
    fn on_complete(&mut self) {}
}

/// Data provider for the global renderer.
///
/// This trait defines how a game mode exposes its state for rendering.
/// Modes provide **data only** (options, progress, characters, stats);
/// the global renderer in [`ui`](super::ui) handles layout and styling.
pub trait Renderer {
    /// Mode-specific options to display after the mode selector.
    /// `focused_index` is None when mode selector is focused.
    fn get_options(&self, focused_index: Option<usize>) -> OptionGroup;

    /// Handle option selection (Enter/Space on a mode-specific option).
    fn select_option(&mut self, index: usize);

    /// Handle option value change while editing (arrows in edit mode).
    fn adjust_option(&mut self, index: usize, direction: Direction);

    /// Returns true if any mode option is currently being edited.
    fn is_option_editing(&self) -> bool;

    /// Number of mode-specific options (for navigation bounds).
    fn option_count(&self) -> usize;

    /// Progress text to display (e.g., "45" for timer, "23/50" for word count).
    fn get_progress(&self) -> String;

    /// Characters to display with their semantic states.
    fn get_characters(&self) -> Vec<StyledChar>;

    /// Statistics for the completion screen.
    fn get_stats(&self) -> GameStats;

    /// WPM data points for the chart: (time_seconds, wpm).
    fn get_wpm_data(&self) -> Vec<(f64, f64)>;

    /// Optional mode-specific key hints for the footer.
    fn footer_hints(&self) -> Vec<FooterHint> {
        vec![]
    }
}

/// A marker trait combining [`Handler`] and [`Renderer`].
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
    pub fn new(wpm: f64, accuracy: f64, duration: f64) -> Self {
        Self {
            wpm,
            accuracy,
            duration,
        }
    }

    pub fn wpm(&self) -> f64 {
        self.wpm
    }

    pub fn accuracy(&self) -> f64 {
        self.accuracy
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// Calculates statistics based on the test results.
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
