use std::path::PathBuf;

pub const APP_DIR: &str = "com.github.nozwock.amect";

// https://github.com/rzander/AutoLogon
pub const AUTO_LOGON: &[u8] = include_bytes!("../resources/autologon.exe");

pub fn app_cache_dir() -> Option<PathBuf> {
    Some(dirs::cache_dir()?.join(APP_DIR))
}
