use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::state::State;

pub fn draw(frame: &mut Frame, _state: &State) {
    let layout = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(frame.area());

    let main_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .title(Line::from(" TTT ").centered());

    let footer_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_set(symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::ROUNDED
        });

    let footer = Paragraph::new(" Quit (ESC)").block(footer_block);

    frame.render_widget(main_block, layout[0]);
    frame.render_widget(footer, layout[1]);
}
