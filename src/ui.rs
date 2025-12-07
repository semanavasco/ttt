use std::time::Instant;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::{App, State};

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

    frame.render_widget(text, *area);
}

fn render_body(frame: &mut Frame, area: &Rect, app: &App) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    match app.state {
        State::NotStarted => {
            let text = Paragraph::new(vec![
                Line::from("30")
                    .style(
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    )
                    .centered(),
                Line::from(
                    app.target_text
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            Span::from(c.to_string()).style(if i == 0 {
                                Style::default().bg(Color::White)
                            } else {
                                Style::default()
                            })
                        })
                        .collect::<Vec<Span>>(),
                )
                .centered()
                .style(Style::default().fg(Color::DarkGray)),
            ])
            .wrap(Wrap { trim: false })
            .block(block);

            frame.render_widget(text, *area);
        }
        State::Started => {
            let mut text: Vec<Span> = vec![];

            for (i, target_char) in app.target_text.chars().enumerate() {
                if let Some(typed_char) = app.typed_text.chars().nth(i) {
                    let style = if typed_char == target_char {
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(Color::Red)
                            .add_modifier(Modifier::UNDERLINED)
                    };

                    text.push(Span::from(typed_char.to_string()).style(style));
                } else {
                    let style = if i == app.typed_text.len() {
                        Style::default().fg(Color::DarkGray).bg(Color::White)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };

                    text.push(Span::styled(target_char.to_string(), style));
                }
            }

            let paragraph = Paragraph::new(vec![
                Line::from(format!(
                    "{}",
                    if let Some(start_time) = app.start_time {
                        30 - Instant::now().duration_since(start_time).as_secs()
                    } else {
                        30
                    }
                ))
                .style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )
                .centered(),
                Line::from(text).centered(),
            ])
            .block(block)
            .wrap(Wrap { trim: false });

            frame.render_widget(paragraph, *area);
        }
        State::Ended => {
            let wpm = app.calculate_wpm();
            let accuracy = app.calculate_accuracy();

            let stats = vec![
                Line::from(""),
                Line::from("Test Complete!").centered().style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Line::from(""),
                Line::from(format!("WPM: {:.1}", wpm))
                    .centered()
                    .style(Style::default().fg(Color::Cyan)),
                Line::from(format!("Accuracy: {:.1}%", accuracy))
                    .centered()
                    .style(Style::default().fg(Color::Yellow)),
                Line::from("Time: 30s")
                    .centered()
                    .style(Style::default().fg(Color::Magenta)),
            ];

            let paragraph = Paragraph::new(stats).block(block);
            frame.render_widget(paragraph, *area);
        }
    }
}

fn render_footer(frame: &mut Frame, area: &Rect, _app: &App) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = Paragraph::new(match _app.state {
        State::NotStarted => " Quit: ESC | Start typing to launch test...",
        State::Started => " Quit: ESC | Restart: TAB | Backspace: delete",
        State::Ended => " Quit: ESC | Restart: TAB",
    })
    .block(block);

    frame.render_widget(text, *area);
}
