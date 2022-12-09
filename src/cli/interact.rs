use std::fmt;

#[derive(Debug)]
pub enum UserOptions {
    SetUsername,
    SetPassword,
    SetAdminPassword,
    UserElevation,
}

impl fmt::Display for UserOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserOptions::SetUsername => write!(f, "Set username"),
            UserOptions::SetPassword => write!(f, "Set user password"),
            UserOptions::SetAdminPassword => write!(f, "Set admin password"),
            UserOptions::UserElevation => write!(f, "User elevation"),
        }
    }
}

#[derive(Debug)]
pub enum VisualOptions {
    SetProfile,
    SetLockscreen,
}

impl fmt::Display for VisualOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VisualOptions::SetProfile => write!(f, "Set profile image"),
            VisualOptions::SetLockscreen => write!(f, "Set lockscreen image"),
        }
    }
}

#[derive(Debug)]
pub enum LoginOptions {
    UserRequirement,
    AutoLogin,
}

impl fmt::Display for LoginOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginOptions::UserRequirement => write!(f, "Username requirement"),
            LoginOptions::AutoLogin => write!(f, "Auto-login"),
        }
    }
}
