use std::time::{Duration, Instant};

use crossterm::event::KeyEvent;
use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::{
    Resource,
    app::{
        modes::{GameStats, Handler, Renderer},
        state::Mode,
        ui::{
            CORRECT_STYLE, CURSOR_STYLE, INCORRECT_STYLE, PENDING_STYLE, SELECTED_STYLE,
            SKIPPED_STYLE,
        },
    },
    config::Config,
};

pub struct Clock {
    duration: Duration,
    start: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
}

impl Handler for Clock {
    fn initialize(&mut self, config: Config) {
        let bytes = Resource::get(&config.defaults.text)
            .map(|f| f.data.into_owned())
            .expect(&format!("Couldn't find \"{}\" text", &config.defaults.text));

        let text: Vec<&str> = str::from_utf8(&bytes)
            .expect("Text contains non-utf8 characters")
            .lines()
            .collect();

        let mut words: Vec<String> = text
            .iter()
            .cycle()
            .take(config.defaults.words as usize)
            .map(|s| s.to_string())
            .collect();

        let mut rng = rand::rng();
        words.shuffle(&mut rng);

        self.target_words = words;
        self.typed_words.clear();
        self.start = None;
        #[allow(irrefutable_let_patterns)]
        if let Mode::Clock { duration, .. } = &config.defaults.mode {
            self.duration = *duration;
        }
    }

    fn handle_input(&mut self, key: KeyEvent) {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key.code {
            KeyCode::Char(c) => {
                if self.start.is_none() {
                    self.start = Some(Instant::now());
                }
                if c == 'h' && key.modifiers.contains(KeyModifiers::CONTROL) {
                    if let Some((typed_idx, typed_word)) =
                        self.typed_words.iter_mut().enumerate().last()
                    {
                        if let Some(target_word) = self.target_words.get(typed_idx) {
                            if typed_word != target_word {
                                if typed_word.is_empty() {
                                    self.typed_words.pop();
                                } else {
                                    typed_word.clear();
                                }
                            }
                        }
                    }
                } else if c == ' ' {
                    self.typed_words.push(String::new());
                } else if let Some(word) = self.typed_words.last_mut() {
                    word.push(c);
                } else {
                    self.typed_words.push(c.to_string());
                }
            }
            KeyCode::Backspace => {
                if let Some((typed_idx, typed_word)) =
                    self.typed_words.iter_mut().enumerate().last()
                {
                    if let Some(target_word) = self.target_words.get(typed_idx) {
                        if typed_word != target_word && typed_word.pop().is_none() {
                            self.typed_words.pop();
                        }
                    }
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

    fn get_stats(&self) -> GameStats {
        let elapsed = self.start.map_or(Duration::ZERO, |s| s.elapsed());

        let wpm = if self.typed_words.is_empty() {
            0.0
        } else {
            self.typed_words.len() as f64 / (self.duration.as_secs_f64() / 60.0)
        };

        let accuracy = if self.typed_words.is_empty() {
            0.0
        } else {
            let correct = self
                .target_words
                .iter()
                .zip(&self.typed_words)
                .filter(|(target, typed)| *target == *typed)
                .count();

            (correct as f64 / self.typed_words.len() as f64) * 100.0
        };

        GameStats {
            wpm,
            accuracy,
            duration: elapsed,
        }
    }

    fn reset(&mut self) {
        self.start = None;
        self.typed_words.clear();
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
        let game_stats = self.get_stats();

        let stats = vec![
            Line::from(""),
            Line::from("Test Complete!").centered().style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            Line::from(format!("WPM: {:.1}", game_stats.wpm()))
                .centered()
                .style(Style::default().fg(Color::Cyan)),
            Line::from(format!("Accuracy: {:.1}%", game_stats.accuracy()))
                .centered()
                .style(Style::default().fg(Color::Yellow)),
            Line::from(format!("Time: {:.1}s", game_stats.duration().as_secs_f64()))
                .centered()
                .style(Style::default().fg(Color::Magenta)),
        ];

        let paragraph = Paragraph::new(stats);
        paragraph.render(area, buf);
    }
}

fn get_typing_spans<'a>(target_words: &'a [String], typed_words: &'a [String]) -> Vec<Span<'a>> {
    let mut spans: Vec<Span<'a>> = Vec::new();

    let cursor_pos: (usize, usize) = if typed_words.is_empty() {
        (0, 0)
    } else {
        let last_idx = typed_words.len() - 1;
        (last_idx, typed_words[last_idx].len())
    };

    for (word_idx, target_word) in target_words.iter().enumerate() {
        let target_chars: Vec<char> = target_word.chars().collect();
        let typed_word = typed_words.get(word_idx);
        let typed_chars: Vec<char> = typed_word.map(|w| w.chars().collect()).unwrap_or_default();

        let is_current_word = word_idx == cursor_pos.0;
        let is_past_word = word_idx < cursor_pos.0;

        // Render each character of the target word
        for (char_idx, &target_char) in target_chars.iter().enumerate() {
            let is_cursor_here = is_current_word && char_idx == cursor_pos.1;

            let style = if is_cursor_here {
                CURSOR_STYLE
            } else if let Some(&typed_char) = typed_chars.get(char_idx) {
                if typed_char == target_char {
                    CORRECT_STYLE
                } else {
                    INCORRECT_STYLE
                }
            } else if is_past_word || (is_current_word && char_idx < cursor_pos.1) {
                SKIPPED_STYLE
            } else {
                PENDING_STYLE
            };

            spans.push(Span::styled(target_char.to_string(), style));
        }

        // Render extra typed characters
        for (char_idx, &typed_char) in typed_chars.iter().enumerate().skip(target_chars.len()) {
            let is_cursor_here = is_current_word && char_idx == cursor_pos.1;

            let style = if is_cursor_here {
                CURSOR_STYLE
            } else {
                INCORRECT_STYLE
            };

            spans.push(Span::styled(typed_char.to_string(), style));
        }

        // Render space after word
        let cursor_on_space = is_current_word
            && cursor_pos.1 >= target_chars.len()
            && cursor_pos.1 >= typed_chars.len();

        let space_style = if cursor_on_space {
            CURSOR_STYLE
        } else {
            Style::default()
        };

        spans.push(Span::styled(" ", space_style));
    }

    spans
}
