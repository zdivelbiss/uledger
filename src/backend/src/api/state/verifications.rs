use crate::{api::state::State, config::cfg, util::EmailAddress};
use redis::{aio::MultiplexedConnection, cmd, AsyncCommands, Client};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid db url: {url}")]
    InvalidUrl { url: &'static str },

    #[error("connection to db failed: {url}")]
    ConnectionFailed { url: &'static str },

    #[error("unknown error")]
    Unknown(Box<dyn std::error::Error>),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct VerificationState(MultiplexedConnection);

impl VerificationState {
    pub async fn connect() -> Result<Self> {
        use crate::cfg;

        let url = cfg().verifications_url();
        let client = Client::open(url).map_err(|_| Error::InvalidUrl { url })?;
        let mut connection = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|_| Error::ConnectionFailed { url })?;

        if let Some(db_num) = cfg().verifications_db_num() {
            cmd("SELECT")
                .arg(db_num)
                .exec_async(&mut connection)
                .await
                .unwrap();
        }

        Ok(Self(connection))
    }

    fn connection(&mut self) -> &mut MultiplexedConnection {
        &mut self.0
    }

    pub async fn gen_email_address_verification(
        &mut self,
        email_address: EmailAddress,
    ) -> Result<Uuid> {
        let token = Uuid::new_v4();

        let result = cmd("SET")
            .arg(email_address)
            .arg(token)
            .arg("EX")
            .arg(cfg().verifications_lifetime().as_secs())
            .arg("NX")
            .exec_async(self.connection())
            .await;

        match result {
            Ok(()) => Ok(token),
            Err(err) => Err(Error::Unknown(Box::new(err))),
        }
    }

    pub async fn get_email_address_verification(
        &mut self,
        email_address: EmailAddress,
    ) -> Result<Option<Uuid>> {
        let result = self.0.get(email_address).await;

        match result {
            Ok(token) => Ok(token),
            Err(err) => Err(Error::Unknown(Box::new(err))),
        }
    }
}

impl axum::extract::FromRef<super::State> for VerificationState {
    fn from_ref(state: &State) -> Self {
        state.verification_state.clone()
    }
}
