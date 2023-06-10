#[derive(Debug, PartialEq)]
pub enum Privilege {
    Root,
    User,
    Suid,
}

pub struct User {
    pub uid: u32,
    pub gid: u32,
    pub privilege: Privilege,
}
