#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use app::NightgraphApp;

mod app;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "nightgraph ui",
        native_options,
        Box::new(|cc| Box::new(NightgraphApp::new(cc))),
    );
}
