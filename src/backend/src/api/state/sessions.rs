use crate::api::state::State;
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
pub struct SessionState(MultiplexedConnection);

impl SessionState {
    pub async fn connect() -> Result<Self> {
        use crate::cfg;

        let url = cfg().sessions_url();
        let client = Client::open(url).map_err(|_| Error::InvalidUrl { url })?;
        let mut connection = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|_| Error::ConnectionFailed { url })?;

        if let Some(db_num) = cfg().sessions_db_num() {
            cmd("SELECT")
                .arg(db_num)
                .exec_async(&mut connection)
                .await
                .map_err(|err| Error::Unknown(Box::new(err)))?;
        }

        Ok(Self(connection))
    }

    fn connection(&mut self) -> &mut MultiplexedConnection {
        &mut self.0
    }

    pub async fn gen_user_session(&mut self, user_id: Uuid, lifetime_secs: u32) -> Result<Uuid> {
        let id = Uuid::now_v7();

        cmd("SET")
            .arg(id)
            .arg(user_id)
            .arg("EX")
            .arg(lifetime_secs)
            .arg("NX")
            .exec_async(self.connection())
            .await
            .map_err(|err| Error::Unknown(Box::new(err)))?;

        Ok(id)
    }

    pub async fn get_user_session(&mut self, id: Uuid) -> Result<Option<Uuid>> {
        let opt = self
            .0
            .get(id)
            .await
            .map_err(|err| Error::Unknown(Box::new(err)))?;

        Ok(opt)
    }
}

impl axum::extract::FromRef<super::State> for SessionState {
    fn from_ref(state: &State) -> Self {
        state.session_state.clone()
    }
}
