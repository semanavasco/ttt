use std::io;

use ttt::app::{self, state::State};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut state = State::default();
    let result = app::run(&mut terminal, &mut state);

    ratatui::restore();
    result
}
