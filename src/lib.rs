use rust_embed::Embed;

pub mod app;
pub mod config;

#[derive(Embed)]
#[folder = "res/"]
#[include = "*.txt"]
pub struct Resource;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lorem_is_present() {
        let lorem_txt = Resource::get("lorem.txt");

        assert!(lorem_txt.is_some());

        let lorem_txt = lorem_txt.unwrap();

        println!("{:?}", std::str::from_utf8(lorem_txt.data.as_ref()));
    }

    #[test]
    fn other_is_absent() {
        let other = Resource::get("test.other");

        assert!(other.is_none());
    }
}
