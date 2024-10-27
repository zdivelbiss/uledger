use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
};

pub struct HtmxRequest {
    is_boosted: bool,
    prompt: Option<String>,
    target: Option<String>,
    trigger: Option<String>,
    trigger_name: Option<String>,
}

impl HtmxRequest {
    fn new(headers: &HeaderMap) -> Option<Self> {
        headers.get("HX-Request").map(|_| Self {
            is_boosted: headers.get("HX-Boosted").map_or(false, |_| true),
            prompt: headers
                .get("HX-Prompt")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned),
            target: headers
                .get("HX-Target")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned),
            trigger: headers
                .get("HX-Trigger")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned),
            trigger_name: headers
                .get("HX-Trigger-Name")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned),
        })
    }

    pub fn is_boosted(&self) -> bool {
        self.is_boosted
    }

    pub fn prompt(&self) -> Option<&str> {
        self.prompt.as_deref()
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    pub fn trigger(&self) -> Option<&str> {
        self.trigger.as_deref()
    }

    pub fn trigger_name(&self) -> Option<&str> {
        self.trigger_name.as_deref()
    }
}

#[axum::async_trait]
impl<S: Sync + Send> FromRequestParts<S> for HtmxRequest {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state).await.unwrap();
        HtmxRequest::new(&headers).ok_or((StatusCode::BAD_REQUEST, "Expected an HTMX request."))
    }
}
