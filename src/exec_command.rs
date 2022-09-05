use log::{debug, error, info, warn};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

use crate::{
    lumberstack::{Lumberstack, APP_NAME_KEY},
    manifest::CommandStep,
};

pub fn exec_cmd(app_name: &String, command_step: &CommandStep) {
    let context = &command_step.context.to_owned().unwrap_or("./".to_string());
    let command = &command_step.command;

    let cmd_to_run = Lumberstack::var_replacer(&command, &APP_NAME_KEY.to_string(), &app_name);

    debug!("👀 Running command: {} in {}", &cmd_to_run, &context);

    let cmd_vec: Vec<&str> = cmd_to_run.split(" ").collect();
    let (program, args) = cmd_vec.split_at(1);

    let mut child = Command::new(program[0])
        .args(args)
        .current_dir(&context)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdout = child.stdout.take().unwrap();
    let child_stderr = child.stderr.take().unwrap();

    let out = BufReader::new(child_stdout);
    let err = BufReader::new(child_stderr);

    let thread = thread::spawn(move || {
        err.lines().for_each(|line| {
            let l = line.unwrap();
            if l.starts_with("warning") {
                warn!("{}", &l)
            } else {
                error!("{}", &l)
            }
        });
    });

    out.lines().for_each(|line| info!("{}", line.unwrap()));

    thread.join().unwrap();

    let status = child.wait().unwrap();
    info!("Command exited with status: {}", status);
}
