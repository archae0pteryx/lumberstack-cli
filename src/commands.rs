use crate::manifest::CommandStep;
use indicatif::ProgressBar;
use log::{debug, error, info, warn};
use std::process::exit;
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

        if !status.success() {
            error!("Error running {}. Failing hard", command);
            exit(exitcode::IOERR);
        }
    }

    fn cmd_context(app_name: &String, context: &Option<String>) -> String {
        match context {
            Some(c) => {
                if c.eq(".") {
                    String::from(c)
                } else {
                    format!("{}/{}", app_name, c)
                }
            }
            None => app_name.to_owned(),
        }
    }
}
