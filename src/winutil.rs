// #[cfg(windows)]
use {
    anyhow::{bail, Result},
    std::{process::Command, slice, str},
    windows::{
        core::{PCWSTR, PWSTR},
        Win32::{
            NetworkManagement::NetManagement::NetUserSetInfo,
            System::WindowsProgramming::GetUserNameW,
        },
    },
};

// * #[cfg(windows)] attrs are commented temporarily bcz I'm developing on unix

// #[cfg(windows)]
/// Retrieves the name of the user associated with the current thread.
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
/// Returns `None` if username was changed in the active session.
pub fn get_session_username() -> Option<String> {
    str::from_utf8(
        Command::new("cmd")
            .args(&["/C", "wmic computersystem get username"])
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
                .and_then(|(_desktop, username)| Some(username.to_owned()))
        })
    })
}

pub fn get_domainname() -> Option<String> {
    todo!()
}

// #[cfg(windows)]
// ! doesn't work yet
pub fn set_username(curr: &str, new: &str) -> Result<()> {
    // https://learn.microsoft.com/en-us/windows/win32/api/lmaccess/nf-lmaccess-netusersetinfo

    let servername = PCWSTR::null();
    let curr = curr.encode_utf16().collect::<Vec<_>>();
    let username = PCWSTR(curr.as_ptr());
    let level = 1011_u32;
    let buf = new.as_ptr();

    let result = unsafe { NetUserSetInfo(servername, username, level, buf, None) };

    dbg!(&result);

    if result != 0 {
        bail!("failed to set username");
    }

    Ok(())
}

// #[cfg(windows)]
pub fn set_password(username: &str, password: &str) -> Result<()> {
    // https://learn.microsoft.com/en-us/windows/win32/api/lmaccess/nf-lmaccess-netusersetinfo
    todo!()
}
