use crate::AppState;
use axum::{Json, extract::State};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    paste: PasteConfiguration,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteConfiguration {
    max_size_bytes: u64,
    max_expiry: u64,
    expiry_required: bool,
}

pub async fn get_config_handler(State(state): State<AppState>) -> Json<GetConfigurationResponse> {
    Json(GetConfigurationResponse {
        paste: PasteConfiguration {
            max_expiry: state.paste_max_expiry.as_secs(),
            expiry_required: state.paste_expiry_required,
            max_size_bytes: state.paste_max_size.0,
        },
    })
}
