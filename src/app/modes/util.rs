use std::time::Duration;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Span,
    widgets::{Axis, Chart, Dataset, Widget},
};

use crate::app::ui::{CORRECT_STYLE, CURSOR_STYLE, INCORRECT_STYLE, PENDING_STYLE, SKIPPED_STYLE};

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

pub fn calculate_wpm_accuracy(
    duration: Duration,
    typed_words: &[String],
    target_words: &[String],
) -> (f64, f64) {
    let duration_mins = duration.as_secs_f64() / 60.0;

    if typed_words.is_empty() || duration_mins == 0.0 {
        return (0.0, 0.0);
    }

    let mut total_chars = 0;
    let mut correct_chars = 0;

    for (i, typed) in typed_words.iter().enumerate() {
        if let Some(target) = target_words.get(i) {
            total_chars += typed.len();

            let min_len = typed.len().min(target.len());
            for j in 0..min_len {
                if typed.chars().nth(j) == target.chars().nth(j) {
                    correct_chars += 1;
                }
            }

            if i < typed_words.len() - 1 {
                total_chars += 1;
                if typed == target {
                    correct_chars += 1;
                }
            }
        }
    }

    let accuracy = if total_chars > 0 {
        (correct_chars as f64 / total_chars as f64) * 100.0
    } else {
        0.0
    };

    let gross_wpm = (total_chars as f64 / 5.0) / duration_mins;
    let wpm = gross_wpm * (accuracy / 100.0);

    (wpm, accuracy)
}

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
