use crate::prelude::*;

use std::{
    env,
    fs::{self, File},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_dir: PathBuf,

    pub server_gui: bool,
    pub auto_eula: bool,
    pub memory_max_mb: u32,

    pub git_integration: bool,
    pub lockfile: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server_dir: ".".into(),
            server_gui: true,
            auto_eula: true,
            memory_max_mb: 4 * 1024,
            git_integration: true,
            lockfile: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        let config_path = {
            match env::var("CONFIG_PATH") {
                Ok(path) => {
                    debug!("loading config from {} instead of the default path", path);
                    PathBuf::from(&path)
                }
                Err(_) => {
                    let config_file = CONFIG_DIR.join("config.yml");
                    fs::create_dir_all(CONFIG_DIR.as_path())?;
                    config_file
                }
            }
        };

        match File::open(&config_path) {
            Ok(file) => {
                let config: Config = serde_yaml::from_reader(file)?;
                Ok(config)
            }
            Err(err) => {
                warn!(
                    "an error has occurred while loading the config, falling back to default: {err}"
                );

                let config = Config::default();
                fs::write(&config_path, serde_yaml::to_string(&config)?)?;

                trace!("config written");

                Ok(config)
            }
        }
    }
}
