use crate::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

pub async fn admin_auth_middleware(
    State(state): State<AppState>,
    authorization_header: TypedHeader<Authorization<Bearer>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = authorization_header.token();
    if state.admin_tokens.contains(&token.to_string()) {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
