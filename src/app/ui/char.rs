//! # Character Module
//!
//! This module defines the core data structures for representing styled
//! characters and their states in the typing area.

/// State of a character in the typing area.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CharState {
    #[default]
    Default,
    Pending,
    Correct,
    Incorrect,
    Skipped,
    Extra,
    Cursor,
}

/// A single character and its state.
#[derive(Clone)]
pub struct StyledChar {
    pub char: char,
    pub state: CharState,
}

impl StyledChar {
    pub fn new(char: char, state: CharState) -> Self {
        Self { char, state }
    }
}
