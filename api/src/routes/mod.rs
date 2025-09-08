mod admin;
mod config;
mod pastes;
mod statistics;
pub use admin::{
    admin_delete_paste_handler, admin_delete_report_handler, admin_get_all_reports_handler,
    admin_get_report_handler, get_auth_status_handler,
};
pub use config::get_config_handler;
pub use pastes::{
    create_paste_report_handler, paste_create_handler, paste_delete_handler, paste_get_handler,
};
pub use statistics::get_statistics_handler;
