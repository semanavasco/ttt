use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::state::State;

pub fn handle_events(state: &mut State) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }

        match key.code {
            KeyCode::Esc => state.exit = true,
            _ => {}
        }
    }
    Ok(())
}
