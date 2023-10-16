use mc_oxide::prelude::*;

use crate::RUNTIME;

pub fn try_init_logger() -> Result<()> {
    RUNTIME.block_on(mc_oxide::logger::init())
}

pub fn log_trace(msg: &str) {
    trace!("{}", msg);
}

pub fn log_debug(msg: &str) {
    debug!("{}", msg);
}

pub fn log_info(msg: &str) {
    info!("{}", msg);
}

pub fn log_warn(msg: &str) {
    warn!("{}", msg);
}

pub fn log_error(msg: &str) {
    error!("{}", msg);
}
