use std::process::exit;

use clap::Parser;
use log::error;

use crate::cli_args::CliArgs;

pub struct System;

impl System {
    pub fn check_prerequsites() {
        let args = CliArgs::parse();
        if !args.disable_checks {
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
            exit(exitcode::OK);
        }
    }

    fn check_node_version() {
        let output = Self::check_app_installed("node");
        if !output.contains("v14") {
            error!("❌ node v14 required but found: {}", output);
            exit(exitcode::OSFILE);
        }
    }

    fn check_app_installed(bin_name: &str) -> String {
        let app_check = cmd_lib::run_fun!($bin_name -v);
        if app_check.is_err() {
            error!("❌ {} not found but required", bin_name);
            exit(exitcode::OSFILE);
        }

        return app_check.unwrap_or(String::new());
    }

    fn check_docker() {
        let check = cmd_lib::run_fun!(docker ps);
        if check.is_err() || check.unwrap().contains("Cannot connect") {
            error!("❌ Docker is not installed or not running");
            exit(exitcode::OSFILE);
        }
    }
}
