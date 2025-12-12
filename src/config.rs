use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,
}

#[derive(Serialize, Deserialize)]
pub struct Defaults {
    #[serde(default = "default_text")]
    pub text: String,
    #[serde(default = "default_word_count")]
    pub words: u16,

    #[serde(flatten)]
    #[serde(default)]
    pub mode: Mode,
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            text: default_text(),
            words: default_word_count(),
            mode: Mode::default(),
        }
    }
}

pub fn default_text() -> String {
    "lorem".to_string()
}

pub fn default_word_count() -> u16 {
    100
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum Mode {
    Clock {
        #[serde(default = "default_clock_duration", with = "duration_as_secs")]
        duration: Duration,
    },
}

pub fn default_clock_duration() -> Duration {
    Duration::from_secs(30)
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Clock {
            duration: default_clock_duration(),
        }
    }
}

mod duration_as_secs {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(seconds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_serialize() {
        let config = Config::default();
        let config = toml::to_string(&config).unwrap();

        println!("{}", config);

        assert!(config.contains("[defaults]"));
        assert!(config.contains("text = \"lorem\""));
        assert!(config.contains("words = 100"));
        assert!(config.contains("mode = \"clock\""));
        assert!(config.contains("duration = 30"));
    }

    #[test]
    fn config_deserialize_with_defaults() {
        // Empty config
        let toml_str = "";
        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.defaults.text, "lorem");

        // Partial config with count mode
        let toml_str = r#"
            [defaults]
            mode = "clock"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.defaults.text, "lorem");

        #[allow(irrefutable_let_patterns)]
        if let Mode::Clock { duration, .. } = config.defaults.mode {
            assert_eq!(duration, default_clock_duration());
        } else {
            panic!("Expected Clock mode");
        }
    }
}
