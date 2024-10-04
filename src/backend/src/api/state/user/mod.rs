use sqlx::{Pool, Postgres};

pub mod login;
pub mod register;
pub mod verify;

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

#[derive(Clone)]
pub struct UserState(Pool<Postgres>);

impl UserState {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }
}

impl axum::extract::FromRef<super::AppState> for UserState {
    fn from_ref(state: &super::AppState) -> Self {
        state.user_state().clone()
    }
}
