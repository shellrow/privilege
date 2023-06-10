use std::error::Error;
use std::process::Command;

pub fn get_privilege() -> Privilege {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };

    match (uid, euid) {
        (0, 0) => Privilege::Root,
        (_, 0) => Privilege::Suid,
        (_, _) => Privilege::User,
    }
}
