use crate::defines::{app_cache_dir, AUTO_LOGON};
use anyhow::Result;
use std::{fmt, fs, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum CacheEmbedded {
    AutoLogon,
}

impl fmt::Display for CacheEmbedded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheEmbedded::AutoLogon => write!(f, "autologin.exe"),
        }
    }
}

impl CacheEmbedded {
    /// Returns the path to the embedded resource.
    ///
    /// Cache is used if it exists else the embedded data is written to a file
    /// and the path is returned.
    pub fn load(self) -> Result<PathBuf> {
        let cache_dir = app_cache_dir().unwrap_or_default();
        fs::create_dir_all(&cache_dir)?;

        let file_name = self.to_string();
        for entry in fs::read_dir(&cache_dir)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy() == file_name {
                // return cache if exists
                return Ok(entry.path());
            }
        }

        let path = cache_dir.join(file_name);
        let mut file = fs::File::create(&path)?;
        file.write_all(self.get_item())?;

        Ok(path)
    }
    fn get_item(self) -> &'static [u8] {
        match self {
            CacheEmbedded::AutoLogon => AUTO_LOGON,
        }
    }
}
