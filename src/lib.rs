//! # TTT Library
//!
//! The core library for TTT.
//! It manages application state, configuration, command-line parsing, and
//! embedded resource management.

use std::{fs, io::Error};

use directories::ProjectDirs;
use rust_embed::Embed;

pub mod app;
pub mod cli;
pub mod config;

/// Manager for application resources.
///
/// This struct handles both embedded default texts and external user-provided
/// text files for typing tests.
#[derive(Embed)]
#[folder = "res/"]
pub struct Resource;

impl Resource {
    /// Retrieves text data by name.
    ///
    /// It first checks the user's local configuration directory for a matching
    /// file in the `texts/` subdirectory. If not found, it falls back to
    /// searching the embedded resources.
    ///
    /// # Arguments
    /// * `name` - The identifier of the text to retrieve (e.g., "english", "lorem").
    ///
    /// # Errors
    /// Returns an [`Error`] if the config directory cannot be determined or if
    /// the requested text does not exist in either local storage or embedded resources.
    pub fn get_text(name: &str) -> Result<Vec<u8>, Error> {
        let project_dir = ProjectDirs::from("com", "semanavasco", "ttt").ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine config dir".to_string(),
            )
        })?;

        let texts_path = project_dir
            .config_dir()
            .to_path_buf()
            .join("texts")
            .join(name);

        if texts_path.exists() {
            fs::read(&texts_path)
        } else {
            Resource::get(name)
                .map(|f| f.data.into_owned())
                .ok_or_else(|| {
                    Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Text '{}' not found", name),
                    )
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lorem_is_present() {
        let lorem_text = Resource::get("lorem");

        assert!(lorem_text.is_some());

        let lorem_text = lorem_text.unwrap();

        println!("{:?}", std::str::from_utf8(lorem_text.data.as_ref()));
    }
}
