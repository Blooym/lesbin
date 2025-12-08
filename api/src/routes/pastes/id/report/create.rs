use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::query;
use tracing::error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportPasteRequest {
    decryption_key: String,
    reason: String,
}

pub async fn create_paste_report_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<ReportPasteRequest>,
) -> impl IntoResponse {
    if !state.reports_enabled {
        return (
            StatusCode::FORBIDDEN,
            "Reporting is not enabled for this instance.",
        )
            .into_response();
    }

    // Validate report reason and decryption key
    let decryption_key = payload.decryption_key.trim();
    let report_reason = payload.reason.trim();
    if decryption_key.is_empty() {
        return (StatusCode::BAD_REQUEST, "Decryption key must not be empty.").into_response();
    }
    if report_reason.len() < state.reports_min_length {
        return (
            StatusCode::BAD_REQUEST,
            format!(
                "Report reason must be at least {} characters long.",
                state.reports_min_length
            ),
        )
            .into_response();
    }

    // Insert the report into the database
    let result = query!(
        "INSERT INTO paste_reports (paste_id, reason, decryption_key) VALUES (?1, ?2, ?3)",
        id,
        report_reason,
        payload.decryption_key
    )
    .execute(state.database.pool())
    .await;
    match result {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(err) => {
            error!("Failed to insert paste report into database: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
