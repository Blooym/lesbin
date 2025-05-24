use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::query;
use tracing::error;

pub async fn admin_delete_paste_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> StatusCode {
    match query!("DELETE FROM pastes WHERE id = $1", id)
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
