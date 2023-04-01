mod cli;

use anyhow::Result;
use clap::Parser;
// use dialoguer::theme::ColorfulTheme;

use crate::cli::AmectCli;

fn main() -> Result<()> {
    let cli = AmectCli::parse();

    match dbg!(cli.command) {
        None => unreachable!(),
        Some(cli::Commands::User(opts)) => {
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
                    todo!();
                }
                // Admin password
                if inquire::Confirm::new("Set new Admin password?")
                    .with_default(false)
                    .prompt()?
                {
                    let new_password = inquire::Password::new("New password:")
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()?;
                    todo!();
                }
                // User elevation
                match inquire::Confirm::new("Should user be elevated?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(elevate_user) => todo!(),
                    None => {}
                }
                // Username
                if inquire::Confirm::new("Set new username?")
                    .with_default(false)
                    .prompt()?
                {
                    let new_username = inquire::Text::new("New username:").prompt()?;
                    todo!();
                }
            }
            // Cli
            else {
                if let Some(new_password) = opts.user_password {
                    todo!();
                }
                if let Some(new_password) = opts.admin_password {
                    todo!();
                }
                if let Some(elevate_user) = opts.elevate_user {
                    todo!();
                }
                if let Some(new_username) = opts.username {
                    // ! Do `username` at the end; Order is important
                    // ! So that the other fns won't fail early due to a username change they didn't expect
                    todo!();
                }
            }
        }
        Some(cli::Commands::Visual(opts)) => {
            // Interactive
            if opts == Default::default() {
                if inquire::Confirm::new("Set a new profile image?")
                    .with_default(false)
                    .prompt()?
                {
                    todo!();
                }
                if inquire::Confirm::new("Set a new profile image?")
                    .with_default(false)
                    .prompt()?
                {
                    todo!();
                }
                match inquire::Confirm::new("Set Lockscreen blur?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(set_blur) => todo!(),
                    None => {}
                }
            }
            // Cli
            else {
                if let Some(new_pfp) = opts.profile_img {
                    todo!();
                }
                if let Some(new_lock_img) = opts.lockscreen_img {
                    todo!();
                }
                if let Some(set_blur) = opts.lockscreen_blur {
                    todo!();
                }
            }
        }
        Some(cli::Commands::Login(opts)) => {
            // Interactive
            if opts == Default::default() {
                match inquire::Confirm::new("Require username on login?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(require_username) => todo!(),
                    None => {}
                }
                match inquire::Confirm::new("Ask for password when logging in?")
                    .with_help_message("Press Esc to skip")
                    .prompt_skippable()?
                {
                    Some(auto_login) => {
                        let user_password = inquire::Password::new("Enter your user password:")
                            .with_display_mode(inquire::PasswordDisplayMode::Masked)
                            .with_help_message("Autologin would be setup using this password!")
                            .prompt()?;
                    }
                    None => {}
                }
            }
            // Cli
            else {
                if let Some(require_username) = opts.require_username {
                    todo!();
                }
                if let Some(auto_login) = opts.auto_login {
                    todo!();
                }
            }
        }
    }
    Ok(())
}
