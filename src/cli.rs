use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;

use crate::config::{Config, DefaultMode, default_clock_duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The text to get the words from
    #[arg(short, long)]
    text: Option<String>,

    /// The number of words
    #[arg(short, long)]
    words: Option<u16>,

    /// The game mode to use
    #[arg(short, long)]
    mode: Option<String>,

    /// The duration of the test
    #[arg(short, long)]
    duration: Option<u64>,

    /// Read config from file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Save config, applies overrides provided by other arguments
    #[arg(short, long, default_value_t = false)]
    save_config: bool,
}

impl Args {
    pub fn get_config(&self) -> Config {
        let mut config: Config = match &self.config {
            Some(path) => {
                let content = std::fs::read_to_string(path).expect("Couldn't read config content");
                toml::from_str(&content).unwrap_or_default()
            }
            _ => {
                if let Some(config_dir) = self.config_dir()
                    && let Ok(config_str) = std::fs::read_to_string(config_dir.join("config.toml"))
                {
                    toml::from_str(&config_str).unwrap_or_default()
                } else {
                    Config::default()
                }
            }
        };

        self.apply_config_overrides(&mut config);

        config
    }

    pub fn should_save(&self) -> bool {
        self.save_config
    }

    pub fn config_dir(&self) -> Option<PathBuf> {
        let project_dir = ProjectDirs::from("com", "semanavasco", "ttt")?;
        Some(project_dir.config_dir().to_path_buf())
    }

    fn apply_config_overrides(&self, config: &mut Config) {
        if let Some(text) = &self.text {
            config.defaults.text = text.to_string();
        }

        if let Some(words) = self.words {
            config.defaults.words = words;
        }

        if let Some(mode) = &self.mode {
            match mode.to_lowercase().as_str() {
                "clock" => {
                    config.defaults.mode = DefaultMode::Clock {
                        duration: default_clock_duration(),
                    }
                }
                _ => {}
            }
        }

        if let Some(new_duration) = self.duration {
            match &mut config.defaults.mode {
                DefaultMode::Clock { duration } => *duration = new_duration,
            }
        }
    }
}
