/// Represents the privilege level of the current process.
#[derive(Debug, PartialEq)]
pub enum Privilege {
    /// Indicates that the process is running with root (administrator) privileges.
    Root,
    /// Indicates that the process is running with standard user privileges.
    User,
    /// Indicates that the process is running with SUID (Set owner User ID) privileges.
    Suid,
}

/// Checks the privilege level of the current process.
///
/// * `true` if the process is running with root or SUID privileges.
/// * `false` if the process is running with standard user privileges.
pub fn privileged() -> bool {
    match crate::user::get_privilege() {
        Privilege::Root | Privilege::Suid => true,
        Privilege::User => false,
    }
}
