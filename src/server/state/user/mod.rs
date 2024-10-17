use sqlx::{Pool, Postgres};

pub mod login;
pub mod register;
pub mod verify;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive, Serialize, Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
#[repr(i16)]
pub enum Role {
    Admin = 0,

    #[default]
    Regular = 100,
}

#[derive(Clone)]
pub struct UserState {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<super::AppState> for UserState {
    fn from_ref(input: &super::AppState) -> Self {
        Self { db: input.db() }
    }
}
