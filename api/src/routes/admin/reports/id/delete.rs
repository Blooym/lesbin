use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::query;
use tracing::error;

pub async fn admin_delete_report_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> StatusCode {
    match query!("DELETE FROM paste_reports WHERE id = $1", id)
        .execute(state.database.pool())
        .await
    {
        Ok(result) => {
            if result.rows_affected() != 0 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        }
        Err(err) => {
            error!("Failed to delete report from database: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
