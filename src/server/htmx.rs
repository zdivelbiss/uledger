pub fn is_htmx(headers: &axum::http::HeaderMap) -> bool {
    headers.get("HX-Request").is_some()
}

pub fn hx_redirect(redirect_url: &str) -> (&str, &str) {
    ("HX-Redirect", redirect_url)
}
