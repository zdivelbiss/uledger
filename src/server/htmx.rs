pub const fn hx_redirect(redirect_url: &str) -> (&'static str, &str) {
    ("HX-Redirect", redirect_url)
}

pub const fn hx_trigger(event: &str) -> (&'static str, &str) {
    ("HX-Trigger", event)
}
