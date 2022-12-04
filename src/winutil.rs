// #[cfg(windows)]
use {
    anyhow::{bail, Result},
    std::{ffi::CString, process::Command, slice, str},
    widestring::U16CString,
    windows::{
        core::{PCWSTR, PWSTR},
        Win32::{
            NetworkManagement::NetManagement::{NERR_Success, NERR_UserNotFound, NetUserSetInfo},
            System::WindowsProgramming::GetUserNameW,
        },
    },
};

// * #[cfg(windows)] attrs are commented temporarily bcz I'm developing on unix

// #[cfg(windows)]
/// Retrieves the name of the user associated with the current thread.
///
/// Uses `GetUserNameW` API to get the username.
pub fn get_username() -> Option<String> {
    // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew
    // https://stackoverflow.com/questions/68716774/
    let mut cb_buffer = 257_u32;

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
// ! doesn't work idk why; figure this out later
/// Uses `NetUserSetInfo` API to get the username.
pub fn set_username(curr: &str, new: &str) -> Result<()> {
    // https://learn.microsoft.com/en-us/windows/win32/api/lmaccess/nf-lmaccess-netusersetinfo
    let curr = U16CString::from_str(curr)?;
    let new = CString::new(new)?;

    let username = PCWSTR::from_raw(curr.as_ptr());
    let buf = new.as_ptr() as *const u8;

    let result = unsafe { NetUserSetInfo(PCWSTR::null(), username, 1011_u32, buf, None) };

    if result == NERR_Success {
        return Ok(());
    } else if result == NERR_UserNotFound {
        bail!("username not found");
    }

    bail!("failed to set username; {}", result);
}
