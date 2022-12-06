// #[cfg(windows)]
use {
    anyhow::{bail, Result},
    std::{process::Command, slice, str},
    widestring::U16CString,
    windows::{
        core::{PCWSTR, PWSTR},
        Win32::{
            NetworkManagement::NetManagement::{
                NERR_Success, NERR_UserNotFound, NetUserSetInfo, USER_INFO_0, USER_INFO_1003,
            },
            System::WindowsProgramming::GetUserNameW,
        },
    },
    winreg::{enums::HKEY_LOCAL_MACHINE, RegKey},
};

// * #[cfg(windows)] attrs are commented temporarily bcz I'm developing on unix; yes it's a pain

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

    let mut buf = USER_INFO_0::default();
    buf.usri0_name = PWSTR::from_raw(new.as_mut_ptr());

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

    let mut buf = USER_INFO_1003::default();
    buf.usri1003_password = PWSTR::from_raw(password.as_mut_ptr());

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
            .args(&["computersystem", "get", "username"])
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
                .and_then(|(desktop, username)| Some((desktop.to_owned(), username.to_owned())))
        })
    })
}

// #[cfg(windows)]
/// **NOTE:** returns `None` if username was changed in the active session.
pub fn wmic_get_user_sid(username: &str) -> Option<String> {
    str::from_utf8(
        Command::new("WMIC")
            .args(&[
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
        .args(&[
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
        .args(&["user", username, password])
        .status()?;

    if !result.success() {
        bail!("failed to set password for {}; {}", username, result);
    }

    Ok(())
}

// #[cfg(windows)]
/// Have a look here-
/// https://git.ameliorated.info/Joe/amecs/src/branch/master#user-elevation
pub fn net_user_elevate(username: &str) -> Result<()> {
    let result = Command::new("NET")
        .args(&["localgroup", "administrators", username, "/add"])
        .status()?;

    if !result.success() {
        bail!("failed to elevate for {}; {}", username, result);
    }

    Ok(())
}

// #[cfg(windows)]
/// Have a look here-
/// https://git.ameliorated.info/Joe/amecs/src/branch/master#user-elevation
pub fn net_user_unelevate(username: &str) -> Result<()> {
    let result = Command::new("NET")
        .args(&["localgroup", "administrators", username, "/delete"])
        .status()?;

    if !result.success() {
        bail!("failed to elevate for {}; {}", username, result);
    }

    Ok(())
}

// #[cfg(windows)]
pub fn disable_username_login_req() -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _disp) =
        hklm.create_subkey(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"#)?;

    key.delete_value("dontdisplaylastusername")
        .map_err(Into::into)
}

// #[cfg(windows)]
pub fn enable_username_login_req() -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _disp) =
        hklm.create_subkey(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"#)?;

    key.set_value("dontdisplaylastusername", &1_u32)
        .map_err(Into::into)
}
