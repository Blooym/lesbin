use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::Serialize;
use sqlx::query;
use tracing::error;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPasteResponse {
    id: String,
    encrypted_title: String,
    encrypted_content: String,
    expires_at: Option<i64>,
    syntax_type: String,
    created_at: i64,
}

pub async fn paste_get_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetPasteResponse>, StatusCode> {
    let now = Utc::now().timestamp();
    match query!("SELECT id, encryptedTitle, encryptedContent, expiresAt, syntaxType, createdAt FROM pastes WHERE id = $1 AND (expiresAt IS NULL OR expiresAt > $2)", id, now)
    .fetch_optional(state.database.pool())
    .await
    {
        Ok(Some(row)) => Ok(Json(GetPasteResponse {
            id: row.id,
            encrypted_title: row.encryptedTitle,
            encrypted_content: row.encryptedContent,
            expires_at: row.expiresAt,
            syntax_type: row.syntaxType,
            created_at: row.createdAt,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) =>{
            error!("Failed to get paste from database: {err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
