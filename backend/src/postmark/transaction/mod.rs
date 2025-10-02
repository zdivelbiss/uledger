use crate::VerificationToken;
use crate::{config::cfg, postmark::MessageStream};
use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt::Debug;

mod kind;
pub use kind::*;

#[derive(Debug)]
pub struct Transaction<'a, K: Kind> {
    message_stream: MessageStream,
    to: &'a str,
    track_opens: bool,

    kind: K,
}

impl<'a, K: Kind> Transaction<'a, K> {
    pub fn new(message_stream: MessageStream, to: &'a str, kind: K) -> Self {
        Self {
            message_stream,
            to,
            track_opens: true,
            kind,
        }
    }
}

impl<K: Kind> Serialize for Transaction<'_, K> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut serializer = serializer
            .serialize_struct("Transaction", 5 + K::FIELDS)
            .unwrap();

        serializer.serialize_field("MessageStream", &self.message_stream)?;

        serializer.serialize_field("From", &cfg().postmark.sender)?;
        serializer.serialize_field("To", &self.to)?;

        serializer.serialize_field("TrackOpens", &self.track_opens)?;

        self.kind.serialize_into(&mut serializer)?;

        serializer.end()
    }
}

impl<'a> Transaction<'a, Verification> {
    pub fn verification(
        to: &'a str,
        creation: DateTime<Utc>,
        proof_token: VerificationToken,
    ) -> Self {
        Self::new(
            MessageStream::Verification,
            to,
            Verification::new(creation, proof_token),
        )
    }
}
