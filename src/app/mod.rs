//! # App Module
//!
//! The core engine of the application. This module manages the main application
//! loop, state transitions, and the orchestration of events and rendering.

pub mod events;
pub mod modes;
pub mod ui;

use std::io;

use ratatui::DefaultTerminal;
use strum::VariantNames;

use crate::{
    app::modes::{Direction, GameMode, Mode, create_mode},
    app::ui::Theme,
    config::Config,
};

/// The container for the application's state and logic.
pub struct App {
    /// Flag to signal the main loop to terminate.
    pub should_exit: bool,
    /// The high-level lifecycle state (Home, Running, etc.).
    pub state: State,
    /// The active gamemode logic, handled via dynamic dispatch.
    pub mode: Box<dyn GameMode>,
    /// The current mode configuration.
    pub mode_config: Mode,
    /// Theme for styling.
    pub theme: Theme,
    /// Currently focused option index (0 = mode selector, 1+ = mode options).
    pub focused_option: usize,
    /// Whether we're currently editing an option value.
    pub is_editing: bool,
    /// Mode name being edited in the mode selector.
    pub editing_mode: Option<String>,
}

/// Represents the lifecycle of the application.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum State {
    /// The main menu of the application.
    #[default]
    Home,
    /// A typing test session is in progress.
    Running,
    /// The test has finished, results should be displayed.
    Complete,
}

impl App {
    /// Creates a new application instance based on the provided configuration.
    pub fn from_config(config: &Config) -> Self {
        let mode_config = config.defaults.mode.clone();
        let mut mode = create_mode(&mode_config);
        mode.initialize(config);

        App {
            should_exit: false,
            state: State::default(),
            mode,
            mode_config,
            theme: Theme::default(),
            focused_option: 0,
            is_editing: false,
            editing_mode: None,
        }
    }

    /// Returns the current mode name.
    pub fn current_mode_name(&self) -> &'static str {
        self.mode_config.name()
    }

    /// Total number of options (1 for mode selector + mode-specific options).
    pub fn total_options(&self) -> usize {
        1 + self.mode.option_count()
    }

    /// Navigate to previous option.
    pub fn navigate_left(&mut self) {
        if self.focused_option > 0 {
            self.focused_option -= 1;
        } else {
            self.focused_option = self.total_options() - 1;
        }
    }

    /// Navigate to next option.
    pub fn navigate_right(&mut self) {
        if self.focused_option < self.total_options() - 1 {
            self.focused_option += 1;
        } else {
            self.focused_option = 0;
        }
    }

    /// Adjust current option value (when editing).
    pub fn adjust_current_option(&mut self, direction: Direction) {
        if self.focused_option == 0 {
            // Cycle through modes
            if let Some(ref mut mode_name) = self.editing_mode {
                let idx = Mode::VARIANTS
                    .iter()
                    .position(|&m| m == mode_name.as_str())
                    .unwrap_or(0);
                let new_idx = match direction {
                    Direction::Left => idx.checked_sub(1).unwrap_or(Mode::VARIANTS.len() - 1),
                    Direction::Right => (idx + 1) % Mode::VARIANTS.len(),
                };
                *mode_name = Mode::VARIANTS[new_idx].to_string();
            }
        } else {
            // Mode-specific option adjustment
            let option_index = self.focused_option - 1;
            self.mode.adjust_option(option_index, direction);
        }
    }

    /// Select/edit current option.
    /// Returns `Some(mode_name)` if mode should be switched.
    pub fn select_current_option(&mut self) -> Option<String> {
        if self.focused_option == 0 {
            // Mode selector
            if self.is_editing {
                // Confirm mode change
                if let Some(mode_name) = self.editing_mode.take() {
                    self.is_editing = false;
                    if mode_name != self.current_mode_name() {
                        return Some(mode_name);
                    }
                }
            } else {
                // Enter edit mode
                self.is_editing = true;
                self.editing_mode = Some(self.current_mode_name().to_string());
            }
        } else {
            // Mode-specific option
            let option_index = self.focused_option - 1;
            self.mode.select_option(option_index);
        }
        None
    }
}

/// The main application loop.
///
/// This function runs until `app.should_exit` is set to true. In each iteration:
/// 1. **Draw**: Renders the current state to the terminal using `ui::draw`.
/// 2. **Events**: Polls for user input or system events and updates the `app` state.
///
/// # Errors
/// Returns an `io::Result` if the terminal fails to draw or if event polling fails.
pub fn run(terminal: &mut DefaultTerminal, app: &mut App, config: &Config) -> io::Result<()> {
    while !app.should_exit {
        terminal.draw(|frame| ui::draw(frame, app))?;
        events::handle_events(app, config)?;
    }
    Ok(())
}
