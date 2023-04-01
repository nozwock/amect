use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct AmectCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Users related settings
    User(User),
    /// Visuals related settings
    Visual(Visual),
    /// Login related settings
    Login(Login),
}

#[derive(Debug, Args, Default, PartialEq, Eq)]
pub struct User {
    /// Set new username
    #[arg(long, value_name = "STRING")]
    pub username: Option<String>,
    /// Set new user password
    #[arg(long, value_name = "STRING")]
    pub user_password: Option<String>,
    /// Set new admin password
    #[arg(long, value_name = "STRING")]
    pub admin_password: Option<String>,
    /// Add or remove user from admin group
    #[arg(long, value_name = "BOOL")]
    pub elevate_user: Option<bool>,
}

#[derive(Debug, Args, Default, PartialEq, Eq)]
pub struct Visual {
    /// Set new profile image
    #[arg(long, value_name = "FILE")]
    pub profile_img: Option<PathBuf>,
    /// Set new lockscreen image
    #[arg(long, value_name = "FILE")]
    pub lockscreen_img: Option<PathBuf>,
    /// Enable/disable lockscreen blur
    #[arg(long, value_name = "BOOL")]
    pub lockscreen_blur: Option<bool>,
}

#[derive(Debug, Args, Default, PartialEq, Eq)]
pub struct Login {
    /// Whether to require username on login or not
    #[arg(long, value_name = "BOOL")]
    pub require_username: Option<bool>,
    /// Enable/disable AutoLogon
    #[arg(long, value_name = "BOOL")]
    pub auto_login: Option<bool>,
    #[arg(
        short,
        long,
        value_name = "STRING",
        required_if_eq("auto_login", "true")
    )]
    /// Current user password; Needed to setup autologin
    pub user_password: Option<String>,
}
