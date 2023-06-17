use crate::global_config::GlobalConfig;
use crate::prelude::*;

#[tauri::command]
pub fn get_config(state: tauri::State<GlobalConfig>) -> &GlobalConfig {
    trace!("get_config");
    state.inner()
}
