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
pub struct GetPasteResponse {
    id: String,
    encrypted_title: String,
    encrypted_content: String,
    encrypted_syntax_type: String,
    expires_at: Option<i64>,
    created_at: i64,
}

pub async fn paste_get_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetPasteResponse>, StatusCode> {
    match query!("SELECT id, encrypted_title, encrypted_content, encrypted_syntax_type, expires_at, created_at FROM pastes WHERE id = $1 AND (expires_at IS NULL OR expires_at > strftime('%s','now'))", id)
    .fetch_optional(state.database.pool())
    .await
    {
        Ok(Some(row)) => Ok(Json(GetPasteResponse {
            id: row.id,
            encrypted_title: row.encrypted_title,
            encrypted_content: row.encrypted_content,
            encrypted_syntax_type: row.encrypted_syntax_type,
            expires_at: row.expires_at,
            created_at: row.created_at,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) =>{
            error!("Failed to get paste from database: {err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
