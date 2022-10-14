use clap::Parser;
use std::fmt;
use std::process::{Command, Output};
use crate::{cli_args::CliArgs, manifest::Manifest};

pub fn init_system(manifest: Manifest) -> Result<(), SystemError> {
    let commands = LumberStackSysCommands{};
    return System::new(manifest.clone(), commands ).run();
}

pub struct System<T:SysCommands>{
    manifest: Manifest,
    command_runner: T
}

pub trait SysCommands {
    fn app_version(&self, bin_name: &str) -> Result<Output, std::io::Error>;
    fn docker_ps(&self)  -> Result<Output, std::io::Error>;
}

impl <T: SysCommands> System<T>{
    pub fn new(manifest: Manifest, command_runner: T) -> Self { Self { manifest, command_runner } }

    pub fn run(&self) -> Result<(), SystemError>{
        let args = CliArgs::parse();
        if !args.skip_checks {
            self.os_ok()?;
            self.has_required_bin("yarn")?;
            self.check_docker()?;
            self.has_required_bin("node")?;
            self.check_node_version()?;
        }

      self.create_working_dir()?;
      Ok(())
    }

    fn os_ok(&self) -> Result<(), SystemError> {
        if cfg!(windows) {
            return Err(SystemError{message: format!("❌ Windows is not supported at this time") });
        }

        return Ok(());
    }

    fn check_node_version(&self) -> Result<(), SystemError> {
        match self.has_required_bin("node") {
            Ok(output) => {
                if !output.contains("v14") {
                    return Err(SystemError{message: format!("❌ node v14 required but found: {}", output) });
                }
                Ok(())
            }
            Err(err) =>{ return Err(err) }
        }
    }

    fn has_required_bin(&self, bin_name: &str) -> Result<String, SystemError> {
        match self.command_runner.app_version(bin_name) {
            Ok(output) => { Ok(String::from_utf8(output.stdout).unwrap()) }
            Err(_) =>{
                return Err(SystemError{message: format!("❌ {} not found but required", bin_name) });
            }
        }
    }

    fn check_docker(&self) -> Result<(), SystemError> {
        self.has_required_bin("docker")?;

        match self.command_runner.docker_ps() {
            Err(_) => {
                return Err(SystemError{message: format!("❌ Docker not running") })
            }
            Ok(output) => {
                let message = String::from_utf8(output.stderr).unwrap();
                if message.contains("Error response") || message.contains("Cannot connect") {
                    return Err(SystemError{message: format!("❌ Docker not running") });
                }
            }
        }
        return Ok(())
    }

    fn create_working_dir(&self) -> Result<(), SystemError> {
        let workdir = self.manifest.workdir.clone().unwrap_or_default();
        match fs_extra::dir::create_all(workdir, false) {
            Ok(_) => { return Ok(()) }
            Err(_) =>{
                return Err(SystemError{message: format!("Error creating / cleaning working dir") });
            }
        }
    }
}

pub struct SystemError {
    pub message: String
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct LumberStackSysCommands;

impl SysCommands for LumberStackSysCommands{
    fn app_version(&self, bin_name: &str) -> Result<Output, std::io::Error> {
        return Command::new(bin_name).arg("--version").output();
    }

    fn docker_ps(&self)  -> Result<Output, std::io::Error> {
        return Command::new("docker").arg("ps").output();
    }
}

#[cfg(test)]
mod tests {
    use crate::{manifest};
    use manifest::Manifest;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus};
    use super::*;

    struct FakeSysCommandsPass;
    struct FakeSysCommandsFail;

    impl SysCommands for FakeSysCommandsPass{
        fn app_version(&self, bin_name: &str) -> Result<Output, std::io::Error> {
            let output = Output{
                status: ExitStatus::from_raw(0x007f),
                stdout: Vec::from(bin_name.as_bytes()),
                stderr: Vec::new()
            };
            return Ok(output);
        }

        fn docker_ps(&self)  -> Result<Output, std::io::Error> {
            let output = Output{
                status: ExitStatus::from_raw(0x007f),
                stdout: Vec::from("docker".as_bytes()),
                stderr: Vec::new()
            };
            return Ok(output);
        }
    }

    impl SysCommands for FakeSysCommandsFail{
        fn app_version(&self, _bin_name: &str) -> Result<Output, std::io::Error> {
            let e = std::io::Error::new(std::io::ErrorKind::Other, "BOOM");
            return Err(e);
        }

        fn docker_ps(&self)  -> Result<Output, std::io::Error> {
            let e = std::io::Error::new(std::io::ErrorKind::Other, "BOOM");
            return Err(e);
        }
    }

    #[test]
    fn has_required_bin_success() {
        let manifest = Manifest::load().unwrap();
        let commands = FakeSysCommandsPass{};

        let system = System::new(manifest.clone(), commands);
        match system.has_required_bin("yarn") {
            Ok(value) => assert_eq!(value, "yarn"),
            Err(_) => return assert!(false),
        };
    }

    #[test]
    fn has_required_bin_fail() {
        let manifest = Manifest::load().unwrap();
        let commands = FakeSysCommandsFail{};

        let system = System::new(manifest.clone(), commands);
        match system.has_required_bin("yarn") {
            Ok(_) => assert!(false),
            Err(_) => return assert!(true),
        };
    }
}
