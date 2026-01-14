use std::io::stdout;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use ttt::app::{self, App};
use ttt::cli::Args;
use ttt::config::Config;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = if args.use_defaults() {
        Config::default()
    } else {
        args.get_config()
    };

    if args.should_save() {
        let config_str = toml::to_string(&config).context("Couldn't serialize config")?;

        let config_path = args
            .config_dir()
            .ok_or_else(|| anyhow!("Couldn't find config directory"))?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).context("Couldn't create config directory")?;
        }

        let config_file_path = config_path.join("config.toml");
        std::fs::create_dir_all(&config_path).context("Couldn't create config directory")?;

        std::fs::write(&config_file_path, config_str).context("Couldn't save config")?;

        println!("Saved config to {}", config_file_path.display());
        std::process::exit(0);
    };

    let mut terminal = ratatui::init();

    let _ = execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );

    let mut app = App::from_config(&config)?;
    let result = app::run(&mut terminal, &mut app, &config);

    let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
    ratatui::restore();
    result
}
