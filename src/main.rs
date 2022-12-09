// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// but also gets rid of stdout :-/
use anyhow::Result;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    use amect::{
        cli::args as CliArgs,
        winutils::{
            get_username, is_admin,
            misc::{
                net_set_user_elevated, set_lockscreen_img, set_profile_img,
                set_username_login_requirement,
            },
            set_password, set_username, wmic_get_session_user, wmic_get_user_sid,
        },
        AmectApp, AmectCli,
    };
    use anyhow::{bail, Context};
    use clap::Parser;
    use eframe::epaint::Vec2;
    use native_dialog::MessageDialog;

    let cli = AmectCli::parse();

    let get_session_user_err =
        "failed to retrieve username, it's likely that username has been recently modified. \
        Please try again after a relogin.";

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
        Some(CliArgs::Commands::Cli(cli)) => {
            if !is_admin() {
                bail!("admin privileges are required!");
            }

            match cli {
                CliArgs::Cli::User(user) => {
                    if user == Default::default() {
                        // default interactive mode
                        unimplemented!();
                    }

                    let (_session_domain, session_username) =
                        wmic_get_session_user().context(get_session_user_err)?;

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
                    // ! do set_username at the end; order is important
                    // so that other functions don't fail early due to a username change they didn't expect
                    if let Some(username) = user.username {
                        set_username(&session_username, &username)?;
                    }

                    // print msg when no errors
                    println!("Changes have been successfully made!");
                }
                CliArgs::Cli::Visual(visual) => {
                    if visual == Default::default() {
                        // default interactive mode
                        unimplemented!();
                    }

                    let (_session_domain, session_username) =
                        wmic_get_session_user().context(get_session_user_err)?;
                    let user_sid =
                        wmic_get_user_sid(&session_username).context(get_session_user_err)?;

                    if let Some(profile_img) = visual.profile_img {
                        set_profile_img(&user_sid, profile_img)?;
                    }
                    if let Some(lockscreen_img) = visual.lockscreen_img {
                        set_lockscreen_img(&user_sid, lockscreen_img)?;
                    }

                    // print msg when no errors
                    println!("Changes have been successfully made!");
                }
                CliArgs::Cli::Login(login) => {
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
