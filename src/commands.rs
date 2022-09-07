use crate::manifest::CommandStep;
use indicatif::ProgressBar;
use log::debug;
use std::process::{Command, Stdio};

pub struct Commands;

impl Commands {
    pub fn process(commands: Vec<CommandStep>, progress_bar: &ProgressBar) {
        for command_step in commands.iter() {
            let feedback = command_step.feedback.to_owned();

            if let Some(feedback) = feedback {
                progress_bar.set_message(feedback);
            }

            Self::exec(&command_step);
        }
    }

    pub fn exec(command_step: &CommandStep) {
        let command = &command_step.command;
        let context = &command_step.context.clone().unwrap_or(".".to_string());

        debug!("👀 Running command: [{}]", &command);

        let cmd_vec: Vec<&str> = command.split(" ").collect();
        let (program, args) = cmd_vec.split_at(1);

        let std_err = Self::should_inherit_stdio();
        let std_out = Self::should_inherit_stdio();

        let child = Command::new(program[0])
            .args(args)
            .stdin(Stdio::inherit())
            .stderr(std_err)
            .stdout(std_out)
            .current_dir(context)
            .spawn()
            .expect("error running command");

        child.wait_with_output().unwrap();
    }

    fn should_inherit_stdio() -> Stdio {
        if log::log_enabled!(log::Level::Info) {
            return Stdio::inherit();
        }
        return Stdio::piped();
    }
}
