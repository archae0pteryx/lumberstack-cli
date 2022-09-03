use super::arguments::CliArgs;
use clap::Parser;
use env_logger::fmt::Color as LogEnvColor;
use env_logger::fmt::Color;
use log::{log_enabled, Level};
use std::io::Write;

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
                        let msg = buf_style.set_color(Color::Cyan).value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    Level::Warn => {
                        let msg = buf_style.set_color(Color::Yellow).value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    Level::Error => {
                        let msg = buf_style
                            .set_color(Color::White)
                            .set_bg(Color::Red)
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    Level::Debug => {
                        let msg = buf_style
                            .set_color(Color::Black)
                            .set_bg(Color::White)
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    _ => writeln!(buf, "{}", record.args()),
                }
            })
            .init();
    }
}
