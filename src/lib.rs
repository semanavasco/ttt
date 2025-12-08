pub mod app;
pub mod ui;

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, poll};
use ratatui::{Terminal, prelude::Backend};

use crate::app::{App, State};

pub fn run_app(terminal: &mut Terminal<impl Backend>, app: &mut App) -> Result<(), io::Error> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        // Check test state
        if let State::Started = app.state
            && let Some(start_time) = app.start_time
            && start_time.elapsed() >= Duration::from_secs(30)
        {
            app.state = State::Ended;
            continue;
        }

        let timeout = Duration::from_millis(100);
        if !poll(timeout)? {
            continue;
        }

        // Handle Key Events
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.state {
                State::NotStarted => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        app.start_time = Some(Instant::now());
                        app.typed_text.push(c);
                        app.state = State::Started;
                    }
                    _ => {}
                },
                State::Started => match key.code {
                    KeyCode::Char(c) => {
                        app.typed_text.push(c);
                    }
                    KeyCode::Backspace => {
                        app.typed_text.pop();
                    }
                    KeyCode::Tab => {
                        app.start_time = None;
                        app.typed_text.clear();
                        app.state = State::NotStarted;
                    }
                    KeyCode::Esc => break,
                    _ => {}
                },
                State::Ended => match key.code {
                    KeyCode::Tab => {
                        app.start_time = None;
                        app.typed_text.clear();
                        app.state = State::NotStarted;
                    }
                    KeyCode::Esc => break,
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
