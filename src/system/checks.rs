use std::{io::Result, process::exit};

use log::error;

pub(super) fn os_ok() {
    if cfg!(windows) {
        error!("❌ Windows is not supported at this time");
        std::process::exit(exitcode::OK);
    }
}

pub(super) fn check_node_version() -> Result<()> {
    let output = check_app_installed("node")?;
    if !output.contains("v14") {
        error!("❌ node v14 required but found: {}", output);
        exit(exitcode::OSFILE);
    }
    Ok(())
}

pub(super) fn check_app_installed(bin_name: &str) -> Result<String> {
    let app_check = cmd_lib::run_fun!($bin_name -v);
    if app_check.is_err() {
        error!("❌ {} not found but required", bin_name);
        exit(exitcode::OSFILE);
    }
    return app_check;
}

pub(super) fn check_docker() -> Result<()> {
    let check = cmd_lib::run_fun!(docker ps);
    if check.is_err() || check.unwrap().contains("Cannot connect") {
        error!("❌ Docker is not installed or not running");
        exit(exitcode::OSFILE);
    }
    Ok(())
}
