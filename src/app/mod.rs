pub mod events;
pub mod state;
pub mod ui;

use std::io;

use ratatui::DefaultTerminal;

use crate::app::state::{Menu, State};

pub fn run(terminal: &mut DefaultTerminal, state: &mut State) -> io::Result<()> {
    while !state.exit {
        terminal.draw(|frame| ui::draw(frame, state))?;
        if let Menu::Running = state.menu {
            events::handle_is_done(state);
        };
        events::handle_events(state)?;
    }
    Ok(())
}
