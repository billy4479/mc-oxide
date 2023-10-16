use mc_oxide::prelude::*;

pub struct Config(mc_oxide::Config);

pub fn try_load_config() -> Result<Box<Config>> {
    mc_oxide::Config::load().map(|c| Box::new(Config(c)))
}

pub fn config_get_server_dir(config: &Config) -> &str {
    config.0.server_dir.to_str().expect("path not string")
}
