pub mod events;
pub mod state;
pub mod ui;

use std::io;

use ratatui::DefaultTerminal;

use crate::app::state::State;

pub fn run(terminal: &mut DefaultTerminal, state: &mut State) -> io::Result<()> {
    while !state.exit {
        terminal.draw(|frame| ui::draw(frame, state))?;
        events::handle_events(state)?;
    }
    Ok(())
}
