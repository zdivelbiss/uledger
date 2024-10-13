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
    let search_path = crate::cfg().styles.clone();
    let mut cache = CssCache::new();
    let grass_options = grass::Options::default().style(grass::OutputStyle::Compressed);

    debug!("Caching stylesheets @ {search_path:?}");
    cache_styles(&mut cache, &search_path, &grass_options).expect("failed to cache styles");

    debug!("Compiled stylesheets: {:?}", cache.keys());

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
                    |css_string| Css::from(css_string.as_str()).into_response(),
                )
        }),
    )
}

fn cache_styles(
    cache: &mut CssCache,
    search_path: impl AsRef<Path>,
    grass_options: &grass::Options,
) -> std::io::Result<()> {
    fn cache_styles_inner(
        cache: &mut CssCache,
        search_path: impl AsRef<Path>,
        current_path: impl AsRef<Path>,
        grass_options: &grass::Options,
    ) -> std::io::Result<()> {
        for entry in read_dir(current_path)? {
            let entry_path = entry?.path();

            if entry_path.is_dir() {
                cache_styles_inner(
                    cache,
                    search_path.as_ref(),
                    entry_path.as_path(),
                    grass_options,
                )?;
            } else {
                debug!("Compiling: {entry_path:?}");

                let scss = std::fs::read_to_string(entry_path.as_path())?;
                match grass::from_string(scss, grass_options) {
                    Ok(css) => {
                        let relativel_path = entry_path.strip_prefix(search_path.as_ref()).unwrap();
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

    cache_styles_inner(
        cache,
        search_path.as_ref(),
        search_path.as_ref(),
        grass_options,
    )
}
