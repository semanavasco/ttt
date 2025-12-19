use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::state::{Menu, State};

pub const SELECTED_STYLE: Style = Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD);
pub const CORRECT_STYLE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
pub const INCORRECT_STYLE: Style = Style::new()
    .fg(Color::Red)
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);
pub const PENDING_STYLE: Style = Style::new().fg(Color::DarkGray);
pub const SKIPPED_STYLE: Style = Style::new()
    .fg(Color::DarkGray)
    .add_modifier(Modifier::UNDERLINED)
    .underline_color(Color::Red);
pub const CURSOR_STYLE: Style = Style::new().bg(Color::White).fg(Color::DarkGray);

pub fn draw(frame: &mut Frame, state: &State) {
    let layout = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(frame.area());

    let main_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .padding(ratatui::widgets::Padding::symmetric(4, 2))
        .title(Line::from(" TTT ").centered());

    let main_area = main_block.inner(layout[0]);

    // Render mode using trait
    match state.menu {
        Menu::Home => state.mode.render_home(main_area, frame.buffer_mut()),
        Menu::Running => state.mode.render_running(main_area, frame.buffer_mut()),
        Menu::Completed => state.mode.render_complete(main_area, frame.buffer_mut()),
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

    let footer = Paragraph::new(Line::from(match state.menu {
        Menu::Home => {
            vec![
                Span::from(" Quit "),
                Span::from("(ESC)").style(SELECTED_STYLE),
                Span::from(" | Navigate Options "),
                Span::from("(<- | ->)").style(SELECTED_STYLE),
                Span::from(" | Select "),
                Span::from("(ENTER/SPACE)").style(SELECTED_STYLE),
                Span::from(" | Press any key to start your typing session... "),
            ]
        }
        Menu::Running | Menu::Completed => {
            vec![
                Span::from(" Quit "),
                Span::from("(ESC)").style(SELECTED_STYLE),
                Span::from(" | Restart "),
                Span::from("(TAB)").style(SELECTED_STYLE),
            ]
        }
    }))
    .block(footer_block);

    frame.render_widget(main_block, layout[0]);
    frame.render_widget(footer, layout[1]);
}
