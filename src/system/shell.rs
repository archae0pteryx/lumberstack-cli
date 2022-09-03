use super::error::AppError;
use log::{error, info, log_enabled, warn, Level, debug};
use std::io::{BufRead, BufReader};

pub struct Shell;

impl Shell {
    pub fn exec(cmd: String) -> Result<(), AppError> {
        debug!("🔦 [shell::exec] {}", cmd);
        cmd_lib::spawn_with_output!(bash -c "$cmd")?.wait_with_pipe(&mut |pipe| {
            BufReader::new(pipe).lines().for_each(|line| match line {
                Ok(l) => info!("{}", l),
                Err(e) => error!("{}", e),
            });
        })?;
        Ok(())
    }

    pub fn exec_quiet(cmd: String) -> Result<(), AppError> {
        let out = cmd_lib::run_fun!(cmd).map_err(|err| {
            AppError {
                message: format!("[shell::exec_quiet] Error executing: {}", cmd)
            }
        })?;
        println!("huh? {}", out);
        Ok(())
    }
}
