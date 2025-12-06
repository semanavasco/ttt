pub mod app;
pub mod ui;

use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{Terminal, prelude::Backend};

use crate::{app::App, ui::ui};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error> {
    while !app.should_quit {
        terminal.draw(|frame| ui(frame, app))?;

        // Handle Key Events
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => app.should_quit = true,
                _ => {}
            }
        }
    }

    Ok(())
}
