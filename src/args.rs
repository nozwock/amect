use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct AmectCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Use the app in cli mode
    #[command(short_flag = 'c', subcommand)]
    Cli(Cli),
}

#[derive(Debug, Subcommand)]
pub enum Cli {
    Users(Users),
    Visuals(Visuals),
    Login(Login),
}

#[derive(Debug, Args, Default, PartialEq)]
pub struct Users {
    /// Set new username
    #[arg(long, value_name = "STRING")]
    pub username: Option<String>,
    /// Set new user password
    #[arg(long, value_name = "STRING")]
    pub user_password: Option<String>,
    /// Set new admin password
    #[arg(long, value_name = "STRING")]
    pub admin_password: Option<String>,
    /// Set new profile image
    #[arg(long, value_name = "FILE")]
    pub profile_img: Option<PathBuf>,
    #[arg(long, value_name = "BOOL")]
    pub elevate_user: Option<bool>,
}

#[derive(Debug, Args, Default, PartialEq)]
pub struct Visuals {
    /// Set new lockscreen image
    #[arg(long, value_name = "FILE")]
    pub lockscreen_img: Option<PathBuf>,
}

#[derive(Debug, Args, Default, PartialEq)]
pub struct Login {
    #[arg(long, value_name = "BOOL")]
    pub require_username: Option<bool>,
    #[arg(long, value_name = "BOOL")]
    pub auto_login: Option<bool>,
}
