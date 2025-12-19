use std::io::{self, stdout};

use clap::Parser;
use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use ttt::app::{self, state::State};
use ttt::cli::Args;
use ttt::config::Config;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let config = if args.use_defaults() {
        Config::default()
    } else {
        args.get_config()
    };

    if args.should_save() {
        let config_str = match toml::to_string(&config) {
            Ok(config_str) => config_str,
            Err(_) => panic!("Couldn't serialize config"),
        };

        let config_path = match args.config_dir() {
            Some(dir) => {
                std::fs::create_dir_all(&dir).expect("Couldn't create config directory");
                dir.join("config.toml")
            }
            None => panic!("Couldn't find config directory"),
        };

        std::fs::write(&config_path, config_str).expect("Couldn't save config");
        println!("Saved config to {}", config_path.display());
        std::process::exit(0);
    };

    let mut terminal = ratatui::init();

    let _ = execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );

    let mut state = State::from_config(&config);
    let result = app::run(&mut terminal, &mut state, &config);

    let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
    ratatui::restore();
    result
}
