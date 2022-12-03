use native_dialog::FileDialog;
use std::path::PathBuf;

pub fn browse_image_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .ok()?
}
