use axum::{http::StatusCode, response::IntoResponse};

pub fn email_in_use() -> impl IntoResponse {
    (StatusCode::CONFLICT, "Email already in use.")
}

pub fn user_not_exists() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "User does not exist.")
}

pub fn not_authenticated() -> impl IntoResponse {
    (StatusCode::UNAUTHORIZED, "You must authenticate.")
}

pub fn internal_error(error: impl std::fmt::Debug) -> impl IntoResponse {
    error!("{error:?}");

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error; please contact: support@uledger.me",
    )
}
