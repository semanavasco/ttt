//! # Mode Utilities Module
//!
//! This module provides shared helper functions used by various game modes
//! for rendering text, calculating metrics, and displaying visual components
//! like charts.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Span,
    widgets::{Axis, Chart, Dataset, Widget},
};

use crate::app::ui::{CORRECT_STYLE, CURSOR_STYLE, INCORRECT_STYLE, PENDING_STYLE, SKIPPED_STYLE};

/// Generates a list of styled text spans for the typing area.
///
/// This function compares the user's typed input against the target text and
/// applies appropriate styles ([CORRECT_STYLE], [INCORRECT_STYLE],
/// [SKIPPED_STYLE], [PENDING_STYLE]) to each character.
/// It also handles the visual cursor placement.
///
/// # Arguments
/// * `target_words` - The complete list of words to be typed.
/// * `typed_words` - The list of words typed by the user so far.
pub fn get_typing_spans<'a>(
    target_words: &'a [String],
    typed_words: &'a [String],
) -> Vec<Span<'a>> {
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

/// Renders a line chart displaying WPM over time.
///
/// # Arguments
/// * `area` - The rectangular area where the chart should be drawn.
/// * `buf` - The rendering buffer.
/// * `datasets` - A vector of `Dataset` objects containing the data points.
/// * `duration` - The total duration of the session (used for the X-axis bounds).
/// * `max_wpm` - The maximum WPM achieved (used for the Y-axis bounds).
pub fn render_wpm_chart(
    area: Rect,
    buf: &mut Buffer,
    datasets: Vec<Dataset>,
    duration: f64,
    max_wpm: f64,
) {
    let y_max = max_wpm.max(10.0);

    let x_labels = [
        "0.0",
        &format!("{:.1}", duration / 2.0),
        &format!("{:.1}", duration),
    ];

    let x_axis = Axis::default()
        .title("Time".red())
        .style(Style::default().white())
        .bounds([0.0, duration])
        .labels(x_labels);

    let y_labels = [
        "0.0",
        &format!("{:.1}", y_max / 2.0),
        &format!("{:.1}", y_max),
    ];

    let y_axis = Axis::default()
        .title("WPM".red())
        .style(Style::default().white())
        .bounds([0.0, y_max])
        .labels(y_labels);

    Chart::new(datasets)
        .x_axis(x_axis)
        .y_axis(y_axis)
        .render(area, buf);
}
