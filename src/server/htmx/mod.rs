mod is_htmx;
pub use is_htmx::*;

// mod info;
// pub use info::*;

pub fn hx_redirect(redirect_url: &str) -> (&str, &str) {
    ("HX-Redirect", redirect_url)
}
