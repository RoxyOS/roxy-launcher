#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use roxy_launcher::RoxyLauncher;

const TITLE: &str = "Roxy Launcher";

fn main() -> eframe::Result {
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
