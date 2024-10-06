#[derive(Debug)]
pub enum Role {
    Admin,
    Regular,
}

impl From<Role> for &'static str {
    fn from(value: Role) -> Self {
        match value {
            Role::Admin => "ADM",
            Role::Regular => "REG",
        }
    }
}