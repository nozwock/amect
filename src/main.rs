// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// but also gets rid of stdout :-/
use anyhow::Result;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    use amect::{
        winutil::{
            disable_username_login_req, enable_username_login_req, get_username, net_user_elevate,
            net_user_unelevate, set_password, set_username, wmic_get_session_user,
        },
        AmectApp, AmectCli,
    };
    use anyhow::Context;
    use clap::Parser;
    use eframe::epaint::Vec2;

    let cli = AmectCli::parse();

    match cli.command {
        None => {
            println!(
                "NOTE: This console windows is not ideal, \
            I know but please bear with it as there's really \
            no good solution for hiding it, atleast those I'm aware of."
            );
            println!(
                "I could seperate out the cli and gui each into their own \
            seperate binaries which would allow me to hide the console window indirectly but I haven't decided on that."
            );

            let native_options = eframe::NativeOptions {
                initial_window_size: Some(Vec2::new(510., 400.)),
                min_window_size: Some(Vec2::new(400., 200.)),
                ..Default::default()
            };
            eframe::run_native(
                "Central AME toolkit",
                native_options,
                Box::new(|cc| Box::new(AmectApp::new(cc))),
            );
        }
        Some(amect::args::Commands::Cli(cli)) => match cli {
            amect::args::Cli::Users(users) => {
                if users == Default::default() {
                    // default interactive mode
                    unimplemented!();
                    return Ok(());
                }

                let (session_domain, session_username) = wmic_get_session_user().context(
                    "\
                failed to retrieve username, it's likely that username has been recently modified. \
                Please try again after a relogin.",
                )?;

                if let Some(username) = users.username {
                    set_username(&session_username, &username)?;
                }
                if let Some(password) = users.user_password {
                    set_password(&session_username, &password)?;
                }
                if let Some(password) = users.admin_password {
                    set_password(
                        &get_username().context("failed to retrieve username")?,
                        &password,
                    )?;
                }
                if let Some(elevate_user) = users.elevate_user {
                    match elevate_user {
                        true => net_user_elevate(&session_username)?,
                        false => net_user_unelevate(&session_username)?,
                    };
                }

                // print msg when no errors
                println!("Changes have been successfully made!");
            }
            amect::args::Cli::Visuals(visuals) => {
                if visuals == Default::default() {
                    // default interactive mode
                    unimplemented!();
                    return Ok(());
                }

                if let Some(profile_img) = users.profile_img {
                    unimplemented!();
                }
                if let Some(lockscreen_img) = visuals.lockscreen_img {
                    unimplemented!();
                }

                // print msg when no errors
                println!("Changes have been successfully made!");
            }
            amect::args::Cli::Login(login) => {
                if login == Default::default() {
                    // default interactive mode
                    unimplemented!();
                    return Ok(());
                }

                if let Some(require_username) = login.require_username {
                    match require_username {
                        true => enable_username_login_req()?,
                        false => disable_username_login_req()?,
                    };
                }
                if let Some(auto_login) = login.auto_login {
                    unimplemented!();
                }

                // print msg when no errors
                println!("Changes have been successfully made!");
            }
        },
    }

    Ok(())
}
