use crate::{config::cfg, postmark::MessageStream, util::EmailAddress};
use serde::Serialize;

mod model;
pub use model::*;

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Id {
    EmailVerification = 37491390,
}

impl Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (*self as u64).serialize(serializer)
    }
}

#[derive(Debug, Serialize)]
pub struct Template<M: model::Model> {
    #[serde(rename = "TemplateId")]
    id: Id,
    #[serde(rename = "MessageStream")]
    message_stream: MessageStream,
    #[serde(rename = "TemplateModel")]
    model: M,

    #[serde(rename = "From")]
    from: EmailAddress,
    #[serde(rename = "To")]
    to: EmailAddress,
    #[serde(rename = "ReplyTo")]
    reply_to: Option<EmailAddress>,

    #[serde(rename = "TrackOpens")]
    track_opens: bool,
}

impl<M: Model> Template<M> {
    pub fn create(to: &EmailAddress, reply_to: Option<&EmailAddress>, model: M) -> Self {
        Self {
            id: M::TEMPLATE_ID,
            model,
            from: cfg().postmark.sender.clone(),
            to: to.clone(),
            reply_to: reply_to.cloned(),
            track_opens: true,
            message_stream: M::MESSAGE_STREAM,
        }
    }
}
