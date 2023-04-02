mod cli;

use anyhow::{bail, Context, Result};
use clap::Parser;
use libamect_common::{
    utils::pick_image_file,
    windows::{
        disable_autologon, enable_autologon, get_username, is_admin, net_set_user_elevated,
        set_lockscreen_blur, set_lockscreen_img, set_password, set_profile_img, set_username,
        set_username_login_requirement, wmic_get_session_user, wmic_get_user_sid,
    },
};

use crate::cli::AmectCli;

fn main() -> Result<()> {
    let cli = AmectCli::parse();

    if !is_admin() {
        bail!("admin privileges are required");
    }

    const SESSION_USER_ERR: &str =
        "failed to retrieve username, it's likely that username has been recently modified. \
        Please try again after a relogin.";

    match cli.command {
        None => unreachable!(),
        Some(cli::Commands::User(opts)) => {
            let (_, session_username) = wmic_get_session_user().context(SESSION_USER_ERR)?;

            // Interactive
            if opts == Default::default() {
                // User password
                if inquire::Confirm::new("Set new User password?")
                    .with_default(false)
                    .prompt()?
                {
                    let new_password = inquire::Password::new("New password:")
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()?;
                    set_password(&session_username, &new_password)?;
                }
                // Admin password
                if inquire::Confirm::new("Set new Admin password?")
                    .with_default(false)
                    .prompt()?
                {
                    let new_password = inquire::Password::new("New password:")
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()?;
                    set_password(
                        // *get_username() should return admin user since we're supposed to run the app with admin privs
                        &get_username().context("failed to retrieve username")?,
                        &new_password,
                    )?;
                }
                // User elevation
                match inquire::Confirm::new("Should user be elevated?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(elevate_user) => {
                        net_set_user_elevated(elevate_user, &session_username)?;
                    }
                    None => {}
                }
                // Username
                if inquire::Confirm::new("Set new username?")
                    .with_default(false)
                    .prompt()?
                {
                    let new_username = inquire::Text::new("New username:").prompt()?;
                    set_username(&session_username, &new_username)?;
                }
            }
            // Cli
            else {
                if let Some(new_password) = opts.user_password {
                    set_password(&session_username, &new_password)?;
                }
                if let Some(new_password) = opts.admin_password {
                    set_password(
                        &get_username().context("failed to retrieve username")?,
                        &new_password,
                    )?;
                }
                if let Some(elevate_user) = opts.elevate_user {
                    net_set_user_elevated(elevate_user, &session_username)?;
                }
                if let Some(new_username) = opts.username {
                    // ! Do `username` at the end; Order is important
                    // ! So that the other fns won't fail early due to a username change they didn't expect
                    set_username(&session_username, &new_username)?;
                }
            }
        }
        Some(cli::Commands::Visual(opts)) => {
            let (_, session_username) = wmic_get_session_user().context(SESSION_USER_ERR)?;
            let user_sid = wmic_get_user_sid(&session_username).context(SESSION_USER_ERR)?;

            // Interactive
            if opts == Default::default() {
                if inquire::Confirm::new("Set a new profile image?")
                    .with_default(false)
                    .prompt()?
                {
                    set_profile_img(&user_sid, pick_image_file()?)?;
                }
                if inquire::Confirm::new("Set a new lockscreen image?")
                    .with_default(false)
                    .prompt()?
                {
                    set_lockscreen_img(&user_sid, pick_image_file()?)?;
                }
                match inquire::Confirm::new("Set Lockscreen blur?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(set_blur) => {
                        set_lockscreen_blur(set_blur)?;
                    }
                    None => {}
                }
            }
            // Cli
            else {
                if let Some(new_pfp) = opts.profile_img {
                    set_profile_img(&user_sid, new_pfp)?;
                }
                if let Some(new_lock_img) = opts.lockscreen_img {
                    set_lockscreen_img(&user_sid, new_lock_img)?;
                }
                if let Some(set_blur) = opts.lockscreen_blur {
                    set_lockscreen_blur(set_blur)?;
                }
            }
        }
        Some(cli::Commands::Login(opts)) => {
            let (_, session_username) = wmic_get_session_user().context(SESSION_USER_ERR)?;

            // Interactive
            if opts == Default::default() {
                match inquire::Confirm::new("Require username on login?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(require_username) => {
                        set_username_login_requirement(require_username)?;
                    }
                    None => {}
                }
                match inquire::Confirm::new("Ask for password when logging in?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(true) => disable_autologon()?,
                    Some(false) => {
                        let user_password = inquire::Password::new("Enter your user password:")
                            .with_display_mode(inquire::PasswordDisplayMode::Masked)
                            .with_help_message("Autologin would be setup using this password!")
                            .prompt()?;
                        enable_autologon(&session_username, &user_password)?;
                    }
                    None => {}
                }
            }
            // Cli
            else {
                if let Some(require_username) = opts.require_username {
                    set_username_login_requirement(require_username)?;
                }
                match opts.auto_login {
                    Some(true) => enable_autologon(
                        &session_username,
                        &opts.user_password.expect(
                            "--user-password must always be passed together with --auto-login true",
                        ),
                    )?,
                    Some(false) => disable_autologon()?,
                    None => {}
                }
            }
        }
    }
    Ok(())
}
