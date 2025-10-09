use crate::{state::AppState, util::Commodity};
use axum::{Router, extract::Path, http::StatusCode, routing};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            routing::get(|| async { (StatusCode::OK, Commodity::get_all_serialized()) }),
        )
        .route(
            "/{currency_code}",
            routing::get(|Path(mut currency_code): Path<String>| async move {
                currency_code.make_ascii_uppercase();

                match Commodity::get_serialized(&currency_code) {
                    Some(commodity_serialized) => (StatusCode::OK, commodity_serialized),
                    None => (StatusCode::NOT_FOUND, "Unknown currency code."),
                }
            }),
        )
}
