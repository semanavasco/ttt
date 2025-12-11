mod modes;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::state::{Mode, State};
use modes::clock::ClockMode;

pub const SELECTED_STYLE: Style = Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD);

pub fn draw(frame: &mut Frame, state: &State) {
    let layout = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(frame.area());

    let main_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .padding(ratatui::widgets::Padding::symmetric(4, 2))
        .title(Line::from(" TTT ").centered());

    let main_area = main_block.inner(layout[0]);

    match &state.mode {
        Mode::Clock {
            duration,
            start,
            target_words,
            typed_words,
        } => {
            let clock_widget =
                ClockMode::new(&state.menu, *duration, *start, target_words, typed_words);
            frame.render_widget(clock_widget, main_area);
        }
    }

    // Render footer
    let footer_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_set(symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::ROUNDED
        });

    let footer = Paragraph::new(Line::from(vec![
        Span::from(" Quit "),
        Span::from("(ESC)").style(SELECTED_STYLE),
        Span::from(" | Press any key to start your typing session..."),
    ]))
    .block(footer_block);

    frame.render_widget(main_block, layout[0]);
    frame.render_widget(footer, layout[1]);
}
