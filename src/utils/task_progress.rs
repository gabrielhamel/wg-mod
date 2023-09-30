pub trait TaskProgression {
    fn start(&mut self);
    fn progress(&mut self, value: f32);
    fn end(&mut self);
}

use indicatif::{ProgressBar, ProgressStyle};
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
        self.spinner =
            Some(Spinner::new(Spinners::Arrow3, self.message.clone()));
    }

    fn progress(&mut self, _: f32) {}

    fn end(&mut self) {
        self.spinner.as_mut().map(|s| s.stop_with_newline());
    }
}

pub struct TaskProgressionBar {
    bar: Option<ProgressBar>,
    message: String,
}

impl TaskProgressionBar {
    pub fn new(message: &str) -> Self {
        Self {
            bar: None,
            message: message.to_string(),
        }
    }
}

impl TaskProgression for TaskProgressionBar {
    fn start(&mut self) {
        let pb = ProgressBar::new(100);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/green}] ({eta})")
            .expect("Invalid progress bar template")
            .progress_chars("█▒▒"));
        pb.set_message(self.message.clone());

        self.bar = Some(pb);
    }

    fn progress(&mut self, value: f32) {
        let percentage = (value * 100f32) as u64;
        self.bar.as_mut().map(|pb| pb.set_position(percentage));
    }

    fn end(&mut self) {
        self.bar.as_mut().map(|s| s.finish());
        println!();
    }
}
