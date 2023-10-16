use std::{
    fs::{self, File},
    path::Path,
    sync::Arc,
};

use chrono::prelude::*;
use lazy_static::lazy_static;
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};

lazy_static! {
    static ref LOGS_DIR: PathBuf = CACHE_DIR.join("logs");
    static ref NEW_LOG_PREFIX: String = "__latest_".into();
}

use crate::prelude::*;

fn compress_old_log(new_log: &Path) -> Result<()> {
    fs::read_dir(LOGS_DIR.as_path())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(NEW_LOG_PREFIX.as_str())
                && entry.file_name() != new_log.file_name().expect("log filename")
        })
        .map(|entry| -> Result<()> {
            let compressed_name = entry
                .file_name()
                .to_string_lossy()
                .replace(NEW_LOG_PREFIX.as_str(), "")
                + ".zstd";

            let compressed_log = File::create(LOGS_DIR.join(compressed_name))?;
            let input_log = File::open(entry.path())?;
            zstd::stream::copy_encode(input_log, compressed_log, 0)?;

            fs::remove_file(entry.path())?;

            Ok(())
        })
        .filter(|result| result.is_err())
        .for_each(|err| warn!("unable to compress log: {}", err.unwrap_err()));

    Ok(())
}

pub async fn init() -> Result<()> {
    fs::create_dir_all(LOGS_DIR.as_path())?;

    let now = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let new_log_path = Arc::new(LOGS_DIR.join(format!("{}{}.log", NEW_LOG_PREFIX.as_str(), now)));

    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}][{f}][{l}]: {m}\n",
        )))
        .build();

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}][{l}]: {m}\n",
        )))
        .append(false)
        .build(&*new_log_path)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Warn)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )?;

    let _ = log4rs::init_config(config)?;

    debug!("log4rs initialized");

    #[cfg(debug_assertions)]
    info!("this is a development build");

    tokio::spawn(async move {
        match compress_old_log(&new_log_path) {
            Ok(()) => {}
            Err(e) => {
                warn!("unable to compress old logs: {}", e)
            }
        }
    });

    Ok(())
}
