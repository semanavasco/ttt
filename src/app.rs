use std::time::Instant;

const TEXT: &str = "Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.";

pub struct App {
    pub state: State,
    pub target_text: String,
    pub typed_text: String,
    pub start_time: Option<Instant>,
}

impl App {
    pub fn calculate_wpm(&self) -> f64 {
        if self.typed_text.is_empty() {
            0.0
        } else {
            let words = self.typed_text.len() as f64 / 5.0;
            words / 0.5
        }
    }

    pub fn calculate_accuracy(&self) -> f64 {
        if self.typed_text.is_empty() {
            return 0.0;
        }

        let correct = self
            .target_text
            .chars()
            .zip(self.typed_text.chars())
            .filter(|(t, u)| t == u)
            .count();

        (correct as f64 / self.typed_text.len() as f64) * 100.0
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            state: State::NotStarted,
            target_text: TEXT.to_string(),
            typed_text: String::new(),
            start_time: None,
        }
    }
}

pub enum State {
    NotStarted,
    Started,
    Ended,
}
