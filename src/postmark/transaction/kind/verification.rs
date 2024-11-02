use crate::VerificationToken;
use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct Verification {
    creation_datetime: String,
    proof_token: String,
}

impl Verification {
    pub fn new(creation: DateTime<Utc>, token: VerificationToken) -> Self {
        let creation_datetime = creation.format("%A, %B %e at %l:%M%p %Z").to_string();

        let mut proof_token = token.to_string();
        proof_token.insert(VerificationToken::SIZE, ' ');

        Self {
            creation_datetime,
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
