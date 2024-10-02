use crate::{
    postmark::{template::Id, MessageStream},
    util::EmailAddress,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt::Debug;
use uuid::Uuid;

pub trait Model: Debug + Serialize {
    const TEMPLATE_ID: Id;
    const MESSAGE_STREAM: MessageStream;
}

#[derive(Debug, Serialize)]
pub struct VerificationModel {
    creation_datetime: String,
    origin_email: EmailAddress,
    proof_token: Uuid,
}

impl VerificationModel {
    pub fn new(creation: DateTime<Utc>, origin_email: EmailAddress, proof_token: Uuid) -> Self {
        Self {
            creation_datetime: creation.format("%A, %B %e at %l:%M%p %Z").to_string(),
            origin_email,
            proof_token,
        }
    }
}

impl Model for VerificationModel {
    const TEMPLATE_ID: Id = Id::EmailVerification;
    const MESSAGE_STREAM: MessageStream = MessageStream::Verification;
}
