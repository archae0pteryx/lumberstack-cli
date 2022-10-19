use anyhow::Result;
use std::process::{exit, Command, Output};

use log::{debug, error};

use crate::app_config::{load_app_config, AppConfig, DEFAULT_WORKDIR};

use super::{file_io::FileIO, logger::Logger};

pub struct System;

impl System {
    pub fn init() -> Result<AppConfig> {
        Logger::init();
        let app_config = load_app_config()?;

        if !app_config.skip_checks {
            Self::os_ok();
            Self::has_required_bin("yarn");
            Self::check_docker();
            Self::has_required_bin("node");
            Self::check_node_version();
        }

        if app_config.clean && app_config.tags.is_none() && app_config.skip_tags.is_none() {
            debug!("Found clean flag");
            FileIO::remove(&app_config.app_name);
            FileIO::remove(&app_config.workdir);
            if let Some(lf) = &app_config.log_file {
                FileIO::remove(lf);
            }
        }

        FileIO::create_dir(DEFAULT_WORKDIR)?;
        Ok(app_config)
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
}
