use std::time::Instant;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use crate::app::state::{Menu, Mode, State};

pub fn draw(frame: &mut Frame, state: &State) {
    let layout = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(frame.area());

    let main_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(4, 2))
        .title(Line::from(" TTT ").centered());

    let main_area = main_block.inner(layout[0]);

    let selected_style = Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD);

    match state.menu {
        Menu::Home => {
            let main_layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(5),
            ])
            .split(main_area);

            let current_config = match state.mode {
                Mode::Clock { duration, .. } => {
                    let durations = [15, 30, 60, 120];
                    let current_duration = duration.as_secs();

                    let mut config_spans =
                        vec![Span::from("Clock").style(selected_style), Span::from(" | ")];

                    config_spans.extend(durations.iter().flat_map(|&d| {
                        let style = if current_duration == d {
                            selected_style
                        } else {
                            Style::default()
                        };

                        [Span::styled(format!("{}s", d), style), Span::from(" | ")]
                    }));

                    config_spans.push(if !durations.contains(&current_duration) {
                        Span::from(format!(" 󱁤  {}", current_duration)).style(selected_style)
                    } else {
                        Span::from(" 󱁤 ")
                    });

                    Paragraph::new(Line::from(config_spans)).centered()
                }
            };

            let typing_text = match &state.mode {
                Mode::Clock {
                    target_words: target_text,
                    ..
                } => Paragraph::new(target_text.join(" "))
                    .style(Style::default().fg(Color::DarkGray))
                    .wrap(Wrap { trim: false }),
            };

            frame.render_widget(current_config, main_layout[0]);
            frame.render_widget(typing_text, main_layout[2]);
        }
        Menu::Running => {
            let main_layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(5),
            ])
            .split(main_area);

            let time_text = match state.mode {
                Mode::Clock {
                    duration, start, ..
                } => {
                    if let Some(start_time) = start {
                        let time_left = duration - Instant::now().duration_since(start_time);
                        Paragraph::new(format!("{}", time_left.as_secs())).style(selected_style)
                    } else {
                        Paragraph::new("")
                    }
                }
            };

            let typing_text = match &state.mode {
                Mode::Clock {
                    target_words,
                    typed_words,
                    ..
                } => {
                    let mut styled_chars: Vec<Span> = Vec::new();

                    for (i, target_word) in target_words.iter().enumerate() {
                        if let Some(typed_word) = typed_words.iter().nth(i) {
                            let typed_chars: Vec<char> = typed_word.chars().collect();
                            let target_chars: Vec<char> = target_word.chars().collect();

                            for (j, &typed_char) in typed_chars.iter().enumerate() {
                                let style = match target_chars.get(j) {
                                    Some(&target_char) if target_char == typed_char => {
                                        Style::default()
                                            .fg(Color::Green)
                                            .add_modifier(Modifier::BOLD)
                                    }
                                    _ => Style::default()
                                        .fg(Color::Red)
                                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                                };
                                styled_chars.push(Span::styled(typed_char.to_string(), style));
                            }

                            // Add remaining untyped chars from target
                            styled_chars.extend(target_chars.iter().skip(typed_chars.len()).map(
                                |&ch| {
                                    Span::styled(
                                        ch.to_string(),
                                        Style::default().fg(Color::DarkGray),
                                    )
                                },
                            ));
                        } else {
                            styled_chars.push(Span::from(target_word).style(Color::DarkGray));
                        }

                        styled_chars.push(Span::from(" "));
                    }

                    Paragraph::new(Line::from(styled_chars)).wrap(Wrap { trim: false })
                }
            };

            frame.render_widget(time_text, main_layout[1]);
            frame.render_widget(typing_text, main_layout[2]);
        }
        Menu::Done => {}
    };

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
    frame.render_widget(footer, layout[1]);
}
