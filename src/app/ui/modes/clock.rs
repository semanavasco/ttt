use std::time::{Duration, Instant};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::app::{
    state::Menu,
    ui::{
        CORRECT_STYLE, CURSOR_STYLE, INCORRECT_STYLE, PENDING_STYLE, SELECTED_STYLE, SKIPPED_STYLE,
    },
};

pub struct ClockMode<'a> {
    pub menu: &'a Menu,
    pub duration: Duration,
    pub start: Option<Instant>,
    pub target_words: &'a [String],
    pub typed_words: &'a [String],
}

impl<'a> ClockMode<'a> {
    pub fn new(
        menu: &'a Menu,
        duration: Duration,
        start: Option<Instant>,
        target_words: &'a [String],
        typed_words: &'a [String],
    ) -> Self {
        Self {
            menu,
            duration,
            start,
            target_words,
            typed_words,
        }
    }

    fn render_home(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

        self.render_selected_config(layout[0], buf);

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
        let typing_spans = get_typing_spans(self.target_words, self.typed_words);
        let typing_paragraph = Paragraph::new(Line::from(typing_spans)).wrap(Wrap { trim: false });
        typing_paragraph.render(layout[2], buf);
    }

    fn render_done(&self, area: Rect, buf: &mut Buffer) {
        let wpm = if self.typed_words.is_empty() {
            0.0
        } else {
            self.typed_words.len() as f64 / (self.duration.as_secs_f64() / 60.0)
        };

        let accuracy = if self.target_words.is_empty() {
            0.0
        } else {
            let correct = self
                .target_words
                .iter()
                .zip(self.typed_words)
                .filter(|(target, typed)| target == typed)
                .count();

            (correct as f64 / self.typed_words.len() as f64) * 100.0
        };

        let stats = vec![
            Line::from(""),
            Line::from("Test Complete!").centered().style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            Line::from(format!("WPM: {:.1}", wpm))
                .centered()
                .style(Style::default().fg(Color::Cyan)),
            Line::from(format!("Accuracy: {:.1}%", accuracy))
                .centered()
                .style(Style::default().fg(Color::Yellow)),
            Line::from(format!("Time: {}s", self.duration.as_secs()))
                .centered()
                .style(Style::default().fg(Color::Magenta)),
        ];

        let paragraph = Paragraph::new(stats);

        paragraph.render(area, buf);
    }

    fn render_selected_config(&self, area: Rect, buf: &mut Buffer) {
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
        config.render(area, buf);
    }
}

impl Widget for ClockMode<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.menu {
            Menu::Home => self.render_home(area, buf),
            Menu::Running => self.render_running(area, buf),
            Menu::Done => self.render_done(area, buf),
        }
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
