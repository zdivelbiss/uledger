use crate::server::state::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_extra::response::Css;
use std::{
    collections::HashMap,
    fs::read_dir,
    path::{Path, PathBuf},
};
use tokio::sync::OnceCell;

type CssCache = HashMap<PathBuf, String>;

static CSS_CACHE: OnceCell<CssCache> = OnceCell::const_new();

pub fn router() -> Router<AppState> {
    let cache = cache_styles().expect("failed to cache styles");
    CSS_CACHE.set(cache).expect("CSS cache already initialized");

    Router::new().route(
        "/*path",
        get(|path: axum::extract::Path<PathBuf>| async move {
            CSS_CACHE
                .get()
                .expect("SCSS cache has not been compiled")
                .get(path.as_path())
                .map_or_else(
                    || (StatusCode::NOT_FOUND, "Stylesheet not found.").into_response(),
                    |css| Css::from(css.as_str()).into_response(),
                )
        }),
    )
}

fn cache_styles() -> std::io::Result<CssCache> {
    fn cache_styles_inner(
        cache: &mut CssCache,
        base_path: impl AsRef<Path>,
        next_path: impl AsRef<Path>,
        grass_options: &grass::Options,
    ) -> std::io::Result<()> {
        for entry in read_dir(next_path.as_ref())? {
            let entry_path = entry?.path();

            if entry_path.is_dir() {
                cache_styles_inner(
                    cache,
                    base_path.as_ref(),
                    entry_path.as_path(),
                    grass_options,
                )?;
            } else {
                debug!("Compiling: {entry_path:?}");

                let scss = std::fs::read_to_string(entry_path.as_path())?;
                match grass::from_string(scss, grass_options) {
                    Ok(css) => {
                        let relativel_path = entry_path.strip_prefix(next_path.as_ref()).unwrap();
                        cache.insert(relativel_path.to_path_buf(), css);
                    }

                    Err(error) => {
                        error!("{error:?}");
                    }
                };
            }
        }

        Ok(())
    }

    let mut cache = CssCache::new();
    let search_path = crate::cfg().styles.clone();
    let grass_options = grass::Options::default().style(grass::OutputStyle::Compressed);

    debug!("Caching stylesheets @ {search_path:?}");
    cache_styles_inner(&mut cache, &search_path, &search_path, &grass_options)?;

    debug!("Compiled stylesheets: {:?}", cache.keys());

    Ok(cache)
}
