pub mod clock;

use std::time::Duration;

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::config::Config;

pub trait GameMode {
    fn initialize(&mut self, config: Config);
    fn handle_input(&mut self, key: KeyEvent);
    fn is_complete(&self) -> bool;
    fn get_stats(&self) -> GameStats;
    fn reset(&mut self);
}

pub trait Renderer {
    fn render_home(&self, area: Rect, buf: &mut Buffer);
    fn render_running(&self, area: Rect, buf: &mut Buffer);
    fn render_complete(&self, area: Rect, buf: &mut Buffer);
}

pub struct GameStats {
    wpm: f64,
    accuracy: f64,
    duration: Duration,
}

impl GameStats {
    fn wpm(&self) -> f64 {
        self.wpm
    }

    fn accuracy(&self) -> f64 {
        self.accuracy
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}
