// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// but also gets rid of stdout :-/
use anyhow::Result;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    use amect::{
        winutil::{
            get_username, is_admin, net_set_user_elevated, set_password, set_username,
            set_username_login_requirement, wmic_get_session_user,
        },
        AmectApp, AmectCli,
    };
    use anyhow::{bail, Context};
    use clap::Parser;
    use eframe::epaint::Vec2;
    use native_dialog::MessageDialog;

    let cli = AmectCli::parse();

    match cli.command {
        None => {
            println!(
                "NOTE: This console windows is not ideal, \
            I know but please bear with it as there's really \
            no good solution for hiding it, atleast among those I'm aware of.\n\
            I could seperate out the cli and gui each into their own \
            seperate binaries which would allow me to hide the console window for the gui but I haven't decided on that."
            );

            if !is_admin() {
                MessageDialog::new()
                    .set_type(native_dialog::MessageType::Error)
                    .set_title("Error")
                    .set_text("admin privileges are required!")
                    .show_alert()?;
                bail!("admin privileges are required!");
            }

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
        Some(amect::args::Commands::Cli(cli)) => {
            if !is_admin() {
                bail!("admin privileges are required!");
            }

            match cli {
                amect::args::Cli::User(user) => {
                    if user == Default::default() {
                        // default interactive mode
                        unimplemented!();
                    }

                    let (_session_domain, session_username) = wmic_get_session_user().context(
                        "\
                failed to retrieve username, it's likely that username has been recently modified. \
                Please try again after a relogin.",
                    )?;

                    if let Some(username) = user.username {
                        set_username(&session_username, &username)?;
                    }
                    if let Some(password) = user.user_password {
                        set_password(&session_username, &password)?;
                    }
                    if let Some(password) = user.admin_password {
                        set_password(
                            &get_username().context("failed to retrieve username")?,
                            &password,
                        )?;
                    }
                    if let Some(elevate_user) = user.elevate_user {
                        net_set_user_elevated(elevate_user, &session_username)?;
                    }

                    // print msg when no errors
                    println!("Changes have been successfully made!");
                }
                amect::args::Cli::Visual(visual) => {
                    if visual == Default::default() {
                        // default interactive mode
                        unimplemented!();
                    }

                    if let Some(_profile_img) = visual.profile_img {
                        unimplemented!();
                    }
                    if let Some(_lockscreen_img) = visual.lockscreen_img {
                        unimplemented!();
                    }

                    // print msg when no errors
                    println!("Changes have been successfully made!");
                }
                amect::args::Cli::Login(login) => {
                    if login == Default::default() {
                        // default interactive mode
                        unimplemented!();
                    }

                    if let Some(require_username) = login.require_username {
                        set_username_login_requirement(require_username)?;
                    }
                    if let Some(_auto_login) = login.auto_login {
                        unimplemented!();
                    }

                    // print msg when no errors
                    println!("Changes have been successfully made!");
                }
            }
        }
    }

    Ok(())
}
