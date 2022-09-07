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
    pub fn process(commands: Vec<CommandStep>, progress_bar: &ProgressBar) {
        for command_step in commands.iter() {
            let feedback = command_step.feedback.to_owned();

            if let Some(feedback) = feedback {
                progress_bar.set_message(feedback);
            }

            Self::exec(&command_step);
        }
        progress_bar.set_message("Finished processing commands!")
    }

    pub fn exec(command_step: &CommandStep) {
        let command = &command_step.command;
        let context = &command_step.context.clone().unwrap_or(".".to_string());
        debug!("👀 Running command: [{}]", &command);

        let cmd_vec: Vec<&str> = command.split(" ").collect();
        let (program, args) = cmd_vec.split_at(1);

        let mut child = Command::new(program[0])
            .args(args)
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .current_dir(context)
            .spawn()
            .expect("error running command");

        // let out = BufReader::new(child.stdout.take().unwrap());
        // let err = BufReader::new(child.stderr.take().unwrap());

        // let thread = thread::spawn(move || {
        //     err.lines().for_each(|line| warn!("{}", line.unwrap()));
        // });

        // out.lines().for_each(|line| info!("{}", line.unwrap()));

        // thread.join().unwrap();

        child.wait_with_output().unwrap();
    }
}
