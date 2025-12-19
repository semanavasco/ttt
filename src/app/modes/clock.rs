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

use crate::{
    Resource,
    app::{
        modes::{
            AVAILABLE_MODES, GameStats, Handler, Mode, ModeAction, Renderer,
            util::{calculate_wpm_accuracy, get_typing_spans, render_wpm_chart},
        },
        ui::SELECTED_STYLE,
    },
    config::Config,
};

const DURATIONS: [u64; 4] = [15, 30, 60, 120];

enum Options {
    Mode(String),
    Durations(u64),
}

impl Default for Options {
    fn default() -> Self {
        Options::Mode("clock".to_string())
    }
}

pub struct Clock {
    selected_option: Options,
    duration: Duration,
    custom_duration: u64,
    is_editing: Option<Options>,
    start: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
    timestamps: Vec<(usize, Instant)>,
    text: String,
}

impl Clock {
    pub fn new(duration: Duration) -> Self {
        let duration_secs = duration.as_secs();
        let custom_duration = if DURATIONS.contains(&duration_secs) {
            30
        } else {
            duration_secs
        };

        Self {
            selected_option: Options::default(),
            duration,
            custom_duration,
            is_editing: None,
            start: None,
            target_words: Vec::new(),
            typed_words: Vec::new(),
            timestamps: Vec::new(),
            text: String::new(),
        }
    }

    fn generate_words(&mut self) {
        let bytes = Resource::get_text(&self.text)
            .unwrap_or_else(|_| panic!("Couldn't find \"{}\" text", &self.text));

        let text: Vec<&str> = str::from_utf8(&bytes)
            .expect("Text contains non-utf8 characters")
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
    }
}

impl Handler for Clock {
    fn initialize(&mut self, config: &Config) {
        self.text = config.defaults.text.clone();
        self.typed_words.clear();
        self.start = None;
        if let Mode::Clock { duration } = &config.defaults.mode {
            self.duration = *duration;
            let secs = duration.as_secs();
            if !DURATIONS.contains(&secs) {
                self.custom_duration = secs;
            }
        }
        self.generate_words();
    }

