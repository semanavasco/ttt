# Terminal Typing Test

A simple Terminal Typing Test utility written in Rust using ratatui, inspired by [Monkeytype](https://monkeytype.com/).

![TTT Preview](./ttt.gif)

## Installation

```bash
git clone https://github.com/semanavasco/ttt.git
cd ttt
cargo install --path .
```

## Usage

```bash
$ ttt
A simple Terminal Typing Test utility.

Usage: ttt [OPTIONS] [COMMAND]

Commands:
  clock  Timer-based game mode
  words  Wourd-count-based game mode
  help   Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Read config from file
  -s, --save-config      Save config, applies overrides provided by other arguments
      --defaults         Use default settings
  -h, --help             Print help
  -V, --version          Print version
```

### Example Commands

```bash
# Run with saved config or defaults
$ ttt

# Run clock mode with 60 second duration
$ ttt clock -d 60

# Run words mode with 100 words using Spanish text
$ ttt words -c 100 -t spanish

# Save current settings as default
$ ttt clock -d 45 -t english --save-config

# Load from custom config file
$ ttt --config ~/my-config.toml
```

## Embedded Texts

| Name         | Description                             |
| ------------ | --------------------------------------- |
| `english`    | 100 most common English words (default) |
| `french`     | 100 most common French words            |
| `german`     | 100 most common German words            |
| `lorem`      | 100 words of Lorem Ipsum                |
| `portuguese` | 100 most common Portuguese words        |
| `spanish`    | 100 most common Spanish words           |
| `swedish`    | 100 most common Swedish words           |

## Configuration

Config file location: `~/.config/ttt/config.toml`

### Example Config

```toml
[defaults]
text = "english"
mode = "clock"
duration = 30
```

_CLI arguments override config file settings._

Custom texts can be placed at: `~/.config/ttt/texts/`

## Contributing

### Adding a New Game Mode

1. Create `src/app/modes/newmode.rs` and implement `Handler` + `Renderer` traits:

```rust
use crate::app::{State, modes::{Action, Handler, Renderer}};
use crate::config::Config;
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct NewMode {
    // your fields
}

impl Handler for NewMode {
    fn initialize(&mut self, config: &Config) { /* ... */ }

    fn handle_input(&mut self, key: KeyEvent) -> Action {
        // Process input and return an Action to trigger state changes
        // e.g., Action::SwitchState(State::Running)
        Action::None
    }
}

impl Renderer for NewMode {
    fn render_home_body(&self, area: Rect, buf: &mut Buffer) { /* ... */ }
    fn render_running_body(&self, area: Rect, buf: &mut Buffer) { /* ... */ }
    fn render_complete_body(&self, area: Rect, buf: &mut Buffer) { /* ... */ }

    // Footer methods have default implementations in Renderer trait,
    // but you can override them:
    // fn render_home_footer(&self, area: Rect, buf: &mut Buffer) { /* ... */ }
}
```

2. Add to `src/app/modes/mod.rs`:

```rust
pub mod newmode;

use crate::app::modes::newmode::NewMode;

// Add variant to Mode enum (derives Subcommand for CLI integration)
#[derive(Serialize, Deserialize, Subcommand, Display, EnumIter, VariantNames, Clone)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum Mode {
    Clock { /* ... */ },
    Words { /* ... */ },

    /// New game mode
    NewMode {
        // your mode-specific fields with defaults
    },
}

// Update create_mode factory
pub fn create_mode(mode: &Mode) -> Box<dyn GameMode> {
    match mode {
        Mode::Clock { duration, text } => Box::new(Clock::new(/* ... */)),
        Mode::Words { count, text } => Box::new(Words::new(/* ... */)),
        Mode::NewMode { text, /* ... */ } => Box::new(NewMode::new(/* ... */)),
    }
}

// Update Mode::default_for helper
impl Mode {
    pub fn default_for(name: &str) -> Self {
        match name {
            "clock" => Mode::Clock { /* defaults */ },
            "words" => Mode::Words { /* defaults */ },
            "newmode" => Mode::NewMode { /* defaults */ },
            _ => Mode::default(),
        }
    }
}
```

3. Open a PR with your new mode _(or enjoy it locally...)_!

## Feedback

This is one of my first Rust projects and I'm actively learning! I'm open to suggestions, code reviews, and constructive criticism. Feel free to open issues. I'd appreciate if you'd let me fix them rather than opening PRs with written solutions. Thank you!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
