use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use sqlx::query;
use tracing::error;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReportsResponse {
    id: i64,
    paste_id: String,
    decryption_key: String,
    reason: String,
    created_at: i64,
}

pub async fn admin_get_report_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ListReportsResponse>, StatusCode> {
    match query!(
        "SELECT id, pasteId, reason, decryptionKey, createdAt FROM paste_reports WHERE id = $1",
        id
    )
    .fetch_optional(state.database.pool())
    .await
    {
        Ok(result) => match result {
            Some(report) => Ok(Json(ListReportsResponse {
                id: report.id,
                paste_id: report.pasteId,
                decryption_key: report.decryptionKey,
                reason: report.reason,
                created_at: report.createdAt,
            })),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(err) => {
            error!("Failed to get report from database: {err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
