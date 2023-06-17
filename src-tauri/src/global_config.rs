use std::{
    env,
    fs::{self, File},
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub server_dir: PathBuf,

    pub server_gui: bool,
    pub auto_eula: bool,
    pub memory_max_mb: u32,

    pub git_integration: bool,
    pub lockfile: bool,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            server_dir: ".".into(),
            server_gui: true,
            auto_eula: true,
            memory_max_mb: 4 * 1024,
            git_integration: true,
            lockfile: true,
        }
    }
}

impl GlobalConfig {
    pub fn load() -> Result<GlobalConfig> {
        let config_path = {
            match env::var("CONFIG_PATH") {
                Ok(path) => {
                    debug!("loading config from {} instead of the default path", path);
                    PathBuf::from(&path)
                }
                Err(_) => {
                    let mut config_dir = dirs::config_dir().ok_or(anyhow!("no config dir"))?;

                    config_dir.push("mc-oxide");
                    let config_file = config_dir.join("config.yml");
                    fs::create_dir_all(config_dir)?;
                    config_file
                }
            }
        };

        match File::open(&config_path) {
            Ok(file) => {
                let config: GlobalConfig = serde_yaml::from_reader(file)?;
                Ok(config)
            }
            Err(err) => {
                warn!(
                    "an error has occurred while loading the config, falling back to default: {err}"
                );

                let config = GlobalConfig::default();
                fs::write(&config_path, serde_yaml::to_string(&config)?)?;

                Ok(config)
            }
        }
    }
}
