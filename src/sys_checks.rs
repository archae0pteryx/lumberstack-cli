use std::process::{exit, Command};

use clap::Parser;
use indicatif::ProgressBar;
use log::error;

use crate::cli_args::CliArgs;

pub struct System;

impl System {
    pub fn check_prerequsites(spinner: &ProgressBar) {
        let args = CliArgs::parse();
        if !args.disable_checks {
            spinner.set_prefix("🚀");
            spinner.set_message("Checking system requirements");
            Self::os_ok();
            Self::check_app_installed("yarn");
            Self::check_docker();
            Self::check_app_installed("heroku");
            Self::check_app_installed("node");
            Self::check_node_version();
        }
    }

    fn os_ok() {
        if cfg!(windows) {
            error!("❌ Windows is not supported at this time");
            exit(exitcode::SOFTWARE);
        }
    }

    fn check_node_version() {
        let output = Self::check_app_installed("node");
        if !output.contains("v14") {
            error!("❌ node v14 required but found: {}", output);
            exit(exitcode::SOFTWARE);
        }
    }

    fn check_app_installed(bin_name: &str) -> String {
        let output = Command::new(bin_name).arg("-v").output();
        if output.is_err() {
            error!("❌ {} not found but required", bin_name);
            exit(exitcode::SOFTWARE);
        }

        return String::from_utf8(output.unwrap().stdout).unwrap();
    }

    // FIXME
    fn check_docker() {
        Self::check_app_installed("docker");
        let output = Command::new("docker").arg("ps").output().expect("error running docker check");
        let stdout = &output.stdout;
        let connected = String::from_utf8(stdout.to_owned()).unwrap();
        if connected.contains("Cannot connect") {
            error!("❌ Docker not running");
            exit(exitcode::SOFTWARE);
        }
    }
}
