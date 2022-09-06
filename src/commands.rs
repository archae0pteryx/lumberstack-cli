use crate::manifest::CommandStep;
use indicatif::ProgressBar;
use log::{debug, info, warn};
use std::thread;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub struct Commands;

impl Commands {
    pub fn process(app_name: &String, commands: Vec<CommandStep>, progress_bar: &ProgressBar) {
        for command_step in commands.iter() {
            let feedback = command_step.feedback.to_owned();

            if let Some(feedback) = feedback {
                progress_bar.set_message(feedback);
            }

            Self::exec(app_name, &command_step);
        }
        progress_bar.set_message("Finished processing commands!")
    }

    pub fn exec(app_name: &String, command_step: &CommandStep) {
        let context = Self::cmd_context(app_name, &command_step.context);
        let command = &command_step.command;

        debug!("👀 Running command: [{}] in [{}]", &command, &context);

        let cmd_vec: Vec<&str> = command.split(" ").collect();
        let (program, args) = cmd_vec.split_at(1);

        let mut child = Command::new(program[0])
            .args(args)
            .stdin(Stdio::inherit())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .current_dir(&context)
            .spawn()
            .expect("error running command");

        let out = BufReader::new(child.stdout.take().unwrap());
        let err = BufReader::new(child.stderr.take().unwrap());

        let thread = thread::spawn(move || {
            err.lines().for_each(|line| warn!("{}", line.unwrap()));
        });

        out.lines().for_each(|line| info!("{}", line.unwrap()));

        thread.join().unwrap();

        let status = child.wait().unwrap();
        debug!("exited {}", status);
    }

    fn cmd_context(app_name: &String, context: &Option<String>) -> String {
        match context {
            Some(c) => {
                return c.to_owned();
            }
            None => app_name.to_owned(),
        }
    }
}
