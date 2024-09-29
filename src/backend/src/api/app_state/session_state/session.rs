use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    user_id: Uuid,
    user_agent: Option<String>,
}

impl Session {
    pub fn new(user_id: Uuid, user_agent: Option<String>) -> Self {
        Self {
            user_id,
            user_agent,
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }
}
