//! # Configuration Module
//!
//! This module defines the application's configuration schema, handling
//! serialization and deserialization of user preferences.

use serde::{Deserialize, Serialize};

use crate::app::modes::Mode;

/// The root configuration object.
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,
}

/// Default settings for typing tests.
#[derive(Serialize, Deserialize)]
pub struct Defaults {
    #[serde(flatten)]
    #[serde(default)]
    pub mode: Mode,
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            mode: Mode::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::modes::{default_clock_duration, default_text};

    use super::*;

    #[test]
    fn config_serialize() {
        let config = Config::default();
        let config = toml::to_string(&config).unwrap();

        println!("{}", config);

        assert!(config.contains("[defaults]"));
        assert!(config.contains("text = \"english\""));
        assert!(config.contains("mode = \"clock\""));
        assert!(config.contains("duration = 30"));
    }

    #[test]
    fn empty_config_deserialize() {
        let toml_str = "";
        let config: Config = toml::from_str(toml_str).unwrap();

        #[allow(irrefutable_let_patterns)]
        if let Mode::Clock { duration, text } = config.defaults.mode {
            assert_eq!(duration, default_clock_duration());
            assert_eq!(text, default_text());
        } else {
            panic!("Expected Clock mode");
        }
    }

    #[test]
    fn partial_config_deserialize() {
        let toml_str = r#"
            [defaults]
            mode = "clock"
            duration = 30
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();

        #[allow(irrefutable_let_patterns)]
        if let Mode::Clock { duration, text } = config.defaults.mode {
            assert_eq!(duration, 30);
            assert_eq!(text, default_text());
        } else {
            panic!("Expected Clock mode");
        }
    }
}
