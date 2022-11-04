use anyhow::Result;
use std::process::{Command, Output};

use log::debug;

use crate::app_config::{load_app_config, AppConfig, DEFAULT_WORKDIR};

use super::{file_io::FileIO, logger::Logger, spinner::create_spinner};

pub fn init_system() -> Result<Box<AppConfig>> {
    let commands = LumberStackSysCommands {};
    let config = System::new(commands).run()?;
    Ok(Box::from(config))
}

pub struct System<T: SysCommands> {
    command_runner: T,
}

impl<T: SysCommands> System<T> {
    pub fn new(command_runner: T) -> Self {
        Self { command_runner }
    }

    pub fn run(&self) -> Result<AppConfig> {
        let app_config = load_app_config()?;

        Logger::init(&app_config);

        let spinner = create_spinner("Initializing...");
        spinner.set_prefix("🖥 ");


        if !app_config.skip_checks {
            self.os_ok()?;
            self.has_required_bin("yarn")?;
            self.check_docker()?;
            self.has_required_bin("node")?;
        }

        if app_config.clean && app_config.tags.is_empty() && app_config.skip_tags.is_empty() {
            debug!("Found clean flag");
            FileIO::remove(&app_config.app_name);
            FileIO::remove(&app_config.workdir);
            if let Some(lf) = &app_config.log_file {
                FileIO::remove(lf);
            }
        }

        self.create_working_dir(String::from(DEFAULT_WORKDIR))?;
        spinner.finish_and_clear();
        Ok(app_config)
    }

    fn os_ok(&self) -> Result<()> {
        if self.command_runner.is_windows() {
            return Err(anyhow::format_err!("System not supported"));
        }

        Ok(())
    }

    fn has_required_bin(&self, bin_name: &str) -> Result<String> {
        match self.command_runner.app_version(bin_name) {
            Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
            Err(_) => Err(anyhow::format_err!(
                "❌ {} not found but required",
                bin_name
            )),
        }
    }

    fn check_docker(&self) -> Result<()> {
        self.has_required_bin("docker")?;

        match self.command_runner.docker_ps() {
            Err(_) => {
                return Err(anyhow::format_err!("❌ Docker not running"));
            }
            Ok(output) => {
                let message = String::from_utf8(output.stderr).unwrap();
                if message.contains("Error response") || message.contains("Cannot connect") {
                    return Err(anyhow::format_err!("❌ Docker not running"));
                }
            }
        }
        Ok(())
    }

    fn create_working_dir(&self, dir: String) -> Result<()> {
        match self.command_runner.crate_dir(dir) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow::format_err!(
                "❌ Error creating / cleaning working dir"
            )),
        }
    }
}

pub trait SysCommands {
    fn app_version(&self, bin_name: &str) -> Result<Output, std::io::Error>;
    fn docker_ps(&self) -> Result<Output, std::io::Error>;
    fn is_windows(&self) -> bool;
    fn crate_dir(&self, dir: String) -> Result<(), fs_extra::error::Error>;
}

pub struct LumberStackSysCommands;

impl SysCommands for LumberStackSysCommands {
    fn app_version(&self, bin_name: &str) -> Result<Output, std::io::Error> {
        Command::new(bin_name).arg("--version").output()
    }

    fn docker_ps(&self) -> Result<Output, std::io::Error> {
        Command::new("docker").arg("ps").output()
    }

    fn is_windows(&self) -> bool {
        cfg!(windows)
    }

    fn crate_dir(&self, path: String) -> Result<(), fs_extra::error::Error> {
        fs_extra::dir::create_all(path, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    struct FakeSysCommandsPass {
        stdout_str: String,
    }
    struct FakeSysCommandsFail;

    impl SysCommands for FakeSysCommandsPass {
        fn app_version(&self, _: &str) -> Result<Output, std::io::Error> {
            let output = Output {
                status: ExitStatus::from_raw(0x007f),
                stdout: Vec::from(self.stdout_str.as_bytes()),
                stderr: Vec::new(),
            };
            return Ok(output);
        }

        fn docker_ps(&self) -> Result<Output, std::io::Error> {
            let output = Output {
                status: ExitStatus::from_raw(0x007f),
                stdout: Vec::from("docker".as_bytes()),
                stderr: Vec::new(),
            };
            return Ok(output);
        }

        fn is_windows(&self) -> bool {
            return false;
        }

        fn crate_dir(&self, _: String) -> Result<(), fs_extra::error::Error> {
            Ok(())
        }
    }

    impl SysCommands for FakeSysCommandsFail {
        fn app_version(&self, _bin_name: &str) -> Result<Output, std::io::Error> {
            let e = std::io::Error::new(std::io::ErrorKind::Other, "BOOM");
            return Err(e);
        }

        fn docker_ps(&self) -> Result<Output, std::io::Error> {
            let e = std::io::Error::new(std::io::ErrorKind::Other, "BOOM");
            return Err(e);
        }

        fn is_windows(&self) -> bool {
            true
        }

        fn crate_dir(&self, _: String) -> Result<(), fs_extra::error::Error> {
            let e = fs_extra::error::Error::new(fs_extra::error::ErrorKind::Other, "BOOM");
            return Err(e);
        }
    }

    #[test]
    fn has_required_bin_success() {
        let commands = FakeSysCommandsPass {
            stdout_str: String::from("yarn"),
        };

        let system = System::new(commands);
        match system.has_required_bin("yarn") {
            Ok(value) => assert_eq!(value, "yarn"),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn has_required_bin_fail() {
        let commands = FakeSysCommandsFail {};

        let system = System::new(commands);
        match system.has_required_bin("yarn") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn os_ok_success() {
        let commands = FakeSysCommandsPass {
            stdout_str: String::from(""),
        };

        let system = System::new(commands);
        match system.os_ok() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn os_ok_fail() {
        let commands = FakeSysCommandsFail {};

        let system = System::new(commands);
        match system.os_ok() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn check_docker_success() {
        let commands = FakeSysCommandsPass {
            stdout_str: String::from(""),
        };

        let system = System::new(commands);
        match system.check_docker() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn check_docker_error_response() {
        let commands = FakeSysCommandsPass {
            stdout_str: String::from("Error response"),
        };

        let system = System::new(commands);
        match system.check_docker() {
            Ok(_) => assert!(true),
            Err(err) => assert_eq!(err.to_string(), "❌ Docker not running"),
        };
    }

    #[test]
    fn check_docker_cannot_connect() {
        let commands = FakeSysCommandsPass {
            stdout_str: String::from("Cannot connect"),
        };

        let system = System::new(commands);
        match system.check_docker() {
            Ok(_) => assert!(true),
            Err(err) => assert_eq!(err.to_string(), "❌ Docker not running"),
        };
    }

    #[test]
    fn check_docker_fail() {
        let commands = FakeSysCommandsFail {};

        let system = System::new(commands);
        match system.check_docker() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn create_working_dir_success() {
        let commands = FakeSysCommandsPass {
            stdout_str: String::from(""),
        };

        let system = System::new(commands);
        match system.create_working_dir(String::from("some/dir")) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn create_working_dir_fail() {
        let commands = FakeSysCommandsFail {};

        let system = System::new(commands);
        match system.create_working_dir(String::from("some/dir")) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "❌ Error creating / cleaning working dir"),
        };
    }
}
