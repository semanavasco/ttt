//! # Mode Utilities Module
//!
//! This module provides shared helper functions used by various game modes.

use crate::app::ui::char::{CharState, StyledChar};

/// Builds styled characters from target and typed words.
///
/// This function compares the user's typed input against the target text and
/// assigns a state to each character (pending, correct, etc).
pub fn build_styled_chars(target_words: &[String], typed_words: &[String]) -> Vec<StyledChar> {
    let mut chars = Vec::new();

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

            let state = if is_cursor_here {
                CharState::Cursor
            } else if let Some(&typed_char) = typed_chars.get(char_idx) {
                if typed_char == target_char {
                    CharState::Correct
                } else {
                    CharState::Incorrect
                }
            } else if is_past_word || (is_current_word && char_idx < cursor_pos.1) {
                CharState::Skipped
            } else {
                CharState::Pending
            };

            chars.push(StyledChar::new(target_char, state));
        }

        // Render extra typed characters
        for (char_idx, &typed_char) in typed_chars.iter().enumerate().skip(target_chars.len()) {
            let is_cursor_here = is_current_word && char_idx == cursor_pos.1;

            let state = if is_cursor_here {
                CharState::Cursor
            } else {
                CharState::Extra
            };

            chars.push(StyledChar::new(typed_char, state));
        }

        // Render space after word
        let cursor_on_space = is_current_word
            && cursor_pos.1 >= target_chars.len()
            && cursor_pos.1 >= typed_chars.len();

        let state = if cursor_on_space {
            CharState::Cursor
        } else {
            CharState::Pending
        };

        chars.push(StyledChar::new(' ', state));
    }

    chars
}
