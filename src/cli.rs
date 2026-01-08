//! # Command Line Interface Module
//!
//! This module defines the command-line arguments for the application
//! and provides logic for loading and merging configuration from various sources.

use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;

use crate::{app::modes::Mode, config::Config};

#[derive(Parser)]
#[command(version, about = "A simple Terminal Typing Test utility.", long_about = None)]
pub struct Args {
    /// The game mode to use
    #[command(subcommand)]
    mode: Option<Mode>,

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
    /// Resolves the final application configuration.
    ///
    /// It loads configuration from a provided path, the default user config
    /// directory, or falls back to system defaults. CLI arguments are then
    /// applied as overrides.
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

        if let Some(mode) = &self.mode {
            config.defaults.mode = mode.clone();
        }

        config
    }

    /// Returns true if the user requested to save the current configuration.
    pub fn should_save(&self) -> bool {
        self.save_config
    }

    /// Returns true if the user requested to ignore config files and use defaults.
    pub fn use_defaults(&self) -> bool {
        self.defaults
    }

    /// Returns the platform-specific configuration directory for TTT.
    pub fn config_dir(&self) -> Option<PathBuf> {
        let project_dir = ProjectDirs::from("com", "semanavasco", "ttt")?;
        Some(project_dir.config_dir().to_path_buf())
    }
}
