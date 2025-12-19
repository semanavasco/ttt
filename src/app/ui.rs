//! # UI Module
//!
//! This module is responsible for the visual representation of the application.
//! It defines the global layout, common styles for text states, and the main
//! rendering entry point.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders},
};

use crate::app::App;

/// Style for the currently active or highlighted element.
pub const SELECTED_STYLE: Style = Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD);

/// Style for characters typed correctly.
pub const CORRECT_STYLE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);

/// Style for characters typed incorrectly.
pub const INCORRECT_STYLE: Style = Style::new()
    .fg(Color::Red)
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);

/// Style for text that has not yet been typed.
pub const PENDING_STYLE: Style = Style::new().fg(Color::DarkGray);

/// Style for characters that were skipped (e.g., jumping to the next word).
pub const SKIPPED_STYLE: Style = Style::new()
    .fg(Color::DarkGray)
    .add_modifier(Modifier::UNDERLINED)
    .underline_color(Color::Red);

/// Style for the typing cursor.
pub const CURSOR_STYLE: Style = Style::new().bg(Color::White).fg(Color::DarkGray);

/// Renders the user interface for the application.
///
/// This function orchestrates the top-level layout by splitting the available [Frame]
/// area into two vertical sections:
/// 1. **Body Area**: A flexible section (minimum 10 rows) that displays the main
///    typing interface or menu content.
/// 2. **Footer Area**: A fixed-height section (3 rows - 2 for borders = 1 line) used for
///    status information and keybindings.
///
/// The actual content within these blocks is delegated to the current [`App`]'s
/// active mode and state via `render_body` and `render_footer`.
///
/// # Arguments
/// * `frame` - The terminal frame used for rendering.
/// * `app` - The global application state.
pub fn draw(frame: &mut Frame, app: &App) {
    let layout = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(frame.area());

    let body_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .padding(ratatui::widgets::Padding::symmetric(4, 2))
        .title(Line::from(" TTT ").centered());

    let body_area = body_block.inner(layout[0]);
    app.mode
        .render_body(body_area, frame.buffer_mut(), &app.state);

    let footer_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_set(symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::ROUNDED
        });

    let footer_area = footer_block.inner(layout[1]);
    app.mode
        .render_footer(footer_area, frame.buffer_mut(), &app.state);

    frame.render_widget(body_block, layout[0]);
    frame.render_widget(footer_block, layout[1]);
}
