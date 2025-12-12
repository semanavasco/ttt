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
    pub mode: DefaultMode,
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            text: default_text(),
            words: default_word_count(),
            mode: DefaultMode::default(),
        }
    }
}

pub fn default_text() -> String {
    "lorem.txt".to_string()
}

pub fn default_word_count() -> u16 {
    100
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum DefaultMode {
    Clock {
        #[serde(default = "default_clock_duration")]
        duration: u64,
    },
}

impl Default for DefaultMode {
    fn default() -> Self {
        DefaultMode::Clock {
            duration: default_clock_duration(),
        }
    }
}

pub fn default_clock_duration() -> u64 {
    30
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
        assert!(config.contains("text = \"lorem.txt\""));
        assert!(config.contains("words = 100"));
        assert!(config.contains("mode = \"clock\""));
        assert!(config.contains("duration = 30"));
    }

    #[test]
    fn config_deserialize_with_defaults() {
        // Empty config
        let toml_str = "";
        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.defaults.text, "lorem.txt");

        // Partial config with count mode
        let toml_str = r#"
            [defaults]
            mode = "clock"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.defaults.text, "lorem.txt");

        #[allow(irrefutable_let_patterns)]
        if let DefaultMode::Clock { duration } = config.defaults.mode {
            assert_eq!(duration, 30);
        } else {
            panic!("Expected Clock mode");
        }
    }
}
