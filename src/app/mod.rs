//! # App Module
//!
//! The core engine of the application. This module manages the main application
//! loop, state transitions, and the orchestration of events and rendering.

pub mod events;
pub mod modes;
pub mod ui;

use std::io;

use ratatui::DefaultTerminal;

use crate::{
    app::modes::{GameMode, create_mode},
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
}

/// Represents the lifecycle of the application.
#[derive(Default)]
pub enum State {
    /// The main menu,
    #[default]
    Home,
    /// A typing test session is in progress.
    Running,
    /// The test has finished, results should be displayed.
    Complete,
}

impl App {
    /// Creates a new application instance based on the provided configuration.
    ///
    /// This initializes the default game mode and applies configuration settings
    /// before the first render.
    pub fn from_config(config: &Config) -> Self {
        let mut mode = create_mode(&config.defaults.mode);

        mode.initialize(config);

        App {
            should_exit: false,
            state: State::default(),
            mode,
        }
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
