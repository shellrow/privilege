pub mod shared;
pub use self::shared::*;

#[cfg(not(target_os = "windows"))]
pub mod unix;
#[cfg(not(target_os = "windows"))]
pub use self::unix::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

use std::ffi::{OsStr, OsString};
use std::io;
use std::process::ExitStatus;

pub struct Command {
    command: OsString,
    args: Vec<OsString>,
    force_prompt: bool,
    hide: bool,
    gui: bool,
}

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Command {
            command: program.as_ref().to_os_string(),
            args: vec![],
            hide: false,
            gui: false,
            force_prompt: true,
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    pub fn args<S: AsRef<OsStr>>(&mut self, args: &[S]) -> &mut Command {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn show(&mut self, val: bool) -> &mut Command {
        self.hide = !val;
        self
    }

    pub fn gui(&mut self, val: bool) -> &mut Command {
        self.gui = val;
        self
    }

    pub fn force_prompt(&mut self, val: bool) -> &mut Command {
        self.force_prompt = val;
        self
    }
    
    pub fn status(&mut self) -> io::Result<ExitStatus> {
        runas_impl(self)
    }
}
