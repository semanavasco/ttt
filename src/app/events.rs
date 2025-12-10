use std::{io, time::Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::state::{Menu, Mode, State};

pub fn handle_events(state: &mut State) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }

        match &mut state.menu {
            Menu::Home => match key.code {
                KeyCode::Esc => state.exit = true,
                KeyCode::Char(c) => {
                    state.menu = Menu::Running;

                    match &mut state.mode {
                        Mode::Clock {
                            typed_words, start, ..
                        } => {
                            typed_words.push(c.to_string());
                            *start = Some(Instant::now());
                        }
                    }
                }
                _ => {}
            },
            Menu::Running => match key.code {
                KeyCode::Esc => state.exit = true,
                KeyCode::Backspace => match &mut state.mode {
                    Mode::Clock { typed_words, .. } => {
                        if let Some(last_word) = typed_words.last_mut() {
                            if last_word.pop() == None {
                                typed_words.pop();
                            }
                        }
                    }
                },
                KeyCode::Char(c) => match &mut state.mode {
                    Mode::Clock { typed_words, .. } => {
                        if c == ' ' {
                            typed_words.push(String::new());
                        } else {
                            if let Some(word) = typed_words.last_mut() {
                                word.push(c);
                            } else {
                                typed_words.push(c.to_string());
                            }
                        }
                    }
                },
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
