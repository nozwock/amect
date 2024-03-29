use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn pick_image_file() -> Result<PathBuf> {
    native_dialog::FileDialog::new()
        .add_filter("All Images", &["png", "jpg", "jpeg", "bmp"])
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .add_filter("Bitmap Image", &["bmp"])
        .show_open_single_file()
        .map(|f| f.context("no file was selected"))?
}
