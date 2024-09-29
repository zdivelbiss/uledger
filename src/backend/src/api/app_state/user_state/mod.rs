use crate::util::EmailAddress;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub mod verify;
pub mod register;
pub mod login;

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
    fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }

    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    pub async fn try_verify_email(
        &self,
        user_id: Uuid,
        email_address: &EmailAddress,
        proof_token: Uuid,
    ) -> Result<u64, sqlx::Error> {
        query!(
            "
            DELETE FROM auth.email_verification
                WHERE (user_id, email_address, proof_token) = ($1, $2, $3)
            ;
            ",
            user_id,
            email_address.as_str(),
            proof_token
        )
        .execute(self.pool())
        .await
        .map(|r| r.rows_affected())
    }
}

impl axum::extract::FromRef<super::AppState> for UserState {
    fn from_ref(state: &super::AppState) -> Self {
        state.user_state().clone()
    }
}
