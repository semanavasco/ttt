use std::{path::PathBuf, time::Duration};

use clap::{Parser, builder::PossibleValuesParser};
use directories::ProjectDirs;

use crate::{
    app::modes::{AVAILABLE_MODES, Mode},
    config::Config,
};

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
    #[arg(short, long, value_parser = PossibleValuesParser::new(AVAILABLE_MODES))]
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

    /// Use default settings
    #[arg(long, default_value_t = false)]
    defaults: bool,
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

    pub fn use_defaults(&self) -> bool {
        self.defaults
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

        if let Some(mode_name) = &self.mode {
            if let Some(new_mode) = Mode::from_string(mode_name) {
                config.defaults.mode = new_mode;
            } else {
                eprintln!("Unknown mode '{}', using default", mode_name);
            }
        }

        if let Some(new_duration) = self.duration {
            match &mut config.defaults.mode {
                Mode::Clock { duration } => {
                    *duration = Duration::from_secs(new_duration);
                }
            }
        }
    }
}
