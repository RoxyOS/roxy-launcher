use std::path::Path;

use crate::utils::path_util;
use tokio::fs;
use tracing::{Level, event, instrument};
use tracing::{error, info};
use tracing_subscriber::filter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

pub async fn init_logging(path: &str, stdout_enabled: bool) {
    let mut file_name: String = String::new();
    let log_dir = path_util::parent_or_cur_dir(path.to_string(), &mut file_name);
    println!("{}", log_dir);
    if !Path::new(log_dir.as_str()).exists() {
        if let Err(e) = fs::create_dir_all(log_dir.as_str()).await {
            eprintln!("error on init logging!{}", e)
        }
    }
    let file_appender = tracing_appender::rolling::daily(log_dir, file_name);
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer_format = tracing_subscriber::fmt::format();
    let file_layer = fmt::Layer::default()
        .event_format(file_layer_format)
        .with_writer(file_writer)
        .json();
    let stdout_layer = fmt::Layer::default()
        .with_writer(std::io::stdout)
        .with_ansi(false)
        .with_filter(filter::LevelFilter::INFO);
    let subscriber = tracing_subscriber::Registry::default()
        .with(file_layer)
        .with(stdout_layer);
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("failed to set global default tracing");
    } else {
        info!("Logger init Succesfully");
    }
}
