use native_dialog::FileDialog;
use std::path::PathBuf;

pub fn browse_image_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("All Images", &["png", "jpg", "jpeg", "bmp"])
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .add_filter("Bitmap Image", &["bmp"])
        .show_open_single_file()
        .ok()?
}
