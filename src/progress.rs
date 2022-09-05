use indicatif::{ProgressBar, ProgressStyle};
use log::{log_enabled, debug};
use std::time::Duration;

pub struct AppProgress {
    _spinner: ProgressBar,
}

impl AppProgress {
    pub fn new() -> AppProgress {
        let spinner = ProgressBar::new_spinner();
        if !log_enabled!(log::Level::Warn) {
            spinner.set_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] \u{0020} {spinner:.yellow} {wide_msg:.bright} ",
                )
                .unwrap()
                .tick_strings(&[
                    "○○○○○",
                    "○○○○○",
                    "◉○○○○",
                    "•◉○○○",
                    "○•◉○○",
                    "○○•◉○",
                    "○○○•◉",
                    "○○○○•",
                ]),
            );
            spinner.set_message(String::from("🚀 Launching Lumberstack Lovingly"));
            spinner.enable_steady_tick(Duration::from_millis(120));
        }
        debug!("🪵 Logging enabled!");
        AppProgress { _spinner: spinner }
    }

    pub fn update(&self, message: &str) {
        debug!("{}", message);
        if !log_enabled!(log::Level::Warn) {
            self._spinner.set_message(String::from(message));
        }
    }

    pub fn finish(&self, message: &str) {
        debug!("{}", message);
        if !log_enabled!(log::Level::Warn) {
            self._spinner.finish_with_message(String::from(message));
        }
    }
}
