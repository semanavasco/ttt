use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::seq::SliceRandom;

use crate::{
    Resource,
    app::{
        events::Action,
        modes::{
            Direction, GameStats, Handler, Mode, OptionGroup, OptionItem, Renderer,
            util::build_styled_chars,
        },
        ui::char::StyledChar,
    },
    config::Config,
};

const DURATIONS: [u64; 4] = [15, 30, 60, 120];

pub struct Clock {
    duration: Duration,
    custom_duration: u64,
    is_editing_custom: bool,
    start: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
    timestamps: Vec<(usize, Instant)>,
    text: String,
}

impl Clock {
    pub fn new(duration: Duration, text: &str) -> Self {
        let duration_secs = duration.as_secs();
        let custom_duration = if DURATIONS.contains(&duration_secs) {
            30
        } else {
            duration_secs
        };

        Self {
            duration,
            custom_duration,
            is_editing_custom: false,
            start: None,
            target_words: Vec::new(),
            typed_words: Vec::new(),
            timestamps: Vec::new(),
            text: text.to_owned(),
        }
    }

    fn generate_words(&mut self) -> Result<()> {
        let bytes = Resource::get_text(&self.text)
            .context(format!("Couldn't find \"{}\" text", &self.text))?;

        let text: Vec<&str> = std::str::from_utf8(&bytes)
            .context("Text contains non-utf8 characters")?
            .lines()
            .collect();

        let mut words: Vec<String> = text
            .iter()
            .cycle()
            .take(100)
            .map(|s| s.to_string())
            .collect();

        let mut rng = rand::rng();
        words.shuffle(&mut rng);

        self.target_words = words;
        Ok(())
    }
}

impl Handler for Clock {
    fn initialize(&mut self, config: &Config) -> Result<()> {
        self.typed_words.clear();
        self.start = None;
        if let Mode::Clock { duration, text } = &config.defaults.mode {
            self.duration = Duration::from_secs(*duration);
            if !DURATIONS.contains(duration) {
                self.custom_duration = *duration;
            }
            self.text = text.clone();
        }
        self.generate_words()?;
        Ok(())
    }

    fn handle_input(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char(c) => {
                if self.start.is_none() {
                    self.start = Some(Instant::now());
                }

                if c == 'h' && key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Clear current word
                    if let Some((typed_idx, typed_word)) =
                        self.typed_words.iter_mut().enumerate().last()
                        && let Some(target_word) = self.target_words.get(typed_idx)
                        && typed_word != target_word
                    {
                        if typed_word.is_empty() {
                            self.typed_words.pop();
                        } else {
                            typed_word.clear();
                        }
                    }
                } else if c == ' ' {
                    // Move to next word
                    if let Some(last) = self.typed_words.last()
                        && !last.is_empty()
                    {
                        self.timestamps
                            .push((self.typed_words.len(), Instant::now()));
                        self.typed_words.push(String::new());
                    }
                } else if let Some(word) = self.typed_words.last_mut() {
                    word.push(c);
                } else {
                    self.typed_words.push(c.to_string());
                }
            }
            KeyCode::Backspace => {
                if let Some((typed_idx, typed_word)) =
                    self.typed_words.iter_mut().enumerate().last()
                    && let Some(target_word) = self.target_words.get(typed_idx)
                    && typed_word != target_word
                    && typed_word.pop().is_none()
                {
                    self.typed_words.pop();
                }
            }
            _ => {}
        }

        Action::None
    }

    fn reset(&mut self) -> Result<()> {
        self.generate_words()?;
        self.start = None;
        self.typed_words.clear();
        self.timestamps.clear();
        Ok(())
    }

    fn is_complete(&self) -> bool {
        self.start
            .map(|s| s.elapsed() >= self.duration)
            .unwrap_or(false)
    }
}

impl Renderer for Clock {
    fn get_options(&self, focused_index: Option<usize>) -> OptionGroup {
        let current = self.duration.as_secs();

        let mut items: Vec<OptionItem> = DURATIONS
            .iter()
            .enumerate()
            .map(|(i, &d)| OptionItem {
                label: format!("{}s", d),
                is_active: current == d,
                is_focused: focused_index == Some(i),
                is_editing: false,
            })
            .collect();

        // Custom option
        items.push(OptionItem {
            label: format!("󱁤 {}", self.custom_duration),
            is_active: !DURATIONS.contains(&current),
            is_focused: focused_index == Some(4),
            is_editing: self.is_editing_custom,
        });

        OptionGroup { items }
    }

    fn select_option(&mut self, index: usize) {
        if index < 4 {
            self.duration = Duration::from_secs(DURATIONS[index]);
            self.is_editing_custom = false;
        } else {
            // Custom - toggle edit mode
            if self.is_editing_custom {
                self.is_editing_custom = false;
            } else {
                self.is_editing_custom = true;
                self.duration = Duration::from_secs(self.custom_duration);
            }
        }
    }

    fn adjust_option(&mut self, index: usize, direction: Direction) {
        if index == 4 {
            match direction {
                Direction::Left => {
                    self.custom_duration = self.custom_duration.saturating_sub(5).max(5);
                }
                Direction::Right => {
                    self.custom_duration += 5;
                }
            }
            self.duration = Duration::from_secs(self.custom_duration);
        }
    }

    fn is_option_editing(&self) -> bool {
        self.is_editing_custom
    }

    fn option_count(&self) -> usize {
        5
    }

    fn get_progress(&self) -> String {
        match self.start {
            Some(start) => {
                let remaining = self.duration.saturating_sub(start.elapsed());
                format!("{}", remaining.as_secs())
            }
            None => String::new(),
        }
    }

    fn get_characters(&self) -> Vec<StyledChar> {
        build_styled_chars(&self.target_words, &self.typed_words)
    }

    fn get_stats(&self) -> GameStats {
        GameStats::calculate(self.duration, &self.typed_words, &self.target_words)
    }

    fn get_wpm_data(&self) -> Vec<(f64, f64)> {
        let mut data = vec![(0.0, 0.0)];

        if let Some(start) = &self.start {
            for (words, ts) in &self.timestamps {
                let duration = ts.duration_since(*start);
                let typed_words = &self.typed_words[..*words];
                let target_words = &self.target_words[..*words];
                let stats = GameStats::calculate(duration, typed_words, target_words);
                data.push((duration.as_secs_f64(), stats.wpm()));
            }
        }

        data
    }
}
