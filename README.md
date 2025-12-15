# Terminal Typing Test

A simple Terminal Typing Test utility written in Rust using ratatui, inspired by [Monkeytype](https://monkeytype.com/).

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

Usage: ttt [OPTIONS]

Options:
  -t, --text <TEXT>          The text to get the words from
  -w, --words <WORDS>        The number of words the test includes [modes: words]
  -m, --mode <MODE>          The game mode to use [possible values: clock, words, ...]
  -d, --duration <DURATION>  The duration of the test [modes: clock]
  -c, --config <CONFIG>      Read config from file
  -s, --save-config          Save config, applies overrides provided by other arguments
      --defaults             Use default settings
  -h, --help                 Print help
  -V, --version              Print version
```

### Example Commands

```bash
# Run with saved config or defaults
$ ttt

# Run with custom settings
$ ttt --mode clock --words 50 --duration 45

# Use a specific language
$ ttt --text spanish

# Save current settings as default
$ ttt --words 75 --duration 60 --save-config

# Load from custom config file
$ ttt --config ~/my-config.toml
```

## Embedded Texts

| Name         | Description                        |
| ------------ | ---------------------------------- |
| `lorem`      | 100 words of Lorem Ipsum (default) |
| `english`    | 100 most common English words      |
| `spanish`    | 100 most common Spanish words      |
| `portuguese` | 100 most common Portuguese words   |
| `german`     | 100 most common German words       |
| `swedish`    | 100 most common Swedish words      |

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

## Contributing

### Adding a New Game Mode

1. Create `src/app/modes/newmode.rs` and implement `Handler` + `Renderer` traits:

```rust
use crate::app::modes: :{GameStats, Handler, Renderer};
use crate::config::Config;
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct NewMode {
    // your fields
}

impl Handler for NewMode {
    fn initialize(&mut self, config: &Config) { /* ... */ }
    fn handle_input(&mut self, key: KeyEvent) { /* ... */ }
    fn is_complete(&self) -> bool { /* ... */ }
    fn handle_complete(&mut self) { /* ... */ }
    fn get_stats(&self) -> GameStats { /* ... */ }
    fn reset(&mut self) { /* ... */ }
}

impl Renderer for NewMode {
    fn render_home(&self, area: Rect, buf: &mut Buffer) { /* ... */ }
    fn render_running(&self, area: Rect, buf: &mut Buffer) { /* ... */ }
    fn render_complete(&self, area: Rect, buf: &mut Buffer) { /* ... */ }
}
```

2. Add to `src/app/modes/mod.rs`:

```rust
pub mod newmode;

pub const AVAILABLE_MODES: &[&str] = &["clock", /* ... */ "newmode"];

#[derive(Serialize, Deserialize, Clone)]
pub enum Mode {
    Clock { duration: Duration },
    /* ... */
    NewMode { /* your config */ },
}

impl Mode {
    pub fn from_string(mode:  &str) -> Option<Self> {
        match mode {
            "clock" => Some(Mode::Clock { duration: default_clock_duration() }),
            /* ... */
            "newmode" => Some(Mode::NewMode { /* ...  */ }),
            _ => None,
        }
    }
}

pub fn create_mode(mode: &Mode) -> Box<dyn GameMode> {
    match mode {
        Mode::Clock { duration } => Box::new(Clock::new(*duration)),
        /* ... */
        Mode::NewMode { /* ... */ } => Box::new(NewMode::new(/* ... */)),
    }
}
```

3. Open a PR with your new mode _(or enjoy it locally...)_!

## Feedback

This is one of my first Rust projects and I'm actively learning! I'm open to suggestions, code reviews, and constructive criticism. Feel free to open issues. I'd appreciate if you'd let me fix them rather than opening PRs with written solutions. Thank you!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
