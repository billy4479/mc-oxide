pub use anyhow::Result;
use lazy_static::lazy_static;
pub use log::{debug, error, info, trace, warn};
pub use std::path::PathBuf;

lazy_static! {
    pub static ref CACHE_DIR: PathBuf = dirs::cache_dir().expect("no cache dir").join("mc-oxide");
    pub static ref LOGS_DIR: PathBuf = CACHE_DIR.join("logs");
    pub static ref NEW_LOG_PREFIX: String = "__latest_".into();
}
