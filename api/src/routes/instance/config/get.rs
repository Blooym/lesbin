use crate::AppState;
use axum::{Json, extract::State};
use email_address::EmailAddress;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    paste: PasteConfiguration,
    report: ReportConfiguration,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteConfiguration {
    max_size_bytes: u64,
    max_expiry: u64,
    expiry_required: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportConfiguration {
    pub email: Option<EmailAddress>,
}

pub async fn get_config_handler(State(state): State<AppState>) -> Json<GetConfigurationResponse> {
    Json(GetConfigurationResponse {
        paste: PasteConfiguration {
            max_expiry: state.paste_max_expiry.as_secs(),
            expiry_required: state.paste_expiry_required,
            max_size_bytes: state.paste_max_size.0,
        },
        report: ReportConfiguration {
            email: state.report_email,
        },
    })
}
