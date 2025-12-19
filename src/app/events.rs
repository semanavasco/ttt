use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};

use crate::{
    app::{
        modes::{Mode, ModeAction, create_mode},
        state::{Menu, State},
    },
    config::Config,
};

pub fn handle_events(state: &mut State, config: &Config) -> io::Result<()> {
    if !poll(Duration::from_millis(100))? {
        return Ok(());
    }

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }

        match state.menu {
            Menu::Home => match key.code {
                KeyCode::Esc => state.exit = true,
                KeyCode::Char(_) => {
                    state.menu = Menu::Running;
                    match state.mode.handle_input(key) {
                        ModeAction::SwitchMode(mode_str) => switch_mode(state, &mode_str, config),
                        ModeAction::None => {}
                    }
                }
                _ => match state.mode.handle_input(key) {
                    ModeAction::SwitchMode(mode_str) => switch_mode(state, &mode_str, config),
                    ModeAction::None => {}
                },
            },
            Menu::Running => match key.code {
                KeyCode::Esc => state.exit = true,
                KeyCode::Tab => {
                    state.mode.reset();
                    state.menu = Menu::Home;
                }
                _ => {
                    state.mode.handle_input(key);
                }
            },
            Menu::Completed => match key.code {
                KeyCode::Esc => state.exit = true,
                KeyCode::Tab => {
                    state.mode.reset();
                    state.menu = Menu::Home;
                }
                _ => {}
            },
        }
    }

    Ok(())
}

fn switch_mode(state: &mut State, mode_str: &str, config: &Config) {
    if let Some(mode_enum) = Mode::from_string(mode_str) {
        state.mode = create_mode(&mode_enum);
        state.mode.initialize(config);
    }
}

pub fn handle_is_done(state: &mut State) {
    if state.mode.is_complete() {
        state.mode.handle_complete();
        state.menu = Menu::Completed;
    }
}
