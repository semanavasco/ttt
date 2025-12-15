use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};

use crate::app::state::{Menu, State};

pub fn handle_events(state: &mut State) -> io::Result<()> {
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
                _ => {
                    state.menu = Menu::Running;
                    state.mode.handle_input(key);
                }
            },
            Menu::Running => match key.code {
                KeyCode::Esc => state.exit = true,
                KeyCode::Tab => {
                    state.mode.reset();
                    state.menu = Menu::Home;
                }
                _ => state.mode.handle_input(key),
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

pub fn handle_is_done(state: &mut State) {
    if state.mode.is_complete() {
        state.mode.handle_complete();
        state.menu = Menu::Completed;
    }
}
