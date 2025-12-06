use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    render_header(frame, &layout[0], app);
    render_body(frame, &layout[1], app);
    render_footer(frame, &layout[2], app);
}

fn render_header(frame: &mut Frame, area: &Rect, _app: &App) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = Paragraph::new(Line::from("TTT").alignment(Alignment::Center)).block(block);

    frame.render_widget(&text, *area);
}

fn render_body(frame: &mut Frame, area: &Rect, _app: &App) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = Paragraph::new("Welcome to the Terminal Typing Test (TTT)!").block(block);

    frame.render_widget(&text, *area);
}

fn render_footer(frame: &mut Frame, area: &Rect, _app: &App) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = Paragraph::new(" Quit: q").block(block);

    frame.render_widget(&text, *area);
}
