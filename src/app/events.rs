//! # Events Module
//!
//! This module handles terminal event polling and input processing.
//! Global controls (ESC, TAB, arrows...) are handled here, with mode-specific
//! input delegated to the active game mode.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, poll};

use crate::{
    app::{
        App, State,
        modes::{Direction, Mode, create_mode},
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
    SwitchMode(Mode),
    /// Request to transition the application's lifecycle state (e.g., from [`State::Home`] to [`State::Running`]).
    SwitchState(State),
    /// Request to quit the application.
    Quit,
}

/// Polls for and processes terminal events.
pub fn handle_events(app: &mut App, config: &Config) -> Result<()> {
    if !poll(Duration::from_millis(100))? {
        return Ok(());
    }

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }

        let action = match app.state {
            State::Home => handle_home_input(app, key)?,
            State::Running => handle_running_input(app, key)?,
            State::Complete => handle_complete_input(app, key)?,
        };

        execute_action(app, action, config)?;
    }

    Ok(())
}

/// Handles input on the Home screen (options navigation, mode selection, typing start).
fn handle_home_input(app: &mut App, key: KeyEvent) -> Result<Action> {
    // Check if mode is editing a custom option
    let mode_editing = app.mode.is_option_editing();

    let action = match key.code {
        KeyCode::Esc => Action::Quit,

        KeyCode::Left | KeyCode::Down => {
            if app.is_editing || mode_editing {
                app.adjust_current_option(Direction::Left)?;
            } else {
                app.navigate_left();
            }
            Action::None
        }

        KeyCode::Right | KeyCode::Up => {
            if app.is_editing || mode_editing {
                app.adjust_current_option(Direction::Right)?;
            } else {
                app.navigate_right();
            }
            Action::None
        }

        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(mode_name) = app.select_current_option()? {
                Action::SwitchMode(Mode::default_for(&mode_name))
            } else {
                Action::None
            }
        }

        // Any typing character starts the game
        KeyCode::Char(_) => {
            let action = app.mode.handle_input(key);
            if matches!(action, Action::None) {
                Action::SwitchState(State::Running)
            } else {
                action
            }
        }

        _ => Action::None,
    };

    Ok(action)
}

/// Handles input during an active typing session.
///
/// **Globally handled keys:**
/// - `ESC`: Quit the application.
/// - `TAB`: Reset the mode and return to Home.
///
/// **Delegated to game mode:** All other keys (typing, backspace, etc.).
fn handle_running_input(app: &mut App, key: KeyEvent) -> Result<Action> {
    match key.code {
        KeyCode::Esc => Ok(Action::Quit),
        KeyCode::Tab => {
            app.mode.reset()?;
            app.focused_option = 0;
            app.is_editing = false;
            Ok(Action::SwitchState(State::Home))
        }
        _ => {
            let action = app.mode.handle_input(key);

            // Check for completion after input
            if app.mode.is_complete() {
                app.mode.on_complete();
                Ok(Action::SwitchState(State::Complete))
            } else {
                Ok(action)
            }
        }
    }
}

/// Handles input on the completion screen (restart or quit only).
fn handle_complete_input(app: &mut App, key: KeyEvent) -> Result<Action> {
    match key.code {
        KeyCode::Esc => Ok(Action::Quit),
        KeyCode::Tab => {
            app.mode.reset()?;
            app.focused_option = 0;
            app.is_editing = false;
            Ok(Action::SwitchState(State::Home))
        }
        _ => Ok(Action::None),
    }
}

/// Executes the given action, updating application state accordingly.
fn execute_action(app: &mut App, action: Action, config: &Config) -> Result<()> {
    match action {
        Action::None => {}
        Action::SwitchMode(mode) => {
            app.mode_config = mode.clone();
            let mut new_mode = create_mode(&mode);
            new_mode.initialize(config)?;
            app.mode = new_mode;
            app.focused_option = 0;
            app.is_editing = false;
            app.editing_mode = None;
        }
        Action::SwitchState(state) => app.state = state,
        Action::Quit => app.should_exit = true,
    }

    Ok(())
}
