use std::env;
use std::ffi::{CString, OsString};
use std::io;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::ptr;

use libc::{fcntl, fileno, waitpid, EINTR, F_GETOWN};
use security_framework_sys::authorization::{
    errAuthorizationSuccess, kAuthorizationFlagDefaults, kAuthorizationFlagDestroyRights,
    AuthorizationCreate, AuthorizationExecuteWithPrivileges, AuthorizationFree, AuthorizationRef,
};

use crate::runas::Command;

fn get_exe_path<P: AsRef<Path>>(exe_name: P) -> Option<PathBuf> {
    let exe_name = exe_name.as_ref();
    if exe_name.has_root() {
        return Some(exe_name.into());
    }

    env::var_os(crate::runas::ENV_PATH).and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

macro_rules! make_cstring {
    ($s:expr) => {
        match CString::new($s.as_bytes()) {
            Ok(s) => s,
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::Other, "null byte in string"));
            }
        }
    };
}

unsafe fn gui_runas(prog: *const i8, argv: *const *const i8) -> i32 {
    let mut authref: AuthorizationRef = ptr::null_mut();
    let mut pipe: *mut libc::FILE = ptr::null_mut();

    if AuthorizationCreate(
        ptr::null(),
        ptr::null(),
        kAuthorizationFlagDefaults,
        &mut authref,
    ) != errAuthorizationSuccess
    {
        return -1;
    }
    if AuthorizationExecuteWithPrivileges(
        authref,
        prog,
        kAuthorizationFlagDefaults,
        argv as *const *mut _,
        &mut pipe,
    ) != errAuthorizationSuccess
    {
        AuthorizationFree(authref, kAuthorizationFlagDestroyRights);
        return -1;
    }

    let pid = fcntl(fileno(pipe), F_GETOWN, 0);
    let mut status = 0;
    loop {
        let r = waitpid(pid, &mut status, 0);
        if r == -1 && io::Error::last_os_error().raw_os_error() == Some(EINTR) {
            continue;
        } else {
            break;
        }
    }

    AuthorizationFree(authref, kAuthorizationFlagDestroyRights);
    status
}

fn runas_root_gui(cmd: &Command) -> io::Result<ExitStatus> {
    let exe: OsString = match get_exe_path(&cmd.command) {
        Some(exe) => exe.into(),
        None => unsafe {
            return Ok(mem::transmute(!0));
        },
    };
    let prog = make_cstring!(exe);
    let mut args = vec![];
    for arg in cmd.args.iter() {
        args.push(make_cstring!(arg))
    }
    let mut argv: Vec<_> = args.iter().map(|x| x.as_ptr()).collect();
    argv.push(ptr::null());

    unsafe { Ok(mem::transmute(gui_runas(prog.as_ptr(), argv.as_ptr()))) }
}

pub fn runas_root(cmd: &Command) -> io::Result<ExitStatus> {
    if cmd.gui {
        runas_root_gui(cmd)
    } else {
        crate::runas::runas_root_sudo(cmd)
    }
}
