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
pub enum UserElevationOptions {
    Enable,
    Disable,
}

impl fmt::Display for UserElevationOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserElevationOptions::Enable => write!(f, "Elevate user to admin"),
            UserElevationOptions::Disable => write!(f, "Revoke user elevations"),
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

#[derive(Debug)]
pub enum UserRequirementOptions {
    Enable,
    Disable,
}

impl fmt::Display for UserRequirementOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRequirementOptions::Enable => write!(f, "Enable username requiement on login"),
            UserRequirementOptions::Disable => write!(f, "Disable username requiement on login"),
        }
    }
}

#[derive(Debug)]
pub enum AutoLoginOptions {
    Enable,
    Disable,
}

impl fmt::Display for AutoLoginOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutoLoginOptions::Enable => write!(f, "Enable AutoLogon"),
            AutoLoginOptions::Disable => write!(f, "Disable AutoLogon"),
        }
    }
}
