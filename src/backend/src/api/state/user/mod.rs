use crate::util::{EmailAddress, VerificationToken};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

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
    fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }

    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    pub async fn finalize_email_verification(
        &self,
        user_id: Uuid,
        email_address: &EmailAddress,
        proof_token: &VerificationToken,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = query!(
            "
            DELETE FROM auth.email_verification
                WHERE (user_id, email_address, proof_token) = ($1, $2, $3)
            ;
            ",
            user_id,
            email_address.as_str(),
            proof_token.to_string()
        )
        .execute(self.pool())
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Ok(false);
        } else if rows_affected > 1 {
            warn!("Dropped {rows_affected} during email validation!");

            return Ok(false);
        }

        query!(
            "
            UPDATE auth.users
                SET
                    email = $2,
                    email_verified_on = $3
                WHERE id = $1
            ;
            ",
            user_id,
            email_address.as_str(),
            chrono::Utc::now().date_naive()
        )
        .execute(self.pool())
        .await?;

        Ok(true)
    }
}

impl axum::extract::FromRef<super::AppState> for UserState {
    fn from_ref(state: &super::AppState) -> Self {
        state.user_state().clone()
    }
}
