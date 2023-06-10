use std::error::Error;
use std::process::Command;

pub fn privileged() -> bool {
    let user_privilege = _is_elevated();
    match user_privilege {
        RunningAs::Root => true,
        RunningAs::User => false,
        RunningAs::Suid => true,
    }
}

pub fn _is_elevated() -> RunningAs {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };

    match (uid, euid) {
        (0, 0) => Root,
        (_, 0) => Suid,
        (_, _) => User,
    }
}
