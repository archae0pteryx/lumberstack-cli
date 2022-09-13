use super::cli_args::CliArgs;
use clap::Parser;
use env_logger::fmt::Color;
use log::Level;
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
                        let msg = buf_style
                            .set_color(Color::White)
                            .set_bg(Color::Rgb(114, 114, 114))
                            .set_intense(true)
                            .value(record.args());
                        writeln!(buf, "{}", msg)
                    }
                    Level::Warn => {
                        let msg = buf_style
                            .set_color(Color::Black)
                            .set_bg(Color::Rgb(255, 178, 43))
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
                            .set_color(Color::Black)
                            .set_bg(Color::Rgb(214, 243, 174))
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
