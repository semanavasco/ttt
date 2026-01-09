use ratatui::{
    style::{Color, Modifier, Style},
    widgets::BorderType,
};
use serde::{Deserialize, Serialize};

use crate::app::ui::CharState;

/// Theme configuration for consistent styling across the application.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Theme {
    #[serde(with = "serde_border")]
    pub border_type: BorderType,
    #[serde(with = "serde_style")]
    pub border_style: Style,

    #[serde(with = "serde_style")]
    pub default: Style,
    #[serde(with = "serde_style")]
    pub pending: Style,
    #[serde(with = "serde_style")]
    pub correct: Style,
    #[serde(with = "serde_style")]
    pub incorrect: Style,
    #[serde(with = "serde_style")]
    pub skipped: Style,
    #[serde(with = "serde_style")]
    pub cursor: Style,
    #[serde(with = "serde_style")]
    pub extra: Style,
    #[serde(with = "serde_style")]
    pub selected: Style,
    #[serde(with = "serde_style")]
    pub editing: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            border_style: Style::default(),
            border_type: BorderType::Rounded,
            default: Style::default(),
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
            extra: Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
            cursor: Style::new().bg(Color::White).fg(Color::DarkGray),
            selected: Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            editing: Style::new()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        }
    }
}

impl Theme {
    /// Convert a [`CharState`] to its corresponding Style.
    pub fn style_for(&self, state: CharState) -> Style {
        match state {
            CharState::Default => self.default,
            CharState::Pending => self.pending,
            CharState::Correct => self.correct,
            CharState::Incorrect => self.incorrect,
            CharState::Skipped => self.skipped,
            CharState::Extra => self.extra,
            CharState::Cursor => self.cursor,
        }
    }
}

/// [`Style`] serializer/deserializer.
mod serde_style {
    use super::{serde_color, serde_modifier};
    use ratatui::style::Style;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(style: &Style, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut parts = Vec::new();

        if let Some(fg) = style.fg {
            parts.push(format!("fg:{}", serde_color::to_string(fg)));
        }
        if let Some(bg) = style.bg {
            parts.push(format!("bg:{}", serde_color::to_string(bg)));
        }
        if let Some(ul) = style.underline_color {
            parts.push(format!("ul:{}", serde_color::to_string(ul)));
        }

        parts.extend(serde_modifier::to_strings(style.add_modifier));

        serializer.serialize_str(&parts.join(" "))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Style, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut style = Style::default();

        for part in s.split_whitespace() {
            if let Some(val) = part.strip_prefix("fg:") {
                if let Some(color) = serde_color::parse(val) {
                    style = style.fg(color);
                }
            } else if let Some(val) = part.strip_prefix("bg:") {
                if let Some(color) = serde_color::parse(val) {
                    style = style.bg(color);
                }
            } else if let Some(val) = part.strip_prefix("ul:") {
                if let Some(color) = serde_color::parse(val) {
                    style = style.underline_color(color);
                }
            } else if let Some(modifier) = serde_modifier::parse(part) {
                style = style.add_modifier(modifier);
            }
        }

        Ok(style)
    }
}

/// [`BorderType`] serializer/deserializer.
mod serde_border {
    use ratatui::widgets::BorderType;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(border: &BorderType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match border {
            BorderType::Plain => "plain",
            BorderType::Rounded => "rounded",
            BorderType::Double => "double",
            BorderType::Thick => "thick",
            _ => "double",
        };
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BorderType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "plain" => Ok(BorderType::Plain),
            "rounded" => Ok(BorderType::Rounded),
            "double" => Ok(BorderType::Double),
            "thick" => Ok(BorderType::Thick),
            _ => Ok(BorderType::Rounded),
        }
    }
}

/// [`Color`] parsing and formatting.
mod serde_color {
    use ratatui::style::Color;

    pub fn to_string(color: Color) -> String {
        match color {
            Color::Reset => "reset".to_string(),
            Color::Black => "black".to_string(),
            Color::Red => "red".to_string(),
            Color::Green => "green".to_string(),
            Color::Yellow => "yellow".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Magenta => "magenta".to_string(),
            Color::Cyan => "cyan".to_string(),
            Color::Gray => "gray".to_string(),
            Color::DarkGray => "dark_gray".to_string(),
            Color::LightRed => "light_red".to_string(),
            Color::LightGreen => "light_green".to_string(),
            Color::LightYellow => "light_yellow".to_string(),
            Color::LightBlue => "light_blue".to_string(),
            Color::LightMagenta => "light_magenta".to_string(),
            Color::LightCyan => "light_cyan".to_string(),
            Color::White => "white".to_string(),
            Color::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
            Color::Indexed(i) => i.to_string(),
        }
    }

