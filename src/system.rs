use std::{process::{exit, Command, Output}, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use log::error;

use crate::{cli_args::CliArgs, manifest::Manifest};

pub struct System;

impl System {
    pub fn init(manifest: Manifest) {
        let args = CliArgs::parse();
        if !args.skip_checks {
            Self::os_ok();
            Self::has_required_bin("yarn");
            Self::check_docker();
            Self::has_required_bin("node");
            Self::check_node_version();
        }

        Self::create_working_dir(manifest);
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

        let err = String::from_utf8(output.stderr).unwrap();

        if err.contains("Error response") || err.contains("Cannot connect") {
            error!("❌ Docker not running");
            exit(exitcode::SOFTWARE);
        }
    }

    fn create_working_dir(manifest: Manifest) {
        let workdir = &manifest.workdir.unwrap_or_default();
        fs_extra::dir::create_all(workdir, false).expect("Error creating / cleaning working dir");
    }

    pub fn load_file(path: PathBuf) -> anyhow::Result<String> {
        let file_str = fs_extra::file::read_to_string(&path).with_context(|| {
            return format!("Error loading file: {}", &path.to_string_lossy());
        })?;
        return Ok(file_str);
    }
}
