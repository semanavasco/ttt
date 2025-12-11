use std::io::{self, stdout};

use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use ttt::app::{self, state::State};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let _ = execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );

    let mut state = State::default();
    let result = app::run(&mut terminal, &mut state);

    let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
    ratatui::restore();
    result
}
