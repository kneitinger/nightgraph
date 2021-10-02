use app::NightgraphApp;

mod app;

fn main() {
    let app = NightgraphApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
