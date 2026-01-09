use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{
        State,
        events::Action,
        modes::{Direction, FooterHint, GameStats, Handler, OptionGroup, Renderer},
        ui::{CharState, StyledChar},
    },
    config::Config,
};

pub struct Zen {
    start: Option<Instant>,
    end: Option<Instant>,
    typed_chars: Vec<char>,
    timestamps: Vec<(usize, Instant)>,
}

impl Default for Zen {
    fn default() -> Self {
        Self::new()
    }
}

impl Zen {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            typed_chars: Vec::new(),
            timestamps: Vec::new(),
        }
    }

    fn word_count(&self) -> usize {
        if self.typed_chars.is_empty() {
            return 0;
        }
        // Count words by spaces + 1 for current word if not empty
        self.typed_chars.iter().filter(|&&c| c == ' ').count()
            + if self.typed_chars.last() != Some(&' ') {
                1
            } else {
                0
            }
    }
}

impl Handler for Zen {
    fn initialize(&mut self, _config: &Config) {
        self.start = None;
        self.end = None;
        self.typed_chars.clear();
        self.timestamps.clear();
    }

    fn handle_input(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Enter => {
                // Enter completes the session
                if self.start.is_some() && !self.typed_chars.is_empty() {
                    self.end = Some(Instant::now());
                }
                Action::None
            }
            KeyCode::Char(c) => {
                if self.start.is_none() {
                    self.start = Some(Instant::now());
                }

                self.typed_chars.push(c);

                // Record timestamp on space (word completed)
                if c == ' ' {
                    self.timestamps
                        .push((self.typed_chars.len(), Instant::now()));
                }

                Action::None
            }
            KeyCode::Backspace => {
                self.typed_chars.pop();
                Action::None
            }
            _ => Action::None,
        }
    }

    fn reset(&mut self) {
        self.start = None;
        self.end = None;
        self.typed_chars.clear();
        self.timestamps.clear();
    }

    fn is_complete(&self) -> bool {
        self.end.is_some()
    }

    fn on_complete(&mut self) {
        if self.end.is_none() {
            self.end = Some(Instant::now());
        }
    }
}

impl Renderer for Zen {
    fn get_options(&self, _focused_index: Option<usize>) -> OptionGroup {
        // No options for Zen mode
        OptionGroup { items: vec![] }
    }

    fn select_option(&mut self, _index: usize) {}

    fn adjust_option(&mut self, _index: usize, _direction: Direction) {}

    fn is_option_editing(&self) -> bool {
        false
    }

    fn option_count(&self) -> usize {
        0
    }

    fn get_progress(&self) -> String {
        if self.start.is_some() {
            format!("{} words", self.word_count())
        } else {
            String::new()
        }
    }

    fn get_characters(&self) -> Vec<StyledChar> {
        let mut chars: Vec<StyledChar> = self
            .typed_chars
            .iter()
            .map(|&c| StyledChar::new(c, CharState::Correct))
            .collect();

        // Add cursor at the end
        chars.push(StyledChar::new(' ', CharState::Cursor));

        chars
    }

    fn get_stats(&self) -> GameStats {
        let duration = match (self.start, self.end) {
            (Some(start), Some(end)) => end.duration_since(start),
            (Some(start), None) => start.elapsed(),
            _ => std::time::Duration::ZERO,
        };

        let duration_mins = duration.as_secs_f64() / 60.0;
        let char_count = self.typed_chars.len();

        let wpm = if duration_mins > 0.0 {
            (char_count as f64 / 5.0) / duration_mins
        } else {
            0.0
        };

        GameStats::new(wpm, 100.0, duration.as_secs_f64())
    }

    fn get_wpm_data(&self) -> Vec<(f64, f64)> {
        let mut data = vec![(0.0, 0.0)];

        if let Some(start) = self.start {
            for &(char_count, ts) in &self.timestamps {
                let duration = ts.duration_since(start);
                let duration_mins = duration.as_secs_f64() / 60.0;

                let wpm = if duration_mins > 0.0 {
                    (char_count as f64 / 5.0) / duration_mins
                } else {
                    0.0
                };

                data.push((duration.as_secs_f64(), wpm));
            }
        }

        data
    }

    fn footer_hints(&self) -> Vec<FooterHint> {
        vec![FooterHint::new("ENTER", "Finish", vec![State::Running])]
    }
}
