use std::{io, time::Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

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
                    Mode::Clock {
                        typed_words,
                        target_words,
                        ..
                    } => {
                        if let Some((typed_idx, typed_word)) =
                            typed_words.iter_mut().enumerate().last()
                            && let Some(target_word) = target_words.get(typed_idx)
                            && typed_word != target_word
                            && typed_word.pop().is_none()
                        {
                            typed_words.pop();
                        }
                    }
                },
                KeyCode::Char(c) => match &mut state.mode {
                    Mode::Clock {
                        typed_words,
                        target_words,
                        ..
                    } => {
                        if c == 'h' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            if let Some((typed_idx, typed_word)) =
                                typed_words.iter_mut().enumerate().last()
                                && let Some(target_word) = target_words.get(typed_idx)
                                && typed_word != target_word
                            {
                                typed_words.pop();
                            }
                        } else if c == ' ' {
                            typed_words.push(String::new());
                        } else if let Some(word) = typed_words.last_mut() {
                            word.push(c);
                        } else {
                            typed_words.push(c.to_string());
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
