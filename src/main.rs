#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use roxy_launcher::{
    RoxyLauncher,
    utils::{logger_util, path_util},
};

const TITLE: &str = "Roxy Launcher";
static DEFUALT_LOG_PATH: &str = "~/.local/state/roxy/log.log";

#[tokio::main]
async fn main() -> eframe::Result {
    logger_util::init_logging(
        path_util::expand_home_dir_string(DEFUALT_LOG_PATH.to_string()).as_str(),
        true,
    )
    .await;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        TITLE,
        native_options,
        Box::new(|cc| Ok(Box::new(RoxyLauncher::new(cc)))),
    )
}
