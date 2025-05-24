use crate::{AppState, cryptography::hash_value_sha256};
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
pub struct DeletePasteRequest {
    deletion_key: String,
}

pub async fn paste_delete_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<DeletePasteRequest>,
) -> StatusCode {
    let hashed_deletion_key = hash_value_sha256(&payload.deletion_key);
    match query!(
        "DELETE FROM pastes WHERE id = $1 AND deletionKey = $2",
        id,
        hashed_deletion_key,
    )
    .execute(state.database.pool())
    .await
    {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(err) => {
            error!("Failed to delete paste from database: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
