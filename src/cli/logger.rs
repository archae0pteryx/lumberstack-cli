use super::arguments::CliArgs;
use clap::Parser;
use env_logger::fmt::Color as LogEnvColor;
use log::log_enabled;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
pub struct Logger;

impl Logger {
    pub fn init() {
        let args = CliArgs::parse();
        env_logger::Builder::new()
            .filter_level(args.verbose.log_level_filter())
            .format(|buf, record| {
                let mut style = buf.style();
                style
                    .set_intense(true)
                    .set_bg(LogEnvColor::Rgb(78, 78, 78))
                    .set_color(LogEnvColor::Rgb(178, 178, 178));
                writeln!(buf, "{}", style.value(record.args()))
            })
            .init();
    }

    pub fn loud_error(message: String) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(
                ColorSpec::new()
                    .set_bg(Some(Color::Red))
                    .set_fg(Some(Color::White)),
            )
            .unwrap();
        writeln!(&mut stdout, "{}", message).unwrap();
    }

    pub fn loud_info(message: String) {
        if log_enabled!(log::Level::Warn) {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Rgb(0, 135, 135)))
                        .set_fg(Some(Color::White)),
                )
                .unwrap();
            writeln!(&mut stdout, "\u{0020}{}", message).unwrap();
        }
    }
}
