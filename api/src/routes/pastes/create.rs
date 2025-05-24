use crate::{AppState, cryptography::hash_value_sha256};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use sqlx::query;
use tracing::error;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePasteRequest {
    encrypted_title: String,
    encrypted_content: String,
    expires_at: Option<i64>,
    encrypted_syntax_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePasteResponse {
    id: String,
    deletion_key: String,
}

fn generate_paste_id() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}

pub async fn paste_create_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreatePasteRequest>,
) -> Result<Json<CreatePasteResponse>, impl IntoResponse> {
    // Expiry must be provided when configured.
    if state.paste_expiry_required && payload.expires_at.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Paste must have an expiry time attached",
        )
            .into_response());
    }
    // Expiry timestamps must be within a valid range.
    if let Some(expires_at) = payload.expires_at {
        if expires_at > (Utc::now() + state.paste_max_expiry).timestamp() {
            return Err((StatusCode::BAD_REQUEST, "Invalid expiration timestamp").into_response());
        }
    }
    // Paste title/content cannot be empty.
    if payload.encrypted_title.is_empty() || payload.encrypted_content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "TItle or content was empty").into_response());
    }
    // Paste title/content cannot exceed max paste size.
    if payload.encrypted_title.len() + payload.encrypted_content.len()
        > state.paste_max_size.0 as usize
    {
        return Err((StatusCode::PAYLOAD_TOO_LARGE, "Payload too large").into_response());
    }

    let id = generate_paste_id();
    let deletion_key = Uuid::new_v4().to_string();
    let hashed_deletion_key = hash_value_sha256(&deletion_key);
    if let Err(err) = query!(
        "INSERT INTO pastes (id, encryptedTitle, encryptedContent, encryptedSyntaxType, deletionKey, expiresAt) VALUES ($1, $2, $3, $4, $5, $6)",
        id,
        payload.encrypted_title,
        payload.encrypted_content,
        payload.encrypted_syntax_type,
        hashed_deletion_key,
        payload.expires_at,
    ).execute(state.database.pool()).await {
        error!("Failed to insert paste into database: {err:?}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    };

    Ok(Json(CreatePasteResponse { id, deletion_key }))
}
