#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod apps;
mod main_app;
mod meshes;

use eframe::egui;
use main_app::MainApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "wgpu samples",
        options,
        Box::new(|cc: &eframe::CreationContext| Box::new(MainApp::new(cc).unwrap())),
    )
}
