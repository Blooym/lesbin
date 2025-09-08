use crate::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderName, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{self},
};

pub struct AccessTokenHeader(String);

impl AccessTokenHeader {
    pub fn token(&self) -> &str {
        &self.0
    }
}

impl headers::Header for AccessTokenHeader {
    fn name() -> &'static headers::HeaderName {
        static ACCESS_TOKEN_HEADER: HeaderName = HeaderName::from_static("x-access-token");
        &ACCESS_TOKEN_HEADER
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = axum::http::HeaderValue::from_str(&self.0) {
            values.extend(std::iter::once(value));
        }
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i headers::HeaderValue>,
    {
        Ok(Self(
            values
                .next()
                .ok_or(headers::Error::invalid())?
                .to_str()
                .map_err(|_| headers::Error::invalid())?
                .to_string(),
        ))
    }
}

pub async fn api_auth_middleware(
    State(state): State<AppState>,
    authorization_header: TypedHeader<AccessTokenHeader>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if state.access_token == authorization_header.token() {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
