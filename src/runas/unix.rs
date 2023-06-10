use std::io;
use std::process;
use std::process::ExitStatus;

use crate::runas::Command;

pub(crate) const ENV_PATH: &str = "PATH";
pub(crate) const CMD_SUDO: &str = "sudo";

pub(crate) fn runas_root_sudo(cmd: &Command) -> io::Result<ExitStatus> {
    match which::which(crate::runas::CMD_SUDO) {
        Ok(_) => {
            let mut c = process::Command::new("sudo");
            if cmd.force_prompt {
                c.arg("-k");
            }
            c.arg("--").arg(&cmd.command).args(&cmd.args[..]).status()
        }
        Err(_) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Command `{}` not found", crate::runas::CMD_SUDO),
        )),
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) fn runas_root(cmd: &Command) -> io::Result<ExitStatus> {
    runas_root_sudo(cmd)
}
