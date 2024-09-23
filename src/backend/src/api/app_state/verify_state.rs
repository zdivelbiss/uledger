use crate::{api::app_state::AppState, config::cfg, util::EmailAddress};
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
pub struct VerifyState(MultiplexedConnection);

impl VerifyState {
    pub async fn connect() -> Result<Self> {
        use crate::cfg;

        let url = cfg().verification.url.as_str();
        let client = Client::open(url).map_err(|_| Error::InvalidUrl { url })?;
        let mut connection = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|_| Error::ConnectionFailed { url })?;

        if let Some(namespace) = cfg().verification.namespace {
            cmd("SELECT")
                .arg(namespace)
                .exec_async(&mut connection)
                .await
                .unwrap();
        }

        Ok(Self(connection))
    }

    pub async fn gen_email_verify(&self, email_address: &EmailAddress) -> Result<Uuid> {
        let mut conn = self.0.clone();

        let set_result = cmd("SET")
            .arg(email_address)
            .arg(Uuid::new_v4())
            .arg("EX")
            .arg(cfg().verification.timeout.as_secs())
            .arg("NX")
            .exec_async(&mut conn)
            .await;

        match set_result {
            Ok(()) => Ok(Uuid::new_v4()),
            Err(err) => Err(Error::Unknown(Box::new(err))),
        }
    }

    pub async fn get_email_verify(&self, email_address: &EmailAddress) -> Result<Option<Uuid>> {
        let mut conn = self.0.clone();

        match conn.get(email_address).await {
            Ok(token) => Ok(token),
            Err(err) => Err(Error::Unknown(Box::new(err))),
        }
    }
}

impl axum::extract::FromRef<super::AppState> for VerifyState {
    fn from_ref(state: &AppState) -> Self {
        state.2.clone()
    }
}
