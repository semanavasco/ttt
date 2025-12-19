//! # Events Module
//!
//! This module handles terminal event polling and input processing.
//! It translates raw [`KeyEvent`][crossterm::event::KeyEvent]s into application-level [`Action`]s,
//! enabling state transitions and mode switching.

use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyEventKind, poll};

use crate::{
    app::{
        App, State,
        modes::{Mode, create_mode},
    },
    config::Config,
};

/// Defines the intent of an input event after being processed by a mode.
///
/// This allows individual game modes to communicate requests for global
/// changes back to the main application loop.
pub enum Action {
    /// The input was consumed or ignored; no global state change is needed.
    None,
    /// Request to completely replace the current game mode (e.g., from 'Clock' to 'Words').
    SwitchMode(String),
    /// Request to transition the application's lifecycle state (e.g., from [`State::Home`] to [`State::Running`]).
    SwitchState(State),
    /// Request to quit the application.
    Quit,
}

/// Polls for and processes terminal events.
///
/// This function waits for up to 100ms for an event. If a key event occurs:
/// 1. It filters out [`KeyEventKind::Release`] to avoid double-processing.
/// 2. It passes the key to the current active mode.
/// 3. It executes any [`Action`] returned by the mode.
///
/// # Errors
/// Returns an [`io::Error`] if polling or reading the terminal event stream fails.
pub fn handle_events(app: &mut App, config: &Config) -> io::Result<()> {
    if !poll(Duration::from_millis(100))? {
        return Ok(());
    }

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }

        match app.mode.handle_input(key) {
            Action::None => {}
            Action::SwitchMode(mode_str) => switch_mode(app, &mode_str, config),
            Action::SwitchState(state) => app.state = state,
            Action::Quit => app.should_exit = true,
        }
    }

    Ok(())
}

/// Replaces the current active mode with a new one based on a string identifier.
///
/// If the `mode_str` is valid, the new mode is created, initialized with the
/// current config, and swapped into the `app` instance.
fn switch_mode(app: &mut App, mode_str: &str, config: &Config) {
    if let Some(mode) = Mode::from_string(mode_str) {
        app.mode = create_mode(&mode);
        app.mode.initialize(config);
    }
}
