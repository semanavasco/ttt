use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Dataset, GraphType, Paragraph, Widget, Wrap},
};
use strum::VariantNames;

use crate::{
    Resource,
    app::{
        State,
        modes::{
            Action, GameStats, Handler, Mode, Renderer,
            util::{get_typing_spans, render_wpm_chart},
        },
        ui::SELECTED_STYLE,
    },
    config::Config,
};

const WORD_COUNTS: [usize; 4] = [25, 50, 75, 100];

enum Options {
    Mode(String),
    WordCount(usize),
}

impl Default for Options {
    fn default() -> Self {
        Options::Mode("words".to_string())
    }
}

pub struct Words {
    selected_option: Options,
    words: usize,
    custom_words: usize,
    is_editing: Option<Options>,
    start: Option<Instant>,
    end: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
    timestamps: Vec<(usize, Instant)>,
    dictionary: Vec<String>,
    text: String,
}

impl Words {
    pub fn new(words: usize, text: &String) -> Self {
        let custom_words = if WORD_COUNTS.contains(&words) {
            50
        } else {
            words
        };

        Self {
            selected_option: Options::default(),
            words,
            custom_words,
            is_editing: None,
            start: None,
            end: None,
            target_words: Vec::new(),
            typed_words: Vec::new(),
            timestamps: Vec::new(),
            dictionary: Vec::new(),
            text: text.clone(),
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

    fn get_stats(&self) -> GameStats {
        let duration = if let Some(start) = self.start
            && let Some(end) = self.end
        {
            end.duration_since(start)
        } else {
            Duration::from_secs(0)
        };

        GameStats::calculate(duration, &self.typed_words, &self.target_words)
    }

    fn reset(&mut self) {
        self.generate_words();
        self.start = None;
        self.end = None;
        self.typed_words.clear();
        self.timestamps.clear();
    }

    fn is_complete(&self) -> bool {
        self.typed_words.len() == self.target_words.len()
            && self
                .typed_words
                .last()
                .is_some_and(|w| w.len() == self.target_words.last().map_or_else(|| 5, |w| w.len()))
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

        self.dictionary = str::from_utf8(&bytes)
            .expect("Text contains non-utf8 characters")
            .lines()
            .map(ToString::to_string)
            .collect();

        self.generate_words();
    }

    fn handle_input(&mut self, key: KeyEvent) -> Action {
        if key.code == KeyCode::Tab {
            self.reset();
            return Action::SwitchState(State::Home);
        }

        if key.code == KeyCode::Esc {
            return Action::Quit;
        }

        if self.is_complete() {
            if self.end.is_none() {
                self.end = Some(Instant::now());
            }
            return Action::SwitchState(State::Complete);
        }

        if let Some(editing) = &mut self.is_editing {
            match editing {
                Options::Mode(mode) => match key.code {
                    KeyCode::Left | KeyCode::Down => {
                        let current_idx = Mode::VARIANTS
                            .iter()
                            .position(|&m| m == mode.as_str())
                            .unwrap_or(0);
                        let prev_idx = current_idx
                            .checked_sub(1)
                            .unwrap_or(Mode::VARIANTS.len() - 1);
                        *mode = Mode::VARIANTS[prev_idx].to_string();
                    }
                    KeyCode::Right | KeyCode::Up => {
                        let current_idx = Mode::VARIANTS
                            .iter()
                            .position(|&m| m == mode.as_str())
                            .unwrap_or(0);
                        let next_idx = (current_idx + 1) % Mode::VARIANTS.len();
                        *mode = Mode::VARIANTS[next_idx].to_string();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        let new_mode = mode.clone();
                        self.is_editing = None;
                        if new_mode != "words" {
                            return Action::SwitchMode(Mode::default_for(&new_mode));
                        }
                    }
                    _ => {}
                },
                Options::WordCount(_) => match key.code {
                    KeyCode::Left | KeyCode::Down => {
                        self.custom_words = self.custom_words.saturating_sub(5).max(10);
                        self.words = self.custom_words;
                        self.reset();
                    }
                    KeyCode::Right | KeyCode::Up => {
                        self.custom_words += 5;
                        self.words = self.custom_words;
                        self.reset();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => self.is_editing = None,
                    _ => {}
                },
            }
            return Action::None;
        }

        match key.code {
            KeyCode::Left => match self.selected_option {
                Options::Mode(_) => self.selected_option = Options::WordCount(1000),
                Options::WordCount(count) => {
                    self.selected_option = if count == WORD_COUNTS[0] {
                        Options::default()
                    } else if count == WORD_COUNTS[1] {
                        Options::WordCount(WORD_COUNTS[0])
                    } else if count == WORD_COUNTS[2] {
                        Options::WordCount(WORD_COUNTS[1])
                    } else if count == WORD_COUNTS[3] {
                        Options::WordCount(WORD_COUNTS[2])
                    } else {
                        Options::WordCount(WORD_COUNTS[3])
                    }
                }
            },
            KeyCode::Right => match self.selected_option {
                Options::Mode(_) => self.selected_option = Options::WordCount(WORD_COUNTS[0]),
                Options::WordCount(count) => {
                    self.selected_option = if count == WORD_COUNTS[0] {
                        Options::WordCount(WORD_COUNTS[1])
                    } else if count == WORD_COUNTS[1] {
                        Options::WordCount(WORD_COUNTS[2])
                    } else if count == WORD_COUNTS[2] {
                        Options::WordCount(WORD_COUNTS[3])
                    } else if count == WORD_COUNTS[3] {
                        Options::WordCount(1000)
                    } else {
                        Options::default()
                    }
                }
            },
            KeyCode::Enter => match self.selected_option {
                Options::WordCount(count) => {
                    if WORD_COUNTS.contains(&count) {
                        self.words = count;
                        self.reset();
                    } else {
                        self.is_editing = Some(Options::WordCount(1000));
                        self.words = self.custom_words;
                        self.reset();
                    }
                }
                Options::Mode(_) => self.is_editing = Some(Options::default()),
            },
            KeyCode::Char(c) => {
                if self.start.is_none() {
                    if c == ' ' {
                        match self.selected_option {
                            Options::WordCount(count) => {
                                if WORD_COUNTS.contains(&count) {
                                    self.words = count;
                                    self.reset();
                                } else {
                                    self.is_editing = Some(Options::WordCount(1000));
                                    self.words = self.custom_words;
                                    self.reset();
                                }
                            }
                            Options::Mode(_) => self.is_editing = Some(Options::default()),
                        }
                        return Action::None;
                    }
                    self.start = Some(Instant::now());
                }

                if c == 'h' && key.modifiers.contains(KeyModifiers::CONTROL) {
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

                if let Some(start) = self.start
                    && start.elapsed() < Duration::from_millis(100)
                {
                    return Action::SwitchState(State::Running);
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

        if self.is_complete() {
            self.end = Some(Instant::now());
            return Action::SwitchState(State::Complete);
        }

        Action::None
    }
}

impl Renderer for Words {
    fn render_home_body(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

        let current_words = self.words;

        let mut config_spans = vec![];

        let (mode_text, style) = if let Some(Options::Mode(ref m)) = self.is_editing {
            (m.as_str(), SELECTED_STYLE.fg(Color::Yellow).underlined())
        } else {
            let mut style = SELECTED_STYLE;
            if let Options::Mode(_) = self.selected_option {
                style = style.underlined();
            }
            ("words", style)
        };

        config_spans.push(Span::styled(
            format!("{}{}", mode_text[0..1].to_uppercase(), &mode_text[1..]),
            style,
        ));
        config_spans.push(Span::from(" | "));

        config_spans.extend(WORD_COUNTS.iter().flat_map(|&c| {
            let mut style = if current_words == c {
                SELECTED_STYLE
            } else {
                Style::default()
            };

            if let Options::WordCount(count) = self.selected_option
                && c == count
            {
                style = style.underlined();
            }

            [Span::styled(format!("{}", c), style), Span::from(" | ")]
        }));

        let custom_selected = if let Options::WordCount(c) = self.selected_option {
            !WORD_COUNTS.contains(&c)
        } else {
            false
        };

        config_spans.push(
            if !WORD_COUNTS.contains(&current_words)
                || custom_selected
                || self
                    .is_editing
                    .as_ref()
                    .is_some_and(|x| matches!(x, Options::WordCount(_)))
            {
                let val = if !WORD_COUNTS.contains(&current_words) {
                    current_words
                } else {
                    self.custom_words
                };

                let mut style = if !WORD_COUNTS.contains(&current_words) {
                    SELECTED_STYLE
                } else {
                    Style::default()
                };

                if custom_selected {
                    style = style.underlined();
                }

                if let Some(Options::WordCount(_)) = self.is_editing {
                    style = style.fg(Color::Yellow);
                }

                Span::from(format!(" 󱁤  {}", val)).style(style)
            } else {
                Span::from(" 󱁤 ").style(Style::default())
            },
        );

        let config = Paragraph::new(Line::from(config_spans)).centered();
        config.render(layout[0], buf);

        let preview = Paragraph::new(self.target_words.join(" "))
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: false });

        preview.render(layout[2], buf);
    }

    fn render_running_body(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

        let counter = Paragraph::new(format!("{}/{}", self.typed_words.len(), self.words))
            .style(SELECTED_STYLE);
        counter.render(layout[1], buf);

        let typing_spans = get_typing_spans(&self.target_words, &self.typed_words);
        let typing_paragraph = Paragraph::new(Line::from(typing_spans)).wrap(Wrap { trim: false });
        typing_paragraph.render(layout[2], buf);
    }

    fn render_complete_body(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(6), Constraint::Min(10)]).split(area);

        let game_stats = self.get_stats();

        let stats = vec![
            Line::from(""),
            Line::from("Test Complete!").centered().style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            Line::from(format!("Average WPM: {:.1}", game_stats.wpm()))
                .centered()
                .style(Style::default().fg(Color::Cyan)),
            Line::from(format!("Accuracy: {:.1}%", game_stats.accuracy()))
                .centered()
                .style(Style::default().fg(Color::Yellow)),
            Line::from(format!("Time: {:.1}s", game_stats.duration()))
                .centered()
                .style(Style::default().fg(Color::Magenta)),
        ];

        let paragraph = Paragraph::new(stats);
        paragraph.render(layout[0], buf);

        let mut data = vec![(0.0, 0.0)];
        let mut max_wpm = 0.0;

        if let Some(start) = &self.start {
            for (words, ts) in &self.timestamps {
                let duration = ts.duration_since(*start);

                let typed_words = &self.typed_words[..*words];
                let target_words = &self.target_words[..*words];

                let stats = GameStats::calculate(duration, typed_words, target_words);
                let wpm = stats.wpm();

                if wpm > max_wpm {
                    max_wpm = wpm;
                }

                data.push((duration.as_secs_f64(), wpm));
            }
        }

        let datasets = vec![
            Dataset::default()
                .name("WPM Over Time")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(SELECTED_STYLE)
                .data(&data),
        ];

        render_wpm_chart(layout[1], buf, datasets, game_stats.duration(), max_wpm);
    }
}
