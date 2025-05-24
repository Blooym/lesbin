use axum::http::StatusCode;

pub async fn get_auth_status_handler() -> StatusCode {
    StatusCode::OK // handled by middleware
}
