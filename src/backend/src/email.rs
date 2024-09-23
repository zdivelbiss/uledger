use crate::{config::cfg, user_agent, util::EmailAddress};
use chrono::{FixedOffset, NaiveDateTime, Utc};
use core::error;
use reqwest::header::HeaderMap;
use serde::Serialize;
use std::{borrow::Cow, sync::LazyLock};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not serialize email body")]
    BodySerialization,

    #[error("http request error")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

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

#[cfg(debug_assertions)]
pub static TEST_ENDPOINT: LazyLock<EmailAddress> =
    LazyLock::new(|| EmailAddress::new("test@blackhole.postmarkapp.com").unwrap());

#[derive(Debug, Serialize)]
enum MessageStream {
    #[serde(rename = "verification")]
    Verification,
}

#[derive(Debug, Serialize)]
struct Email<'a> {
    #[serde(rename = "From")]
    from: Cow<'a, str>,

    #[serde(rename = "To")]
    to: Cow<'a, str>,

    #[serde(rename = "Subject")]
    subject: Cow<'a, str>,

    #[serde(rename = "HtmlBody")]
    html_body: Cow<'a, str>,

    #[serde(rename = "MessageStream")]
    message_stream: MessageStream,
}

async fn send(email: Email<'_>) -> Result<()> {
    let body = serde_json::to_string(&email).map_err(|_| Error::BodySerialization)?;

    let http_post_result = HTTP_CLIENT
        .post("https://api.postmarkapp.com/email")
        .body(body)
        .send()
        .await;

    match http_post_result {
        Ok(response) => debug!("HTTP response: {response:?}"),
        Err(err) => debug!("HTTP response: {err:?}"),
    }

    Ok(())
}

pub async fn send_verification(to: &EmailAddress, token: Uuid) -> Result<()> {
    let now_cst = Utc::now().with_timezone(&FixedOffset::west_opt(5 + 3600).unwrap());

    let html_body = format!(
        r#"
An email verification was requested on {}. If you recognize this request, please click on the following link to verify your account:

<a href="https://uledger.me/api/v1/auth/verify?email={to}&token={token}">Verify your account...</a>

If you don't recognize this request, <strong>please notify support immediately</strong>:

TODO
"#,
        now_cst.format("%a, %b %e at %H:%M")
    );

    let email = Email {
        from: cfg().postmark.sender.as_str().into(),
        to: to.as_str().into(),
        subject: "Verify your email for µLedger".into(),
        html_body: html_body.into(),
        message_stream: MessageStream::Verification,
    };

    send(email).await
}
