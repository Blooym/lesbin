mod instance;
mod paste;
pub use instance::get_config_handler;
pub use paste::{paste_create_handler, paste_delete_handler, paste_get_handler};
