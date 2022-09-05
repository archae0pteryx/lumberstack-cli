use crate::manifest::CommandStep;
use log::{debug, error, info, warn};
use std::thread;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub struct Commands;

impl Commands {
    pub fn process(commands: &Vec<CommandStep>) {
        for command_step in commands.iter() {
            Self::exec(&command_step);
        }
    }

    pub fn exec(command_step: &CommandStep) {
        let context = &command_step.context.to_owned().unwrap_or("./".to_string());
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
            err.lines().for_each(|line| {
                let l = line.unwrap();
                if l.contains("warning") {
                    warn!("{}", l)
                } else {
                    error!("{}", l)
                }
            });
        });

        out.lines().for_each(|line| info!("{}", line.unwrap()));

        thread.join().unwrap();

        let status = child.wait().unwrap();
        info!("Command exited with status: {}", status);
    }
}
