use crate::{
    api::app_state::{user_state::UserState, AppState},
    util::EmailAddress,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware::{from_fn, FromFnLayer},
    response::IntoResponse,
    routing::post,
    Json,
};
use base64::Engine;
use serde::Deserialize;
use uuid::Uuid;

mod verify;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new().nest("/:user_id/verify", verify::routes())
}
