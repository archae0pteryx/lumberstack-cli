use crate::manifest::CommandItem;
use indicatif::ProgressBar;
use log::{debug, error};
use std::process::{exit, Command, Stdio};

pub struct Commands;

impl Commands {
    pub fn process(commands: Vec<CommandItem>, spinner: &ProgressBar) {
        spinner.set_prefix("👟");
        for command_step in commands.iter() {
            let feedback = command_step.feedback.to_owned();

            if let Some(feedback) = feedback {
                spinner.set_message(feedback);
            }

            Self::exec_command(&command_step);
        }
    }

    pub fn exec_command(command_step: &CommandItem) {
        let command = &command_step.command;
        let context = &command_step.context.clone().unwrap_or(".".to_string());

        debug!("👀 Running command: [{}]", &command);

        let cmd_vec: Vec<&str> = command.split(" ").collect();
        let (program, args) = cmd_vec.split_at(1);

        Self::exec_raw(context, program[0], args, true);
    }

    pub fn exec_raw(context: &str, program: &str, args: &[&str], panic_fail: bool) {
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
        return Stdio::piped();
    }
}
