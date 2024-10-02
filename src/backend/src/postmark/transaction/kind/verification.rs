use crate::util::EmailAddress;
use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Verification {
    creation_datetime: String,
    origin_email: EmailAddress,
    proof_token: Uuid,
}

impl Verification {
    pub fn new(creation: DateTime<Utc>, origin_email: EmailAddress, proof_token: Uuid) -> Self {
        Self {
            creation_datetime: creation.format("%A, %B %e at %l:%M%p %Z").to_string(),
            origin_email,
            proof_token,
        }
    }
}

impl super::Kind for Verification {
    const FIELDS: usize = 2;

    fn serialize_into<S: SerializeStruct>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_field("TemplateId", &37491390)?;
        serializer.serialize_field("TemplateModel", self)?;

        Ok(())
    }
}
