#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use amect::AMEApp;
    use eframe::epaint::Vec2;

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(510., 400.)),
        min_window_size: Some(Vec2::new(400., 200.)),
        ..Default::default()
    };
    eframe::run_native(
        "Central AME toolkit",
        native_options,
        Box::new(|cc| Box::new(AMEApp::new(cc))),
    );
}
