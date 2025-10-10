use crate::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query, query_scalar};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReportsRequest {
    page: i64,
    per_page: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReportsResponse {
    total: i64,
    page: i64,
    pages: i64,
    reports: Vec<ListReportsItem>,
}

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReportsItem {
    id: i64,
    paste_id: String,
    decryption_key: String,
    reason: String,
    created_at: i64,
}

pub async fn admin_get_all_reports_handler(
    Query(query_str): Query<ListReportsRequest>,
    State(state): State<AppState>,
) -> Result<Json<ListReportsResponse>, StatusCode> {
    let offset = (query_str.page - 1) * query_str.per_page;

    let total_reports = query_scalar("SELECT COUNT(*) AS count FROM paste_reports")
        .fetch_one(state.database.pool())
        .await
        .unwrap();

    let rows = query!( "SELECT id, paste_id, reason, decryption_key, created_at FROM paste_reports ORDER BY created_at DESC LIMIT ?1 OFFSET ?2", query_str.per_page, offset)
    .fetch_all(state.database.pool())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ListReportsResponse {
        total: total_reports,
        page: query_str.page,
        pages: (total_reports + query_str.per_page - 1) / query_str.per_page,
        reports: rows
            .into_iter()
            .map(|row| ListReportsItem {
                id: row.id,
                paste_id: row.paste_id,
                decryption_key: row.decryption_key,
                created_at: row.created_at,
                reason: row.reason,
            })
            .collect(),
    }))
}
