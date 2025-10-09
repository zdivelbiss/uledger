use crate::{state::AppState, util::Currency};
use axum::{Router, extract::Path, http::StatusCode, routing};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            routing::get(|| async { (StatusCode::OK, Currency::get_all_serialized()) }),
        )
        .route(
            "/{iso_code}",
            routing::get(|Path(mut iso_code): Path<String>| async move {
                iso_code.make_ascii_uppercase();

                match Currency::get_serialized(&iso_code) {
                    Some(currency_serialized) => (StatusCode::OK, currency_serialized),
                    None => (StatusCode::NOT_FOUND, "Unknown currency code."),
                }
            }),
        )
}
