// use crate::manifest::CommandItem;
use log::error;
use std::process::{exit, Command, Stdio};

pub struct ShellCommand;

impl ShellCommand {
    pub fn exec(context: &str, program: &str, args: &[&str], panic_fail: bool) {
        let std_err = Self::should_inherit_stdio();
        let std_out = Self::should_inherit_stdio();

        let child = Command::new(program)
            .args(args)
            .stdin(Stdio::inherit())
            .stderr(std_err)
            .stdout(std_out)
            .current_dir(context)
            .spawn();

        match child {
            Ok(c) => {
                c.wait_with_output().expect("ERROR RUNNING COMMAND");
            }
            Err(e) => {
                if panic_fail {
                    error!("Error running command {} - {}", program, e);
                    exit(exitcode::IOERR);
                }
            }
        }
    }

    fn should_inherit_stdio() -> Stdio {
        if log::log_enabled!(log::Level::Info) {
            return Stdio::inherit();
        }
        Stdio::piped()
    }
}
