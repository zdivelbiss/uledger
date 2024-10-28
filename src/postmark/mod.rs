use crate::{config::cfg, user_agent};
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::LazyLock;

mod transaction;
pub use transaction::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    BodySerialization(#[from] serde_json::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();

    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert(
        "X-Postmark-Server-Token",
        HeaderValue::from_static(crate::cfg().postmark.apikey.as_str()),
    );

    reqwest::Client::builder()
        .connect_timeout(cfg().network.timeout)
        .gzip(true)
        .https_only(true)
        .user_agent(user_agent())
        .default_headers(headers)
        .build()
        .expect("could not build HTTP email client")
});

#[derive(Debug, Serialize)]
pub enum MessageStream {
    #[serde(rename = "broadcast")]
    Broadcast,

    #[serde(rename = "inbound")]
    Inbound,

    #[serde(rename = "outbound")]
    Outbound,

    #[serde(rename = "verification")]
    Verification,
}

pub async fn send<K: Kind>(transaction: Transaction<K>) -> std::result::Result<(), Error> {
    HTTP_CLIENT
        .post("https://api.postmarkapp.com/email/withTemplate")
        .body(serde_json::to_string(&transaction)?)
        .send()
        .await
        .map(|response| debug!("HTTP response: {response:?}"))
        .map_err(Error::from)
}
