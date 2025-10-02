use axum::http::StatusCode;

mod verification_token;
pub use verification_token::*;

mod commodity;
pub use commodity::*;

pub fn internal_error(error: impl std::error::Error) -> (StatusCode, &'static str) {
    error!("{error:?}");

    (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
}
