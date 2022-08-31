use crate::cli::logger::Logger;

use super::error::AppError;
use log::{log_enabled, warn, Level};
use std::io::{BufRead, BufReader};

pub struct Shell;

impl Shell {
    pub fn exec(cmd: String) -> Result<(), AppError> {
        cmd_lib::spawn_with_output!(bash -c "$cmd")?.wait_with_pipe(&mut |pipe| {
            BufReader::new(pipe)
                .lines()
                // .filter_map(|line| line.ok())
                .for_each(|line| {
                    if line.is_ok() && log_enabled!(Level::Warn) {
                        warn!("{}", line.unwrap());
                    } else {
                        Logger::loud_error(line.unwrap())
                    }
                });
        })?;
        Ok(())
    }
}
