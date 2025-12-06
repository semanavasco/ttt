use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, _app: &App) {
    let layout = Layout::vertical([Constraint::Min(1), Constraint::Max(3)]).split(frame.area());

    let main_block = Block::new().borders(Borders::ALL);
    let welcome_text =
        Paragraph::new("Welcome to the Terminal Typing Test (TTT)!").block(main_block);

    let tooltips_block = Block::new().borders(Borders::ALL);
    let tooltips_text = Paragraph::new("(q) to exit").block(tooltips_block);

    frame.render_widget(&welcome_text, layout[0]);
    frame.render_widget(&tooltips_text, layout[1]);
}
