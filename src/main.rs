#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Hides console on Windows in release build

mod gui;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    use crate::gui::AmectApp;
    use anyhow::bail;
    use eframe::epaint::Vec2;
    use libamect_common::windows::is_admin;

    if !is_admin() {
        native_dialog::MessageDialog::new()
            .set_type(native_dialog::MessageType::Error)
            .set_title("Error")
            .set_text("admin privileges are required!")
            .show_alert()?;
        bail!("admin privileges are required!");
    }

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(550., 400.)),
        min_window_size: Some(Vec2::new(400., 200.)),
        ..Default::default()
    };

    eframe::run_native(
        "Central AME toolkit",
        native_options,
        Box::new(|cc| Box::new(AmectApp::new(cc))),
    );

    Ok(())
}
