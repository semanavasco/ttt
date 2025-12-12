use std::time::{Duration, Instant};

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::app::modes::{GameMode, UI};

pub struct Clock {
    duration: Duration,
    start: Option<Instant>,
    target_words: Vec<String>,
    typed_words: Vec<String>,
}

impl GameMode for Clock {
    fn initialize(&mut self, text: String) {
        todo!("");
    }

    fn update(&mut self, delta: Duration) {
        todo!("");
    }

    fn get_stats(&self) {
        todo!("");
    }

    fn is_complete(&self) -> bool {
        todo!("");
    }

    fn handle_input(&mut self, key: KeyEvent) {
        todo!("");
    }
}

impl UI for Clock {
    fn render_home(&self, area: Rect, buf: &mut Buffer) {
        todo!("");
    }

    fn render_running(&self, area: Rect, buf: &mut Buffer) {
        todo!("");
    }

    fn render_complete(&self, area: Rect, buf: &mut Buffer) {
        todo!("");
    }
}
