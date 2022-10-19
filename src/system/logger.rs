use crate::cli_args::CliArgs;
use clap::Parser;
use env_logger::fmt::Color;
use log::{warn, Level};
use std::{env, io::Write};

pub fn log_task_skip<S: AsRef<str>>(tag: S) {
    warn!("[SKIPPING] {}", tag.as_ref());
}

pub struct Logger;

impl Logger {
    pub fn init() {
        let args = CliArgs::parse();
        Self::set_ansible_log(&args.log_file);
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
                            .set_bg(Color::Rgb(10, 10, 130))
                            .set_intense(true)
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    _ => writeln!(buf, "{}", record.args()),
                }
            })
            .init();
    }

    fn set_ansible_log(log_path: &Option<String>) {
        if let Some(log_path) = &log_path {
            env::set_var("ANSIBLE_LOG_PATH", log_path);
        }
    }
}
