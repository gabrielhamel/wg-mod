pub trait TaskProgression {
    fn start(&mut self);
    fn progress(&mut self, value: f32);
    fn end(&mut self);
}

use spinners::{Spinner, Spinners};

pub struct TaskProgressionSpinner {
    spinner: Option<Spinner>,
    message: String,
}

impl TaskProgressionSpinner {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            spinner: None,
        }
    }
}

impl TaskProgression for TaskProgressionSpinner {
    fn start(&mut self) {
        self.spinner = Some(Spinner::new(Spinners::Arrow3, self.message.clone()));
    }

    fn progress(&mut self, value: f32) {}

    fn end(&mut self) {
        self.spinner.as_mut().map(|s| s.stop_with_newline());
    }
}
