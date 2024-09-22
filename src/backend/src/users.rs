use chrono::{DateTime, FixedOffset};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug)]
pub enum Role {
    Admin,
    Regular,
}

impl From<Role> for &'static str {
    fn from(value: Role) -> Self {
        match value {
            Role::Admin => "ADMIN",
            Role::Regular => "REGULAR",
        }
    }
}

pub struct User {
    id: Uuid,
    role: Role,
    email: String,
    email_confirmed_on: Option<DateTime<FixedOffset>>,
    salt: String,
    password: String,
}

pub struct Users {
    db: Pool<Postgres>,
}
