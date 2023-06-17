// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod global_config;
mod logger;
mod prelude;
mod tauri_interface;

use crate::{prelude::*, tauri_interface::*};
use global_config::GlobalConfig;
use logger::init_logger;
use tauri::generate_handler;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger().await?;
    let global_config = GlobalConfig::load()?;

    tauri::Builder::default()
        .manage(global_config)
        .invoke_handler(generate_handler![get_config])
        .run(tauri::generate_context!())?;

    Ok(())
}
