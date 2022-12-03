#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use amect::gui::App;

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "amect",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
