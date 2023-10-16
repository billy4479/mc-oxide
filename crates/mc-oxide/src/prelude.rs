pub use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
pub use log::{debug, error, info, trace, warn};
pub use serde::{Deserialize, Serialize};
pub use std::path::PathBuf;

lazy_static! {
    pub static ref CACHE_DIR: PathBuf = dirs::data_dir().expect("no cache dir").join("mc-oxide");
    pub static ref CONFIG_DIR: PathBuf =
        dirs::config_dir().expect("no config dir").join("mc-oxide");
}
