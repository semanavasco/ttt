use rust_embed::Embed;

pub mod app;
pub mod cli;
pub mod config;

#[derive(Embed)]
#[folder = "res/"]
pub struct Resource;

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
