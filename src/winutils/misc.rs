use super::helpers::get_env_var;
use crate::cache::CacheEmbedded;
use anyhow::{bail, Result};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, HKEY_USERS},
    RegKey,
};

/// Set user elevated privileges
///
/// Have a look here-
/// https://git.ameliorated.info/Joe/amecs/src/branch/master#user-elevation
pub fn net_set_user_elevated(enable: bool, username: &str) -> Result<()> {
    let action = if enable { "/add" } else { "/delete" };
    let result = Command::new("NET")
        .args(["localgroup", "administrators", username, action])
        .stdout(Stdio::null())
        .status()?;

    if !result.success() {
        bail!("failed to set permissions for {}; {}", username, result);
    }

    Ok(())
}

pub fn set_username_login_requirement(enable: bool) -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _disp) =
        hklm.create_subkey(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"#)?;

    if enable {
        key.set_value("dontdisplaylastusername", &1_u32)
            .map_err(Into::into)
    } else {
        key.delete_value("dontdisplaylastusername")
            .map_err(Into::into)
    }
}

pub fn set_lockscreen_blur(enable: bool) -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _disp) = hklm.create_subkey(r#"SOFTWARE\Policies\Microsoft\Windows\System"#)?;

    if enable {
        key.set_value("DisableAcrylicBackgroundOnLogon", &1_u32)
            .map_err(Into::into)
    } else {
        key.delete_value("DisableAcrylicBackgroundOnLogon")
            .map_err(Into::into)
    }
}

pub fn set_lockscreen_img<P: AsRef<Path>>(user_sid: &str, image_path: P) -> Result<()> {
    // Supposedly necessary for updated 21H2+ versions if RotatingLockScreenEnabled is not already set to 0
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _disp) = hklm.create_subkey(format!(
        "{}\\{}",
        r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Authentication\LogonUI\Creative"#, user_sid
    ))?;
    key.set_value("RotatingLockScreenEnabled", &0_u32)?;

    let hku = RegKey::predef(HKEY_USERS);
    let (key, _disp) = hku.create_subkey(format!(
        "{}\\{}",
        user_sid, r#"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"#
    ))?;
    key.set_value("RotatingLockScreenEnabled", &0_u32)?;

    let windir = get_env_var("windir")?;
    let programdata = get_env_var("programdata")?;

    // Copy img to the right spot
    for img in ["img100.jpg", "img103.png", "img0.jpg"]
        .into_iter()
        .map(|img| format!("{}\\{}\\{}", windir, r#"Web\Screen"#, img))
    {
        // TODO: fix err for when file not found
        // TODO: also set strerr to null
        if !dbg!(Command::new("takeown")
            .args(["/f", img.as_str()])
            .stdout(Stdio::null())
            .status()?)
        .success()
            || !Command::new("icacls")
                .args([img.as_str(), "/reset"])
                .stdout(Stdio::null())
                .status()?
                .success()
        {
            bail!("failed to take ownership of old images");
        }
        // TODO: maybe consider doing proper img conversion rather than just renaming the extension?
        fs::copy(&image_path, Path::new(img.as_str()))?;
    }

    // clear cache
    let systemdata = format!("{}\\{}", programdata, r#"Microsoft\Windows\SystemData"#);
    if !Command::new("takeown")
        .args(["/r", "/d", "y", "/f", systemdata.as_str()])
        .stdout(Stdio::null())
        .status()?
        .success()
        || !Command::new("icacls")
            .args([systemdata.as_str(), "/reset"])
            .stdout(Stdio::null())
            .status()?
            .success()
    {
        bail!("failed to take ownership of directory");
    }

    for entry in fs::read_dir(systemdata.as_str())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            for inner in fs::read_dir(path.join("ReadOnly"))? {
                let inner = inner?;
                let path = inner.path();
                let name = path.to_string_lossy();
                if path.is_dir() && name.starts_with("LockScreen_") {
                    fs::remove_dir_all(path)?;
                }
            }
        }
    }

    Ok(())
}

/// needs testing
pub fn set_profile_img<P: AsRef<Path>>(user_sid: &str, image_path: P) -> Result<()> {
    let pfp = image::open(&image_path)?;

    let public = get_env_var("public")?;
    let usr_pfp_dir = format!("{}\\AccountPictures\\{}", public, user_sid);

    fs::create_dir_all(usr_pfp_dir.as_str())?;
    if !Command::new("takeown")
        .args(["/r", "/d", "y", "/f", usr_pfp_dir.as_str()])
        .stdout(Stdio::null())
        .status()?
        .success()
        || !Command::new("icacls")
            .args([usr_pfp_dir.as_str(), "/reset", "/t"])
            .stdout(Stdio::null())
            .status()?
            .success()
    {
        bail!("failed to take ownership of directory");
    }
    for entry in fs::read_dir(usr_pfp_dir.as_str())? {
        fs::remove_file(entry?.path())?;
    }

    let sizes = [32, 40, 48, 64, 96, 192, 208, 240, 424, 448, 1080];
    for size in sizes.iter() {
        let scaled = pfp.resize_exact(*size, *size, image::imageops::FilterType::Lanczos3);
        scaled.save(format!("{}\\{}x{}.png", usr_pfp_dir.as_str(), *size, *size))?;
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let usr_pfp_key = format!(
        "{}\\{}",
        r#"SOFTWARE\Microsoft\Windows\CurrentVersion\AccountPicture\Users"#, user_sid
    );
    // TODO: figure out how to run the reg delete as system user (for compaitibility) ln 502
    if let Err(err) = hklm.delete_subkey_all(usr_pfp_key.as_str()) {
        match err.kind() {
            std::io::ErrorKind::NotFound => { /* ignore */ }
            _ => Err(err)?,
        }
    }

    let (key, _disp) = hklm.create_subkey(usr_pfp_key)?;
    for size in sizes {
        key.set_value(
            format!("Image{}", size),
            &format!("{}\\{}x{}.png", usr_pfp_dir.as_str(), size, size),
        )?;
    }

    RegKey::predef(HKEY_USERS)
        .create_subkey(format!(
            "{}\\{}",
            user_sid, r#"SOFTWARE\OpenShell\StartMenu\Settings"#
        ))?
        .0
        .set_value(
            "UserPicturePath",
            &format!("{}\\{}x{}.png", usr_pfp_dir, 448, 448),
        )?;

    Ok(())
}

pub fn enable_autologon(username: &str, password: &str) -> Result<()> {
    let autologon_path = CacheEmbedded::AutoLogon.load()?;

    if !Command::new(&autologon_path)
        .arg("/del")
        .stdout(Stdio::null())
        .status()?
        .success()
        || !Command::new(&autologon_path)
            .args([username, password, "/DISABLECAD"])
            .stdout(Stdio::null())
            .status()?
            .success()
    {
        bail!("failed to enable autologon");
    }

    Ok(())
}

pub fn disable_autologon() -> Result<()> {
    let autologon_path = CacheEmbedded::AutoLogon.load()?;

    if !Command::new(&autologon_path)
        .arg("/del")
        .stdout(Stdio::null())
        .status()?
        .success()
    {
        bail!("failed to disable autologon");
    }

    Ok(())
}
