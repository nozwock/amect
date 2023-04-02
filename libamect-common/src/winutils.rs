use std::process::Command;

use anyhow::Result;
use windows::Win32::UI::Shell::IsUserAnAdmin;

pub fn is_admin() -> bool {
    unsafe { IsUserAnAdmin().as_bool() }
}

/// This is not ideal due to the finnicky string manipulation.
///
/// Uses `powershell`.
pub fn get_env_var(env_var: &str) -> Result<String> {
    String::from_utf8(
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
            .stdout,
    )
    .map(|s| s.trim().to_owned())
    .map_err(Into::into)
}
