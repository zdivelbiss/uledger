use crate::{config::cfg, server::internal_error, server::state::App};
use axum::{
    body::Bytes,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use mini_moka::sync::Cache;
use std::{path::PathBuf, sync::LazyLock};

pub fn router() -> Router<App> {
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
        let (mime, bytes) = match self {
            Asset::Css(bytes) => (mime::TEXT_CSS_UTF_8, bytes),
            Asset::Js(bytes) => (mime::TEXT_JAVASCRIPT, bytes),
            Asset::Png(bytes) => (mime::IMAGE_PNG, bytes),
        };

        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, mime.as_ref())],
            bytes,
        )
            .into_response()
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
    #[error("Asset not found.")]
    NotFound,

    #[error("Asset type not supported.")]
    UnsupportedAsset,

    #[error(transparent)]
    Io(tokio::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Grass(#[from] Box<grass::Error>),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::Io(error),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match &self {
            Error::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),

            Error::UnsupportedAsset => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string()).into_response()
            }

            error => internal_error(error).into_response(),
        }
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
