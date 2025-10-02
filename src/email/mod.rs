use crate::{config::CONFIG, email::templates::EmailTemplate};
use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MessageBuilder, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use std::sync::{LazyLock, OnceLock};

pub mod templates;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SendFail(#[from] lettre::transport::smtp::Error),
}

static SMTP_TRANSPORT: OnceLock<AsyncSmtpTransport<Tokio1Executor>> = OnceLock::new();
static SMTP_MESSAGE_TEMPLATE: LazyLock<MessageBuilder> = LazyLock::new(|| {
    let from = CONFIG
        .smtp
        .from
        .parse::<Mailbox>()
        .expect("invalid `SMTP_FROM` value");
    let replyto = CONFIG.smtp.replyto.as_ref().map(|replyto| {
        replyto
            .parse::<Mailbox>()
            .expect("invalid `SMTP_FROM` value")
    });

    Message::builder()
        .from(from.clone())
        .reply_to(replyto.unwrap_or(from))
        .header(ContentType::TEXT_HTML)
});

pub fn init_smtp_transport() {
    let credentials = Credentials::new(CONFIG.smtp.user.clone(), CONFIG.smtp.pass.clone());

    debug!(
        "Connecting to SMTP host: {}:{}",
        &CONFIG.smtp.host, CONFIG.smtp.port
    );

    let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&CONFIG.smtp.host)
        .expect("failed to connect to SMTP host")
        .port(CONFIG.smtp.port)
        .credentials(credentials)
        .build();

    debug!("Connected to SMTP host.");

    SMTP_TRANSPORT
        .set(smtp_transport)
        .expect("SMTP connection already initialized");
}

pub async fn send(to: Address, template: impl EmailTemplate) -> Result<(), Error> {
    let message = SMTP_MESSAGE_TEMPLATE
        .clone()
        .to(to.into())
        .subject(template.subject())
        .body(template.body())
        .expect("failed to build email template");

    SMTP_TRANSPORT
        .get()
        .expect("SMTP not initialized")
        .send(message)
        .await?;

    Ok(())
}
