#[derive(Debug, PartialEq)]
pub enum Privilege {
    Root,
    User,
    Suid,
}

pub fn privileged() -> bool {
    match crate::user::get_privilege() {
        Privilege::Root | Privilege::Suid => true,
        Privilege::User => false,
    }
}
