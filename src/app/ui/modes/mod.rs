use ratatui::style::{Color, Modifier, Style};

pub mod clock;

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
