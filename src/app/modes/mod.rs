pub mod clock;

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    app::modes::clock::Clock,
    config::{Config, Mode},
};

pub trait Handler {
    fn initialize(&mut self, config: &Config);
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

pub trait GameMode: Handler + Renderer {}
impl<T: Handler + Renderer> GameMode for T {}

pub struct GameStats {
    wpm: f64,
    accuracy: f64,
    duration: f64,
}

impl GameStats {
    fn wpm(&self) -> f64 {
        self.wpm
    }

    fn accuracy(&self) -> f64 {
        self.accuracy
    }

    fn duration(&self) -> f64 {
        self.duration
    }
}

pub fn create_mode(mode: &Mode) -> Box<dyn GameMode> {
    match mode {
        Mode::Clock { duration } => Box::new(Clock::new(*duration)),
    }
}
