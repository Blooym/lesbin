use crate::{AppState, cryptography::hash_value_sha256};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sqlx::query;
use tracing::error;

pub async fn paste_delete_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    authorization_header: TypedHeader<Authorization<Bearer>>,
) -> StatusCode {
    let hashed_deletion_key = hash_value_sha256(authorization_header.token());
    match query!(
        "DELETE FROM pastes WHERE id = ?1 AND deletion_key_hash = ?2",
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
