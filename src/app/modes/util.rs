use ratatui::{style::Style, text::Span};

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
