mod is_htmx;
pub use is_htmx::*;

// mod info;
// pub use info::*;

pub fn hx_redirect(redirect_url: &str) -> (&'static str, &str) {
    ("HX-Redirect", redirect_url)
}

pub fn hx_trigger(event: &str) -> (&'static str, &str) {
    ("HX-Trigger", event)
}
