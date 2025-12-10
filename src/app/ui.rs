use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use crate::app::state::State;

pub fn draw(frame: &mut Frame, _state: &State) {
    let layout = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(frame.area());

    let main_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(4, 2))
        .title(Line::from(" TTT ").centered());

    let main_area = main_block.inner(layout[0]);
    let main_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Min(5),
    ])
    .split(main_area);

    let selected_style = Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD);

    // Hardcoded for now
    let config_text = Paragraph::new(Line::from(vec![
        Span::from("Clock").style(selected_style),
        Span::from(" | "),
        Span::from("15s"),
        Span::from(" | "),
        Span::from("30s").style(selected_style),
        Span::from(" | "),
        Span::from("60s"),
        Span::from(" | "),
        Span::from("120s"),
        Span::from(" | "),
        Span::from(" 󱁤 "),
    ]))
    .centered();

    let counter_text = Paragraph::new("");

    let test_text = Paragraph::new(
        "lorem ipsum dolor sit amet consectetur adipiscing elit quisque faucibus ex sapien vitae pellentesque sem placerat in id cursus mi pretium tellus duis convallis tempus leo eu aenean sed diam urna tempor pulvinar vivamus fringilla lacus nec metus bibendum egestas iaculis massa nisl malesuada lacinia integer nunc posuere ut hendrerit semper vel class aptent taciti sociosqu ad litora torquent per conubia nostra inceptos himenaeos orci varius natoque penatibus et magnis dis parturient montes nascetur ridiculus mus donec rhoncus eros lobortis nulla molestie mattis scelerisque maximus eget fermentum odio phasellus non purus est efficitur laoreet mauris pharetra vestibulum fusce dictum risu",
    )
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: false });

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
        Span::from("(ESC)").style(selected_style),
        Span::from(" | Press any key to start your typing session..."),
    ]))
    .block(footer_block);

    frame.render_widget(main_block, layout[0]);
    frame.render_widget(config_text, main_layout[0]);
    frame.render_widget(counter_text, main_layout[1]);
    frame.render_widget(test_text, main_layout[2]);
    frame.render_widget(footer, layout[1]);
}
