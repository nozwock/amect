use std::fs;

// #[cfg(windows)]
use {
    anyhow::{bail, Result},
    std::{path::Path, process::Command, slice, str},
    widestring::U16CString,
    windows::{
        core::{PCWSTR, PWSTR},
        Win32::{
            NetworkManagement::NetManagement::{
                NERR_Success, NERR_UserNotFound, NetUserSetInfo, USER_INFO_0, USER_INFO_1003,
            },
            System::WindowsProgramming::GetUserNameW,
            UI::Shell::IsUserAnAdmin,
        },
    },
    winreg::{
        enums::{HKEY_LOCAL_MACHINE, HKEY_USERS},
        RegKey,
    },
};

// * #[cfg(windows)] attrs are commented temporarily bcz I'm developing on unix; yes it's a pain

pub fn is_admin() -> bool {
    unsafe { IsUserAnAdmin().as_bool() }
}

// #[cfg(windows)]
/// Retrieves the name of the user associated with the current thread.
///
/// Uses `GetUserNameW` API to get the username.
pub fn get_username() -> Option<String> {
    // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew
    // https://stackoverflow.com/questions/68716774/
    let mut cb_buffer: u32 = 257;

    // Create a buffer of the required size
    let mut buffer = Vec::<u16>::with_capacity(cb_buffer as usize);
    // Construct a `PWSTR` by taking the address to the first element in the buffer
    let lp_buffer = PWSTR(buffer.as_mut_ptr());

    let result = unsafe { GetUserNameW(lp_buffer, &mut cb_buffer) };

    let mut user_name = None;
    // If the API returned success, and more than 0 characters were written
    if result.as_bool() && cb_buffer > 0 {
        // Construct a slice over the valid data
        let buffer = unsafe { slice::from_raw_parts(lp_buffer.0, cb_buffer as usize - 1) };

        // And convert from UTF-16 to Rust's native encoding
        user_name = Some(String::from_utf16_lossy(buffer));
    }

    user_name
}

// #[cfg(windows)]
/// Uses `NetUserSetInfo` API to set the username.
pub fn set_username(curr: &str, new: &str) -> Result<()> {
    // API docs
    // https://learn.microsoft.com/en-us/windows/win32/api/lmaccess/nf-lmaccess-netusersetinfo

    // An example on how to use the NewUserSetInfo thanks to Rafael
    // https://gist.github.com/riverar/f49115e6b2736ff9f9adc3ead5919d5d

    let curr = U16CString::from_str(curr)?;
    let mut new = U16CString::from_str(new)?;

    let buf = USER_INFO_0 {
        usri0_name: PWSTR::from_raw(new.as_mut_ptr()),
    };

    let result = unsafe {
        NetUserSetInfo(
            None,
            PCWSTR::from_raw(curr.as_ptr()),
            0,
            &buf as *const _ as _,
            None,
        )
    };

    if result == NERR_Success {
        return Ok(());
    } else if result == NERR_UserNotFound {
        bail!("username not found");
    }

    bail!("failed to set username; {}", result);
}

// #[cfg(windows)]
/// Uses `NetUserSetInfo` API to set the password.
pub fn set_password(username: &str, password: &str) -> Result<()> {
    let username = U16CString::from_str(username)?;
    let mut password = U16CString::from_str(password)?;

    let buf = USER_INFO_1003 {
        usri1003_password: PWSTR::from_raw(password.as_mut_ptr()),
    };

    let result = unsafe {
        NetUserSetInfo(
            None,
            PCWSTR::from_raw(username.as_ptr()),
            1003,
            &buf as *const _ as _,
            None,
        )
    };

    if result == NERR_Success {
        return Ok(());
    } else if result == NERR_UserNotFound {
        bail!("username not found");
    }

    bail!("failed to set password; {}", result);
}

