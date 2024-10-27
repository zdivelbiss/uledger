use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};

pub struct IsHtmx(bool);

impl IsHtmx {
    fn new(headers: &HeaderMap) -> Self {
        Self(headers.get("HX-Request").map_or(false, |_| true))
    }
}

impl std::ops::Deref for IsHtmx {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::async_trait]
impl<S: Sync + Send> FromRequestParts<S> for IsHtmx {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(IsHtmx::new(
            &HeaderMap::from_request_parts(parts, state).await.unwrap(),
        ))
    }
}
