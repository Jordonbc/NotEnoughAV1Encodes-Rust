use log::{trace, info};
use tauri::api::dialog;

use crate::config::{get_config, set_config};

#[tauri::command]
pub fn hello_world() -> &'static str {
    return "Hello, World!";
}
