pub mod clock;

use std::time::Duration;

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub trait GameMode {
    fn initialize(&mut self, text: String);
    fn update(&mut self, delta: Duration);
    fn handle_input(&mut self, key: KeyEvent); // requires return
    fn is_complete(&self) -> bool;
    fn get_stats(&self); // requires return
}

pub trait UI {
    fn render_home(&self, area: Rect, buf: &mut Buffer);
    fn render_running(&self, area: Rect, buf: &mut Buffer);
    fn render_complete(&self, area: Rect, buf: &mut Buffer);
}
