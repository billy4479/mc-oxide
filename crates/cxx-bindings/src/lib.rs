mod config;
mod logger;

use config::*;
use logger::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {

        // logger start
        pub fn try_init_logger() -> Result<()>;

        pub fn log_trace(msg: &str);
        pub fn log_debug(msg: &str);
        pub fn log_info(msg: &str);
        pub fn log_warn(msg: &str);
        pub fn log_error(msg: &str);
        // logger end

        // config start
        type Config;

        pub fn try_load_config() -> Result<Box<Config>>;
        pub fn config_get_server_dir(config: &Config) -> &str;
        // config end
    }
}