    pub fn parse(s: &str) -> Option<Color> {
        match s.to_lowercase().as_str() {
            "reset" => Some(Color::Reset),
            "black" => Some(Color::Black),
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "yellow" => Some(Color::Yellow),
            "blue" => Some(Color::Blue),
            "magenta" => Some(Color::Magenta),
            "cyan" => Some(Color::Cyan),
            "gray" => Some(Color::Gray),
            "darkgray" | "dark_gray" => Some(Color::DarkGray),
            "lightred" | "light_red" => Some(Color::LightRed),
            "lightgreen" | "light_green" => Some(Color::LightGreen),
            "lightyellow" | "light_yellow" => Some(Color::LightYellow),
            "lightblue" | "light_blue" => Some(Color::LightBlue),
            "lightmagenta" | "light_magenta" => Some(Color::LightMagenta),
            "lightcyan" | "light_cyan" => Some(Color::LightCyan),
            "white" => Some(Color::White),
            s if s.starts_with('#') => parse_hex(s),
            s if s.chars().all(char::is_numeric) => s.parse::<u8>().ok().map(Color::Indexed),
            _ => None,
        }
    }

    fn parse_hex(s: &str) -> Option<Color> {
        if s.len() == 7 {
            // #RRGGBB
            let r = u8::from_str_radix(&s[1..3], 16).ok()?;
            let g = u8::from_str_radix(&s[3..5], 16).ok()?;
            let b = u8::from_str_radix(&s[5..7], 16).ok()?;
            Some(Color::Rgb(r, g, b))
        } else {
            None
        }
    }
}

/// [`Modifier`] parsing and formatting.
mod serde_modifier {
    use ratatui::style::Modifier;

    const MODIFIERS: &[(Modifier, &str)] = &[
        (Modifier::BOLD, "bold"),
        (Modifier::DIM, "dim"),
        (Modifier::ITALIC, "italic"),
        (Modifier::UNDERLINED, "underlined"),
        (Modifier::SLOW_BLINK, "slow_blink"),
        (Modifier::RAPID_BLINK, "rapid_blink"),
        (Modifier::REVERSED, "reversed"),
        (Modifier::HIDDEN, "hidden"),
        (Modifier::CROSSED_OUT, "crossed_out"),
    ];

    pub fn to_strings(modifier: Modifier) -> Vec<String> {
        MODIFIERS
            .iter()
            .filter_map(|&(bit, name)| {
                if modifier.contains(bit) {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn parse(s: &str) -> Option<Modifier> {
        MODIFIERS
            .iter()
            .find(|&&(_, name)| name == s.to_lowercase())
            .map(|&(bit, _)| bit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::{Color, Modifier, Style};

    #[test]
    fn test_serialize_deserialize_style() {
        let style = Style::new()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC);

        let theme = Theme {
            pending: style,
            ..Theme::default()
        };

        let serialized = toml::to_string(&theme).expect("Failed to serialize theme");

        // Order might vary, checking contains
        assert!(serialized.contains("fg:red"));
        assert!(serialized.contains("bg:blue"));
        assert!(serialized.contains("bold"));
        assert!(serialized.contains("italic"));

        let deserialized: Theme = toml::from_str(&serialized).expect("Failed to deserialize theme");
        assert_eq!(deserialized.pending, style);
    }

    #[test]
    fn test_deserialize_partial() {
        let toml_str = r#"
            pending = "fg:green bg:black underlined"
        "#;

        let theme: Theme = toml::from_str(toml_str).expect("Failed to deserialize partial theme");

        assert_eq!(theme.pending.fg, Some(Color::Green));
        assert_eq!(theme.pending.bg, Some(Color::Black));
        assert!(theme.pending.add_modifier.contains(Modifier::UNDERLINED));

        // Check default
        assert_eq!(theme.correct, Theme::default().correct);
    }

    #[test]
    fn test_hex_colors() {
        let toml_str = r#"
            pending = "fg:#ff0000 bg:#0000ff"
        "#;
        let theme: Theme = toml::from_str(toml_str).expect("Failed to deserialize hex colors");

        assert_eq!(theme.pending.fg, Some(Color::Rgb(255, 0, 0)));
        assert_eq!(theme.pending.bg, Some(Color::Rgb(0, 0, 255)));
    }

    #[test]
    fn test_border_type() {
        let toml_str = r#"
            border_type = "double"
        "#;
        let theme: Theme = toml::from_str(toml_str).expect("Failed to deserialize border type");
        assert_eq!(theme.border_type, ratatui::widgets::BorderType::Double);
    }
}
