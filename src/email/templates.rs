use crate::util::VerificationToken;
use askama::Template;
use chrono::{DateTime, TimeZone};
use std::fmt::Display;

pub trait EmailTemplate: askama::Template {
    fn subject(&self) -> String;
    fn body(&self) -> String;
}

#[derive(askama::Template)]
#[template(path = "emails/verification.html")]
pub struct VerificationEmailTemplate {
    creation_datetime: String,
    proof_token: String,
}

impl EmailTemplate for VerificationEmailTemplate {
    fn subject(&self) -> String {
        format!("Your Verification Code: {}", self.proof_token)
    }

    fn body(&self) -> String {
        self.render().unwrap()
    }
}

impl VerificationEmailTemplate {
    pub fn new<TZ>(creation: DateTime<TZ>, verification_token: &VerificationToken) -> Self
    where
        TZ: TimeZone,
        TZ::Offset: Display,
    {
        Self {
            creation_datetime: creation.format("%A, %B %e at %l:%M%p %Z").to_string(),
            proof_token: verification_token.to_string(),
        }
    }
}
