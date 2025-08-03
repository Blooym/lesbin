use crate::AppState;
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::query_scalar;
use tracing::error;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsResponse {
    total_pastes: i64,
}

pub async fn get_statistics_handler(
    State(state): State<AppState>,
) -> Result<Json<StatisticsResponse>, StatusCode> {
    match query_scalar!("SELECT COUNT(*) FROM pastes")
        .fetch_one(state.database.pool())
        .await
    {
        Ok(count) => Ok(Json(StatisticsResponse {
            total_pastes: count,
        })),
        Err(err) => {
            error!("Failed to insert paste into database: {err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