    fn handle_input(&mut self, key: KeyEvent) -> ModeAction {
        if let Some(editing) = &mut self.is_editing {
            match editing {
                Options::Mode(mode) => match key.code {
                    KeyCode::Left | KeyCode::Down => {
                        let current_idx = AVAILABLE_MODES
                            .iter()
                            .position(|&m| m == mode.as_str())
                            .unwrap_or(0);
                        let prev_idx = current_idx
                            .checked_sub(1)
                            .unwrap_or(AVAILABLE_MODES.len() - 1);

                        *mode = AVAILABLE_MODES[prev_idx].to_string();
                    }
                    KeyCode::Right | KeyCode::Up => {
                        let current_idx = AVAILABLE_MODES
                            .iter()
                            .position(|&m| m == mode.as_str())
                            .unwrap_or(0);
                        let next_idx = (current_idx + 1) % AVAILABLE_MODES.len();

                        *mode = AVAILABLE_MODES[next_idx].to_string();
                    }
                    KeyCode::Enter => {
                        let new_mode = mode.clone();
                        self.is_editing = None;
                        if new_mode != "clock" {
                            return ModeAction::SwitchMode(new_mode);
                        }
                    }
                    KeyCode::Char(c) if c == ' ' => {
                        let new_mode = mode.clone();
                        self.is_editing = None;
                        if new_mode != "clock" {
                            return ModeAction::SwitchMode(new_mode);
                        }
                    }
                    _ => {}
                },
                Options::Durations(_) => match key.code {
                    KeyCode::Left | KeyCode::Down => {
                        self.custom_duration = self.custom_duration.saturating_sub(5).max(5);
                        self.duration = Duration::from_secs(self.custom_duration);
                    }
                    KeyCode::Right | KeyCode::Up => {
                        self.custom_duration += 5;
                        self.duration = Duration::from_secs(self.custom_duration);
                    }
                    KeyCode::Enter => {
                        self.is_editing = None;
                    }
                    KeyCode::Char(c) if c == ' ' => {
                        self.is_editing = None;
                    }
                    _ => {}
                },
            }
            return ModeAction::None;
        }

        match key.code {
            KeyCode::Left => match self.selected_option {
                Options::Mode(_) => self.selected_option = Options::Durations(1000),
                Options::Durations(duration) => {
                    self.selected_option = if duration == DURATIONS[0] {
                        Options::default()
                    } else if duration == DURATIONS[1] {
                        Options::Durations(DURATIONS[0])
                    } else if duration == DURATIONS[2] {
                        Options::Durations(DURATIONS[1])
                    } else if duration == DURATIONS[3] {
                        Options::Durations(DURATIONS[2])
                    } else {
                        Options::Durations(DURATIONS[3])
                    }
                }
            },
            KeyCode::Right => match self.selected_option {
                Options::Mode(_) => self.selected_option = Options::Durations(DURATIONS[0]),
                Options::Durations(duration) => {
                    self.selected_option = if duration == DURATIONS[0] {
                        Options::Durations(DURATIONS[1])
                    } else if duration == DURATIONS[1] {
                        Options::Durations(DURATIONS[2])
                    } else if duration == DURATIONS[2] {
                        Options::Durations(DURATIONS[3])
                    } else if duration == DURATIONS[3] {
                        Options::Durations(1000)
                    } else {
                        Options::default()
                    }
                }
            },
            KeyCode::Enter => match self.selected_option {
                Options::Durations(duration) => {
                    if DURATIONS.contains(&duration) {
                        self.duration = Duration::from_secs(duration);
                    } else {
                        self.is_editing = Some(Options::Durations(1000));
                        self.duration = Duration::from_secs(self.custom_duration);
                    }
                }
                Options::Mode(_) => {
                    self.is_editing = Some(Options::default());
                }
            },
            KeyCode::Char(c) => {
                if c == ' ' && self.start.is_none() {
                    match self.selected_option {
                        Options::Durations(duration) => {
                            if DURATIONS.contains(&duration) {
                                self.duration = Duration::from_secs(duration);
                            } else {
                                self.is_editing = Some(Options::Durations(1000));
                                self.duration = Duration::from_secs(self.custom_duration);
                            }
                        }
                        Options::Mode(_) => {
                            self.is_editing = Some(Options::default());
                        }
                    }
                    return ModeAction::None;
                }

                if self.start.is_none() {
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
        ModeAction::None
    }

    fn is_complete(&self) -> bool {
        if let Some(start) = self.start {
            start.elapsed() >= self.duration
        } else {
            false
        }
    }

    fn handle_complete(&mut self) {
        // Doesn't need to do anything
    }

    fn get_stats(&self) -> GameStats {
        let (wpm, accuracy) =
            calculate_wpm_accuracy(self.duration, &self.typed_words, &self.target_words);

        GameStats {
            wpm,
            accuracy,
            duration: self.duration.as_secs_f64(),
        }
    }

    fn reset(&mut self) {
        self.generate_words();
        self.start = None;
        self.typed_words.clear();
        self.timestamps.clear();
    }
}

impl Renderer for Clock {
    fn render_home(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

        let current_duration = self.duration.as_secs();

        let mut config_spans = vec![];

        let (mode_text, style) = if let Some(Options::Mode(ref m)) = self.is_editing {
            (m.as_str(), SELECTED_STYLE.fg(Color::Yellow).underlined())
        } else {
            let mut style = SELECTED_STYLE;
            if let Options::Mode(_) = self.selected_option {
                style = style.underlined();
            }
            ("clock", style)
        };

        config_spans.push(Span::styled(
            format!("{}{}", mode_text[0..1].to_uppercase(), &mode_text[1..]),
            style,
        ));
        config_spans.push(Span::from(" | "));

        config_spans.extend(DURATIONS.iter().flat_map(|&d| {
            let mut style = if current_duration == d {
                SELECTED_STYLE
            } else {
                Style::default()
            };

            if let Options::Durations(duration) = self.selected_option
                && d == duration
            {
                style = style.underlined();
            }

            [Span::styled(format!("{}s", d), style), Span::from(" | ")]
        }));

        let custom_selected = if let Options::Durations(d) = self.selected_option {
            !DURATIONS.contains(&d)
        } else {
            false
        };

        config_spans.push(
            if !DURATIONS.contains(&current_duration)
                || custom_selected
                || self
                    .is_editing
                    .as_ref()
                    .is_some_and(|x| matches!(x, Options::Durations(_)))
            {
                let val = if !DURATIONS.contains(&current_duration) {
                    current_duration
                } else {
                    self.custom_duration
                };

                let mut style = if !DURATIONS.contains(&current_duration) {
                    SELECTED_STYLE
                } else {
                    Style::default()
                };

                if custom_selected {
                    style = style.underlined();
                }

                if let Some(Options::Durations(_)) = self.is_editing {
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

    fn render_running(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

        // Render timer
        if let Some(start_time) = self.start {
            let time_left = self
                .duration
                .saturating_sub(Instant::now().duration_since(start_time));
            let timer = Paragraph::new(format!("{}", time_left.as_secs())).style(SELECTED_STYLE);
            timer.render(layout[1], buf);
        }

        // Render typing area
        let typing_spans = get_typing_spans(&self.target_words, &self.typed_words);
        let typing_paragraph = Paragraph::new(Line::from(typing_spans)).wrap(Wrap { trim: false });
        typing_paragraph.render(layout[2], buf);
    }

    fn render_complete(&self, area: Rect, buf: &mut Buffer) {
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

        // Collect data
        let mut data = vec![(0.0, 0.0)];
        let mut max_wpm = 0.0;

        if let Some(start) = &self.start {
            for (words, ts) in &self.timestamps {
                let duration = ts.duration_since(*start);

                let typed_words = &self.typed_words[..*words];
                let target_words = &self.target_words[..*words];

                let (wpm, _) = calculate_wpm_accuracy(duration, typed_words, target_words);

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

        render_wpm_chart(
            layout[1],
            buf,
            datasets,
            self.duration.as_secs_f64(),
            max_wpm,
        );
    }
}
