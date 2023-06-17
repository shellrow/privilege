use std::ffi::{OsStr, OsString};
use std::io;
use std::process::ExitStatus;

use crate::runas::runas_root;

/// Represents a command that can be executed with administrative privileges.
pub struct Command {
    pub(crate) command: OsString,
    pub(crate) args: Vec<OsString>,
    pub(crate) force_prompt: bool,
    pub(crate) hide: bool,
    pub(crate) gui: bool,
}

impl Command {
    /// Creates a new command with the given program.
    ///
    /// The `program` argument will be used as the command to run.
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Command {
            command: program.as_ref().to_os_string(),
            args: vec![],
            hide: false,
            gui: false,
            force_prompt: true,
        }
    }
    /// Adds an argument to the command.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.args.push(arg.as_ref().to_os_string());
        self
    }
    /// Adds multiple arguments to the command.
    pub fn args<S: AsRef<OsStr>>(&mut self, args: &[S]) -> &mut Command {
        for arg in args {
            self.arg(arg);
        }
        self
    }
    /// Determines visibility of the program.
    pub fn hide(&mut self, val: bool) -> &mut Command {
        self.hide = val;
        self
    }
    /// Determines whether the command is run with GUI elevation prompt.
    pub fn gui(&mut self, val: bool) -> &mut Command {
        self.gui = val;
        self
    }
    /// Determines whether to force the elevation prompt.
    pub fn force_prompt(&mut self, val: bool) -> &mut Command {
        self.force_prompt = val;
        self
    }
    /// Executes the command with administrative privileges and returns the exit status.
    ///
    /// This function will return an error if the command fails to start or if the process cannot be waited for.
    pub fn run(&mut self) -> io::Result<ExitStatus> {
        runas_root(self)
    }
}
