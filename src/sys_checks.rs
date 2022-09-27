use std::{
    fmt::Error,
    process::{exit, Command, Output},
};

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
            Self::has_required_bin("yarn");
            Self::check_docker();
            Self::has_required_bin("node");
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
        let output = Self::has_required_bin("node");
        if !output.contains("v14") {
            error!("❌ node v14 required but found: {}", output);
            exit(exitcode::SOFTWARE);
        }
    }

    pub fn has_bin(bin: &str) -> bool {
        match Self::check_app_version(bin) {
            Ok(_) => true,
            Err(e) => false
        }
    }

    fn check_app_version(bin_name: &str) -> Result<Output, std::io::Error> {
        return Command::new(bin_name).arg("--version").output();
    }

    fn has_required_bin(bin_name: &str) -> String {
        let output = Self::check_app_version(bin_name);
        if output.is_err() {
            error!("❌ {} not found but required", bin_name);
            exit(exitcode::SOFTWARE);
        }

        return String::from_utf8(output.unwrap().stdout).unwrap();
    }

    fn check_docker() {
        Self::has_required_bin("docker");
        let output = Command::new("docker").arg("ps").output().unwrap();

        // let out = String::from_utf8(output.stdout).unwrap();
        let err = String::from_utf8(output.stderr).unwrap();

        if err.contains("Error response") || err.contains("Cannot connect") {
            error!("❌ Docker not running");
            exit(exitcode::SOFTWARE);
        }
    }
}
