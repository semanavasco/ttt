use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Line,
    widgets::{Dataset, GraphType, Paragraph, Widget, Wrap},
};

use crate::{
    Resource,
    app::{
        modes::{
            GameStats, Handler, Mode, Renderer,
            util::{calculate_wpm_accuracy, get_typing_spans, render_wpm_chart},
        },
        ui::SELECTED_STYLE,
    },
    config::Config,
};

pub struct Words {
    words: usize,
    start: Option<Instant>,
    end: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
    timestamps: Vec<(usize, Instant)>,
    text: String,
}

impl Words {
    pub fn new(words: usize) -> Self {
        Self {
            words,
            start: None,
            end: None,
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
            .take(self.words)
            .map(|s| s.to_string())
            .collect();

        let mut rng = rand::rng();
        words.shuffle(&mut rng);

        self.target_words = words;
    }
}

impl Handler for Words {
    fn initialize(&mut self, config: &Config) {
        self.start = None;
        self.end = None;
        self.typed_words.clear();
        self.text = config.defaults.text.clone();
        if let Mode::Words { count } = &config.defaults.mode {
            self.words = *count;
        }
        self.generate_words();
    }

    fn handle_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
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
    }

    fn is_complete(&self) -> bool {
        self.typed_words.len() == self.target_words.len()
            && self
                .typed_words
                .last()
                .is_some_and(|w| w.len() == self.target_words.last().map_or_else(|| 5, |w| w.len()))
            || self.typed_words.len() > self.target_words.len()
    }

    fn handle_complete(&mut self) {
        self.end = Some(Instant::now());
    }

    fn get_stats(&self) -> GameStats {
        let duration = if let Some(start) = self.start
            && let Some(end) = self.end
        {
            end.duration_since(start)
        } else {
            Duration::from_secs(0)
        };

        let (wpm, accuracy) =
            calculate_wpm_accuracy(duration, &self.typed_words, &self.target_words);

        GameStats {
            wpm,
            accuracy,
            duration: duration.as_secs_f64(),
        }
    }

    fn reset(&mut self) {
        self.generate_words();
        self.start = None;
        self.end = None;
        self.typed_words.clear();
        self.timestamps.clear();
    }
}

impl Renderer for Words {
    fn render_home(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

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

        // Render word count
        let counter = Paragraph::new(format!("{}/{}", self.typed_words.len(), self.words))
            .style(SELECTED_STYLE);
        counter.render(layout[1], buf);

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

        // WPM Chart
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

        render_wpm_chart(layout[1], buf, datasets, game_stats.duration(), max_wpm);
    }
}
