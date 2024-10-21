use crate::{config::cfg, server::AppState};
use axum::{body::Bytes, http, response::IntoResponse, routing::get, Router};
use mini_moka::sync::Cache;
use std::{path::PathBuf, sync::LazyLock};

pub fn router() -> Router<AppState> {
    Router::new().route("/*path", get(get_cached))
}

#[derive(Debug, Clone)]
enum Asset {
    Css(Bytes),
    Png(Bytes),
    Js(Bytes),
}

impl IntoResponse for Asset {
    fn into_response(self) -> axum::response::Response {
        match self {
            Asset::Css(bytes) => (
                [(http::header::CONTENT_TYPE, "text/css; charset=utf-8")],
                bytes,
            )
                .into_response(),

            Asset::Js(bytes) => (
                [(http::header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
                bytes,
            )
                .into_response(),

            Asset::Png(bytes) => {
                ([(http::header::CONTENT_TYPE, "image/png")], bytes).into_response()
            }
        }
    }
}

static CACHE: LazyLock<Cache<PathBuf, Asset>> = LazyLock::new(|| {
    let capacity = cfg().assets.cache.capacity;
    let lifetime = cfg().assets.cache.lifetime;

    Cache::builder()
        .max_capacity(capacity)
        .time_to_live(lifetime)
        .build()
});

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("asset not found")]
    NotFound,

    #[error("asset type not supported")]
    UnsupportedAsset,

    #[error("internal server error")]
    Io(#[from] tokio::io::Error),

    #[error("internal server error")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("internal server error")]
    Grass(#[from] Box<grass::Error>),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use crate::server::internal_error;

        axum::response::Response::builder()
            .status(match &self {
                Error::NotFound => http::StatusCode::NOT_FOUND,
                Error::UnsupportedAsset => http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Error::Io(error) => internal_error(error),
                Error::Utf8(error) => internal_error(error),
                Error::Grass(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[instrument]
async fn get_cached(path: axum::extract::Path<PathBuf>) -> Result<Asset, Error> {
    trace!("Fetching asset...");

    let path = cfg().assets.path.join(&*path).canonicalize()?;

    if !path.exists() || path.is_dir() {
        return Err(Error::NotFound);
    }

    if let Some(asset) = CACHE.get(&path) {
        trace!("Served from cache.");

        return Ok(asset);
    }

    let asset = {
        let extension = path.extension().and_then(std::ffi::OsStr::to_str);
        let file = tokio::fs::read(&path).await?;

        match extension {
            Some("scss" | "sass") => {
                let mut options = grass::Options::default().style(grass::OutputStyle::Compressed);

                if let Some(load_path) = path.parent() {
                    options = options.load_path(load_path);
                }

                let file = String::from_utf8(file)?;

                trace!("Transpiling SCSS...");
                let css = grass::from_string(file, &options)?;

                Asset::Css(css.into())
            }

            Some("js") => Asset::Js(file.into()),

            Some("png") => Asset::Png(file.into()),

            _ => return Err(Error::UnsupportedAsset),
        }
    };

    #[cfg(not(debug_assertions))]
    {
        CACHE.insert(path.clone(), asset.clone());
        trace!("Cached asset.");
    }

    Ok(asset)
}
