use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Chart, Dataset, GraphType, Paragraph, Widget, Wrap},
};

use crate::{
    Resource,
    app::{
        modes::{
            GameStats, Handler, Mode, Renderer,
            util::{calculate_wpm_accuracy, get_typing_spans},
        },
        ui::SELECTED_STYLE,
    },
    config::Config,
};

pub struct Clock {
    duration: Duration,
    start: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
    timestamps: Vec<(usize, Instant)>,
    text: String,
}

impl Clock {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
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
                    if let Some(last) = self.typed_words.last() {
                        if !last.is_empty() {
                            self.timestamps
                                .push((self.typed_words.len(), Instant::now()));
                            self.typed_words.push(String::new());
                        }
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

        let durations = [15, 30, 60, 120];
        let current_duration = self.duration.as_secs();

        let mut config_spans = vec![Span::from("Clock").style(SELECTED_STYLE), Span::from(" | ")];

        config_spans.extend(durations.iter().flat_map(|&d| {
            let style = if current_duration == d {
                SELECTED_STYLE
            } else {
                Style::default()
            };

            [Span::styled(format!("{}s", d), style), Span::from(" | ")]
        }));

        config_spans.push(if !durations.contains(&current_duration) {
            Span::from(format!(" 󱁤  {}", current_duration)).style(SELECTED_STYLE)
        } else {
            Span::from(" 󱁤 ")
        });

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

                let (wpm, _) = calculate_wpm_accuracy(duration, &typed_words, &target_words);

                if wpm > max_wpm {
                    max_wpm = wpm;
                }

                data.push((duration.as_secs_f64(), wpm));
            }
        }

        // Create the datasets to fill the chart with
        let datasets = vec![
            Dataset::default()
                .name("WPM Over Time")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(SELECTED_STYLE)
                .data(&data),
        ];

        // Create the X axis and define its properties
        let total_duration = self.duration.as_secs_f64();

        let x_labels = [
            "0.0",
            &format!("{:.1}", total_duration / 2.0),
            &format!("{:.1}", total_duration),
        ];

        let x_axis = Axis::default()
            .title("Time".red())
            .style(Style::default().white())
            .bounds([0.0, self.duration.as_secs_f64()])
            .labels(x_labels);

        // Create the Y axis and define its properties
        let y_labels = [
            "0.0",
            &format!("{:.1}", max_wpm / 2.0),
            &format!("{:.1}", max_wpm),
        ];

        let y_axis = Axis::default()
            .title("WPM".red())
            .style(Style::default().white())
            .bounds([0.0, max_wpm])
            .labels(y_labels);

        // Create the chart and link all the parts together
        let chart = Chart::new(datasets).x_axis(x_axis).y_axis(y_axis);

        chart.render(layout[1], buf);
    }
}
