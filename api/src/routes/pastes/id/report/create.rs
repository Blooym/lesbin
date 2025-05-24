use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
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
) -> StatusCode {
    let report_reason = payload.reason.trim();

    let result = query!(
        "INSERT INTO paste_reports (pasteId, reason, decryptionKey) VALUES ($1, $2, $3)",
        id,
        report_reason,
        payload.decryption_key
    )
    .execute(state.database.pool())
    .await;
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            error!("Failed to insert paste report into database: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
