use std::time::{Duration, Instant};

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

const WORD_COUNTS: [usize; 4] = [25, 50, 75, 100];

pub struct Words {
    words: usize,
    custom_words: usize,
    is_editing_custom: bool,
    start: Option<Instant>,
    end: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
    timestamps: Vec<(usize, Instant)>,
    dictionary: Vec<String>,
    text: String,
}

impl Words {
    pub fn new(words: usize, text: &str) -> Self {
        let custom_words = if WORD_COUNTS.contains(&words) {
            50
        } else {
            words
        };

        Self {
            words,
            custom_words,
            is_editing_custom: false,
            start: None,
            end: None,
            target_words: Vec::new(),
            typed_words: Vec::new(),
            timestamps: Vec::new(),
            dictionary: Vec::new(),
            text: text.to_owned(),
        }
    }

    fn generate_words(&mut self) {
        let mut rng = rand::rng();
        self.dictionary.shuffle(&mut rng);

        self.target_words = self
            .dictionary
            .iter()
            .cycle()
            .take(self.words)
            .map(ToString::to_string)
            .collect();
    }

    fn check_complete(&self) -> bool {
        self.typed_words.len() == self.target_words.len()
            && self
                .typed_words
                .last()
                .is_some_and(|w| w.len() == self.target_words.last().map_or(5, |w| w.len()))
            || self.typed_words.len() > self.target_words.len()
    }
}

impl Handler for Words {
    fn initialize(&mut self, config: &Config) {
        self.start = None;
        self.end = None;
        self.typed_words.clear();

        if let Mode::Words { count, text } = &config.defaults.mode {
            self.words = *count;
            if !WORD_COUNTS.contains(count) {
                self.custom_words = *count;
            }
            self.text = text.clone();
        }

        let bytes = Resource::get_text(&self.text)
            .unwrap_or_else(|_| panic!("Couldn't find \"{}\" text", &self.text));

        self.dictionary = std::str::from_utf8(&bytes)
            .expect("Text contains non-utf8 characters")
            .lines()
            .map(ToString::to_string)
            .collect();

        self.generate_words();
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

    fn reset(&mut self) {
        self.generate_words();
        self.start = None;
        self.end = None;
        self.typed_words.clear();
        self.timestamps.clear();
    }

    fn is_complete(&self) -> bool {
        self.check_complete()
    }

    fn on_complete(&mut self) {
        if self.end.is_none() {
            self.end = Some(Instant::now());
        }
    }
}

impl Renderer for Words {
    fn get_options(&self, focused_index: Option<usize>) -> OptionGroup {
        let current = self.words;

        let mut items: Vec<OptionItem> = WORD_COUNTS
            .iter()
            .enumerate()
            .map(|(i, &c)| OptionItem {
                label: format!("{}", c),
                is_active: current == c,
                is_focused: focused_index == Some(i),
                is_editing: false,
            })
            .collect();

        // Custom option
        items.push(OptionItem {
            label: format!("󱁤 {}", self.custom_words),
            is_active: !WORD_COUNTS.contains(&current),
            is_focused: focused_index == Some(4),
            is_editing: self.is_editing_custom,
        });

        OptionGroup { items }
    }

    fn select_option(&mut self, index: usize) {
        if index < 4 {
            self.words = WORD_COUNTS[index];
            self.is_editing_custom = false;
            self.reset();
        } else {
            // Custom - toggle edit mode
            if self.is_editing_custom {
                self.is_editing_custom = false;
            } else {
                self.is_editing_custom = true;
                self.words = self.custom_words;
                self.reset();
            }
        }
    }

    fn adjust_option(&mut self, index: usize, direction: Direction) {
        if index == 4 {
            match direction {
                Direction::Left => {
                    self.custom_words = self.custom_words.saturating_sub(5).max(10);
                }
                Direction::Right => {
                    self.custom_words += 5;
                }
            }
            self.words = self.custom_words;
            self.reset();
        }
    }

    fn is_option_editing(&self) -> bool {
        self.is_editing_custom
    }

    fn option_count(&self) -> usize {
        5
    }

    fn get_progress(&self) -> String {
        if self.start.is_some() {
            format!("{}/{}", self.typed_words.len(), self.words)
        } else {
            String::new()
        }
    }

    fn get_characters(&self) -> Vec<StyledChar> {
        build_styled_chars(&self.target_words, &self.typed_words)
    }

    fn get_stats(&self) -> GameStats {
        let duration = if let (Some(start), Some(end)) = (self.start, self.end) {
            end.duration_since(start)
        } else {
            Duration::from_secs(0)
        };

        GameStats::calculate(duration, &self.typed_words, &self.target_words)
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
