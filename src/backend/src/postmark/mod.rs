use crate::{config::cfg, user_agent};
use reqwest::header::HeaderMap;
use serde::Serialize;
use std::sync::LazyLock;

mod template;
pub use template::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not serialize email body")]
    BodySerialization,

    #[error("http request error")]
    Http(#[from] reqwest::Error),
}

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();

    headers.insert("Accept", "application/json".try_into().unwrap());
    headers.insert("Content-Type", "application/json".try_into().unwrap());
    headers.insert(
        "X-Postmark-Server-Token",
        crate::cfg().postmark.apikey.as_str().try_into().unwrap(),
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
    #[serde(rename = "verification")]
    Verification,
}

pub async fn send<M: Model>(template: Template<M>) -> std::result::Result<(), Error> {
    let Ok(body) = serde_json::to_string(&template) else {
        return Err(Error::BodySerialization);
    };

    let http_post_result = HTTP_CLIENT
        .post("https://api.postmarkapp.com/email/withTemplate")
        .body(body)
        .send()
        .await;

    match http_post_result {
        Ok(response) => debug!("HTTP response: {response:?}"),
        Err(err) => debug!("HTTP response: {err:?}"),
    }

    Ok(())
}
