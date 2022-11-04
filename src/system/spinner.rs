use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn create_spinner<T: AsRef<str>>(msg: T) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template(
            "{prefix} {elapsed_precise:.cyan} {spinner:.yellow} {msg:.bright}",
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
    spinner.enable_steady_tick(Duration::from_millis(120));
    spinner.set_message(msg.as_ref().to_string());
    spinner
}
