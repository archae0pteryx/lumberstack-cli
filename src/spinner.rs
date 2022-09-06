use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn create_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template(
            "{spinner:.yellow} {prefix} {wide_msg:.bright} {elapsed_precise:.blue}",
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
    return spinner;
}
