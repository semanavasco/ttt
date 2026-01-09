//! # UI Module
//!
//! This module is responsible for the visual representation of the application.
//! It defines the global layout, theme/styles, and the main rendering entry point.

use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::app::{App, State, modes::util::render_wpm_chart};

/// State of a character in the typing area.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CharState {
    #[default]
    Pending,
    Correct,
    Incorrect,
    Skipped,
    Cursor,
    Extra,
}

/// A single character and its state.
#[derive(Clone)]
pub struct StyledChar {
    pub char: char,
    pub state: CharState,
}

impl StyledChar {
    pub fn new(char: char, state: CharState) -> Self {
        Self { char, state }
    }
}

/// Theme configuration for consistent styling across the application.
#[derive(Clone)]
pub struct Theme {
    pub pending: Style,
    pub correct: Style,
    pub incorrect: Style,
    pub skipped: Style,
    pub cursor: Style,
    pub extra: Style,
    pub selected: Style,
    pub editing: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            pending: Style::new().fg(Color::DarkGray),
            correct: Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
            incorrect: Style::new()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
            skipped: Style::new()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::UNDERLINED)
                .underline_color(Color::Red),
            cursor: Style::new().bg(Color::White).fg(Color::DarkGray),
            extra: Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
            selected: Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            editing: Style::new()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        }
    }
}

impl Theme {
    /// Convert a CharState to its corresponding Style.
    pub fn style_for(&self, state: CharState) -> Style {
        match state {
            CharState::Pending => self.pending,
            CharState::Correct => self.correct,
            CharState::Incorrect => self.incorrect,
            CharState::Skipped => self.skipped,
            CharState::Cursor => self.cursor,
            CharState::Extra => self.extra,
        }
    }
}

/// Renders the application UI with a two-section vertical layout.
///
/// **Layout:**
/// - **Body** (flexible height): Options bar, progress indicator, typing area...
/// - **Footer** (3 rows): Key hints for the current state + borders.
///
/// Game mode data is retrieved via the [`Renderer`](super::modes::Renderer) trait
/// and styled using the application's [`Theme`].
pub fn draw(frame: &mut Frame, app: &App) {
    let layout = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(frame.area());

    let body_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .padding(Padding::symmetric(4, 2))
        .title(Line::from(" TTT ").centered());

    let body_area = body_block.inner(layout[0]);

    let footer_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_set(symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::ROUNDED
        });

    let footer_area = footer_block.inner(layout[1]);

    // Render content based on state
    match app.state {
        State::Home | State::Running => {
            render_game_body(body_area, frame.buffer_mut(), app);
        }
        State::Complete => {
            render_complete_body(body_area, frame.buffer_mut(), app);
        }
    }

    render_footer(footer_area, frame.buffer_mut(), app);

    frame.render_widget(body_block, layout[0]);
    frame.render_widget(footer_block, layout[1]);
}

/// Renders the main game area: options bar, progress, and typing area.
fn render_game_body(area: Rect, buf: &mut Buffer, app: &App) {
    let layout = Layout::vertical([
        Constraint::Length(3), // Options bar
        Constraint::Length(1), // Progress
        Constraint::Min(5),    // Typing area
    ])
    .split(area);

    if app.state == State::Home {
        render_options_bar(layout[0], buf, app);
    }

    if app.state == State::Running {
        render_progress(layout[1], buf, app);
    }

    render_typing_area(layout[2], buf, app);
}

