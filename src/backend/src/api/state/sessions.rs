use crate::api::state::State;
use anyhow::Result;
use redis::{aio::MultiplexedConnection, cmd, AsyncCommands, Client};
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionState(MultiplexedConnection);

impl SessionState {
    pub async fn connect() -> Result<Self> {
        use crate::cfg;

        let client = Client::open(cfg().sessions_url()).unwrap();
        let mut connection = client.get_multiplexed_async_connection().await?;

        if let Some(db_num) = cfg().sessions_db_num() {
            cmd("SELECT")
                .arg(db_num)
                .exec_async(&mut connection)
                .await?;
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
            .await?;

        Ok(id)
    }

    pub async fn get_user_session(&mut self, id: Uuid) -> Result<Option<Uuid>> {
        let opt = self.0.get(id).await?;

        Ok(opt)
    }
}

impl axum::extract::FromRef<super::State> for SessionState {
    fn from_ref(state: &State) -> Self {
        state.session_state.clone()
    }
}
