use crate::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

pub async fn api_auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let Some(auth_header) = request
        .headers()
        .get("X-Access-Token")
        .and_then(|h| h.to_str().ok())
    else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    if state.access_token == auth_header {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
