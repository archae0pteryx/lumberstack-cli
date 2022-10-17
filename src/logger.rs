use super::cli_args::CliArgs;
use clap::Parser;
use env_logger::fmt::Color;
use log::{warn, Level};
use std::io::Write;

// pub fn log_run<S: AsRef<str>>(tag: S) {
//     warn!("[+] {}", tag.as_ref());
// }

// pub fn log_error<S: AsRef<str>>(scope: S, msg: S) {
//     error!("[ERROR][{}] {}", scope.as_ref(), msg.as_ref());
// }

// pub fn log_warn<S: AsRef<str>>(scope: S, msg: S) {
//     warn!("[{}] {}", scope.as_ref(), msg.as_ref())
// }

pub fn log_skip<S: AsRef<str>>(tag: S) {
    warn!("[SKIPPING] {}", tag.as_ref());
}

pub struct Logger;

impl Logger {
    pub fn init() {
        let args = CliArgs::parse();
        env_logger::Builder::new()
            .filter_level(args.verbose.log_level_filter())
            .format(|buf, record| {
                let mut buf_style = buf.style();
                match record.level() {
                    Level::Info => {
                        let msg = buf_style
                            .set_color(Color::Rgb(86, 137, 159))
                            .set_bg(Color::Rgb(199, 239, 255))
                            .set_intense(true)
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    Level::Warn => {
                        let msg = buf_style
                            .set_color(Color::Rgb(100, 100, 100))
                            .set_bg(Color::Rgb(255, 229, 92))
                            .set_intense(true)
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    Level::Error => {
                        let msg = buf_style
                            .set_intense(true)
                            .set_color(Color::White)
                            .set_bg(Color::Rgb(252, 120, 80))
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    Level::Debug => {
                        let msg = buf_style
                            .set_color(Color::White)
                            .set_bg(Color::Rgb(90, 90, 90))
                            .set_intense(true)
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    _ => writeln!(buf, "{}", record.args()),
                }
            })
            .init();
    }
}