/// Renders the mode selector and mode-specific options.
fn render_options_bar(area: Rect, buf: &mut Buffer, app: &App) {
    let mut spans = vec![];

    // Mode selector (index 0)
    let mode_name = app
        .editing_mode
        .as_deref()
        .unwrap_or_else(|| app.current_mode_name());

    let mode_style = if app.is_editing && app.focused_option == 0 {
        app.theme.editing
    } else if app.focused_option == 0 {
        app.theme.selected.add_modifier(Modifier::UNDERLINED)
    } else {
        app.theme.selected
    };

    spans.push(Span::styled(capitalize(mode_name), mode_style));
    if app.mode.option_count() > 0 {
        spans.push(Span::from(" | "));
    }

    // We pass None when mode selector is focused, otherwise pass the mode option index
    let focused_mode_option = if app.focused_option == 0 {
        None
    } else {
        Some(app.focused_option - 1) // -1 to ignore mode index
    };

    let options = app.mode.get_options(focused_mode_option);

    for (i, item) in options.items.iter().enumerate() {
        let style = if item.is_editing {
            app.theme.editing
        } else if item.is_focused {
            if item.is_active {
                app.theme.selected.add_modifier(Modifier::UNDERLINED)
            } else {
                Style::default().add_modifier(Modifier::UNDERLINED)
            }
        } else if item.is_active {
            app.theme.selected
        } else {
            Style::default()
        };

        spans.push(Span::styled(&item.label, style));

        if i < options.items.len() - 1 {
            spans.push(Span::from(" | "));
        }
    }

    Paragraph::new(Line::from(spans))
        .centered()
        .render(area, buf);
}

/// Renders the progress indicator (timer, word count, etc).
fn render_progress(area: Rect, buf: &mut Buffer, app: &App) {
    let progress = app.mode.get_progress();
    Paragraph::new(progress)
        .style(app.theme.selected)
        .render(area, buf);
}

/// Renders styled characters from the game mode using theme colors.
fn render_typing_area(area: Rect, buf: &mut Buffer, app: &App) {
    let chars = app.mode.get_characters();
    let spans: Vec<Span> = chars
        .iter()
        .map(|sc| {
            let style = app.theme.style_for(sc.state);
            Span::styled(sc.char.to_string(), style)
        })
        .collect();

    Paragraph::new(Line::from(spans))
        .wrap(Wrap { trim: false })
        .render(area, buf);
}

/// Renders the completion screen with stats and WPM chart.
fn render_complete_body(area: Rect, buf: &mut Buffer, app: &App) {
    let layout = Layout::vertical([
        Constraint::Length(6), // Stats
        Constraint::Min(10),   // WPM Chart
    ])
    .split(area);

    // Stats
    let stats = app.mode.get_stats();
    let stats_lines = vec![
        Line::from(""),
        Line::from("Test Complete!")
            .centered()
            .green()
            .add_modifier(Modifier::BOLD),
        Line::from(""),
        Line::from(format!("Average WPM: {:.1}", stats.wpm()))
            .centered()
            .cyan(),
        Line::from(format!("Accuracy: {:.1}%", stats.accuracy()))
            .centered()
            .yellow(),
        Line::from(format!("Time: {:.1}s", stats.duration()))
            .centered()
            .magenta(),
    ];
    Paragraph::new(stats_lines).render(layout[0], buf);

    // WPM Chart
    let data = app.mode.get_wpm_data();
    let max_wpm = data.iter().map(|(_, wpm)| *wpm).fold(0.0, f64::max);
    render_wpm_chart(layout[1], buf, &data, stats.duration(), max_wpm);
}

/// Renders key hints (global + mode-specific) in the footer.
fn render_footer(area: Rect, buf: &mut Buffer, app: &App) {
    let mut hints: Vec<(&str, &str)> = match app.state {
        State::Home => vec![("ESC", "Quit"), ("← →", "Navigate"), ("ENTER", "Select")],
        State::Running | State::Complete => vec![("TAB", "Restart"), ("ESC", "Quit")],
    };

    // Add mode-specific hints
    hints.extend(
        app.mode
            .footer_hints()
            .iter()
            .filter(|hint| hint.state == app.state)
            .map(|hint| (hint.key, hint.description))
            .collect::<Vec<(&str, &str)>>(),
    );

    let spans: Vec<Span> = hints
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::from(format!(" {} ", desc)),
                Span::styled(format!("({})", key), app.theme.selected),
            ]
        })
        .collect();

    Paragraph::new(Line::from(spans)).render(area, buf);
}

/// Capitalizes the first character of a string.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