// #[cfg(windows)]
/// Returns a tuple containing `domainname` & `username`.
///
/// **NOTE:** returns `None` if username was changed in the active session.
pub fn wmic_get_session_user() -> Option<(String, String)> {
    str::from_utf8(
        Command::new("WMIC")
            .args(["computersystem", "get", "username"])
            .output()
            .ok()?
            .stdout
            .as_slice(),
    )
    .ok()
    .and_then(|raw| {
        raw.trim().lines().last().and_then(|desktop_user| {
            desktop_user
                .split_once('\\')
                .map(|(desktop, username)| (desktop.to_owned(), username.to_owned()))
        })
    })
}

// #[cfg(windows)]
/// **NOTE:** returns `None` if username was changed in the active session.
pub fn wmic_get_user_sid(username: &str) -> Option<String> {
    str::from_utf8(
        Command::new("WMIC")
            .args([
                "useraccount",
                "where",
                format!("name='{}'", username).as_str(),
                "get",
                "sid",
            ])
            .output()
            .ok()?
            .stdout
            .as_slice(),
    )
    .ok()
    .and_then(|raw| raw.trim().lines().last().map(|s| s.to_owned()))
}

// #[cfg(windows)]
/// Set new username for a user with `curr` as their username.
pub fn wmic_set_username(curr: &str, new: &str) -> Result<()> {
    let result = Command::new("WMIC")
        .args([
            "useraccount",
            "where",
            format!("name='{}'", curr).as_str(),
            "rename",
            new,
        ])
        .status()?;

    if !result.success() {
        bail!("failed to set username; {}", result);
    }

    Ok(())
}

// #[cfg(windows)]
pub fn net_set_password(username: &str, password: &str) -> Result<()> {
    let result = Command::new("NET")
        .args(["user", username, password])
        .status()?;

    if !result.success() {
        bail!("failed to set password for {}; {}", username, result);
    }

    Ok(())
}

// #[cfg(windows)]
/// Set user elevated privileges
///
/// Have a look here-
/// https://git.ameliorated.info/Joe/amecs/src/branch/master#user-elevation
pub fn net_set_user_elevated(enable: bool, username: &str) -> Result<()> {
    let action = if enable { "/add" } else { "/delete" };
    let result = Command::new("NET")
        .args(["localgroup", "administrators", username, action])
        .status()?;

    if !result.success() {
        bail!("failed to set permissions for {}; {}", username, result);
    }

    Ok(())
}

// #[cfg(windows)]
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

pub fn set_lockscreen_img(user_sid: &str, image: impl AsRef<Path>) -> Result<()> {
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
        if !Command::new("takeown")
            .args(["/f", img.as_str()])
            .status()?
            .success()
            && !Command::new("icacls")
                .args([img.as_str(), "/reset"])
                .status()?
                .success()
        {
            bail!("failed to take ownership of old images");
        }
        // TODO: maybe do img conversion rather than just renaming the extension
        fs::copy(image, Path::new(img.as_str()))?;
    }

    // clear cache
    let systemdata = format!("{}\\{}", programdata, r#"Microsoft\Windows\SystemData"#);
    if !Command::new("takeown")
        .args(["/r", "/d", "y", "/f", systemdata.as_str()])
        .status()?
        .success()
        && !Command::new("icacls")
            .args([systemdata.as_str(), "/reset"])
            .status()?
            .success()
    {
        bail!("failed to take ownership of directory");
    }

    // ln 459
    // almost done...
    todo!()
}

fn set_profile_img(user_sid: &str, image: impl AsRef<Path>) -> Result<()> {
    todo!()
}

/// This is not ideal
///
/// **NOTE:** Uses `powershell`
pub fn get_env_var(env_var: &str) -> Result<String> {
    str::from_utf8(
        Command::new("powershell")
            .args([
                "-NoP",
                "-C",
                format!(
                    "[System.Environment]::GetEnvironmentVariable('{}')",
                    env_var
                )
                .as_str(),
            ])
            .output()?
            .stdout
            .as_slice(),
    )
    .map(|s| s.trim().to_owned())
    .map_err(Into::into)
}
