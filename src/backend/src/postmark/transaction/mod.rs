use crate::{config::cfg, postmark::MessageStream, util::EmailAddress};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt::Debug;

mod kind;
pub use kind::*;

#[derive(Debug)]
pub struct Transaction<K: Kind> {
    message_stream: MessageStream,
    from: EmailAddress,
    to: EmailAddress,
    reply_to: Option<EmailAddress>,
    track_opens: bool,

    kind: K,
}

impl<K: Kind> Transaction<K> {
    pub fn new(
        message_stream: MessageStream,
        to: &EmailAddress,
        reply_to: Option<&EmailAddress>,
        kind: K,
    ) -> Self {
        Self {
            message_stream,
            from: cfg().postmark.sender.clone(),
            to: to.clone(),
            reply_to: reply_to.cloned(),
            track_opens: true,
            kind,
        }
    }
}

impl<K: Kind> Serialize for Transaction<K> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut serializer = serializer
            .serialize_struct("Transaction", 5 + K::FIELDS)
            .unwrap();

        serializer.serialize_field("MessageStream", &self.message_stream)?;

        serializer.serialize_field("From", &self.from)?;
        serializer.serialize_field("To", &self.to)?;

        if let Some(reply_to) = self.reply_to.as_ref() {
            serializer.serialize_field("ReplyTo", &reply_to)?;
        }

        serializer.serialize_field("TrackOpens", &self.track_opens)?;

        self.kind.serialize_into(&mut serializer)?;

        serializer.end()
    }
}
