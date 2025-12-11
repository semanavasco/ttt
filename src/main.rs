use std::io::{self, stdout};

use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use ttt::app::{self, state::State};
use ttt::config::Config;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let _ = execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );

    let config = Config::default();
    let mut state = State::from_config(&config);
    let result = app::run(&mut terminal, &mut state);

    let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
    ratatui::restore();
    result
}
