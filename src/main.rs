// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// but also gets rid of stdout :-/
use anyhow::Result;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    use amect::{
        cli::{
            args as CliArgs,
            interact::{
                AutoLoginOptions, LoginOptions, UserElevationOptions, UserOptions,
                UserRequirementOptions, VisualOptions,
            },
        },
        utils::browse_image_file,
        winutils::{
            get_username, is_admin,
            misc::{
                disable_autologon, enable_autologon, net_set_user_elevated, set_lockscreen_img,
                set_profile_img, set_username_login_requirement,
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
                initial_window_size: Some(Vec2::new(550., 400.)),
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
                    let (_session_domain, session_username) =
                        wmic_get_session_user().context(get_session_user_err)?;

                    if user == Default::default() {
                        // default interactive mode
                        let choice = inquire::Select::new(
                            "Select an option:",
                            vec![
                                UserOptions::SetUsername,
                                UserOptions::SetPassword,
                                UserOptions::SetAdminPassword,
                                UserOptions::UserElevation,
                            ],
                        )
                        .prompt()?;

                        match choice {
                            UserOptions::SetUsername => {
                                let username = inquire::Text::new("New username:").prompt()?;
                                set_username(&session_username, &username)?;
                            }
                            UserOptions::SetPassword => {
                                let password = inquire::Password::new("New user password:")
                                    .with_display_mode(inquire::PasswordDisplayMode::Masked)
                                    .prompt()?;
                                set_password(&session_username, &password)?;
                            }
                            UserOptions::SetAdminPassword => {
                                let password = inquire::Password::new("New admin password:")
                                    .with_display_mode(inquire::PasswordDisplayMode::Masked)
                                    .prompt()?;
                                set_password(
                                    &get_username().context("failed to retrieve username")?,
                                    &password,
                                )?;
                            }
                            UserOptions::UserElevation => {
                                let choice = inquire::Select::new(
                                    "Select an option:",
                                    vec![
                                        UserElevationOptions::Enable,
                                        UserElevationOptions::Disable,
                                    ],
                                )
                                .prompt()?;

                                match choice {
                                    UserElevationOptions::Enable => {
                                        net_set_user_elevated(true, &session_username)?;
                                    }
                                    UserElevationOptions::Disable => {
                                        net_set_user_elevated(false, &session_username)?;
                                    }
                                }
                            }
                        }

                        // print msg when no errors
                        println!("Changes have been successfully made!");
                        return Ok(());
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
                    // ! do set_username at the end; order is important
                    // so that other functions don't fail early due to a username change they didn't expect
                    if let Some(username) = user.username {
                        set_username(&session_username, &username)?;
                    }

                    // print msg when no errors
                    println!("Changes have been successfully made!");
                }
                CliArgs::Cli::Visual(visual) => {
                    let (_session_domain, session_username) =
                        wmic_get_session_user().context(get_session_user_err)?;
                    let user_sid =
                        wmic_get_user_sid(&session_username).context(get_session_user_err)?;

                    if visual == Default::default() {
                        // default interactive mode
                        let choice = inquire::Select::new(
                            "Select an option:",
                            vec![VisualOptions::SetProfile, VisualOptions::SetLockscreen],
                        )
                        .prompt()?;

                        match choice {
                            VisualOptions::SetProfile => {
                                let img =
                                    browse_image_file().context("You must select an image!")?;
                                set_profile_img(&user_sid, img)?;
                            }
                            VisualOptions::SetLockscreen => {
                                let img =
                                    browse_image_file().context("You must select an image!")?;
                                set_lockscreen_img(&user_sid, img)?;
                            }
                        }

                        // print msg when no errors
                        println!("Changes have been successfully made!");
                        return Ok(());
                    }

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
                    let (_session_domain, session_username) =
                        wmic_get_session_user().context(get_session_user_err)?;

                    if login == Default::default() {
                        // default interactive mode
                        let choice = inquire::Select::new(
                            "Select an option:",
                            vec![LoginOptions::UserRequirement, LoginOptions::AutoLogin],
                        )
                        .prompt()?;

                        match choice {
                            LoginOptions::UserRequirement => {
                                let choice = inquire::Select::new(
                                    "Select an option:",
                                    vec![
                                        UserRequirementOptions::Enable,
                                        UserRequirementOptions::Disable,
                                    ],
                                )
                                .prompt()?;

                                match choice {
                                    UserRequirementOptions::Enable => {
                                        set_username_login_requirement(true)?;
                                    }
                                    UserRequirementOptions::Disable => {
                                        set_username_login_requirement(false)?;
                                    }
                                }
                            }
                            LoginOptions::AutoLogin => {
                                let choice = inquire::Select::new(
                                    "Select an option:",
                                    vec![AutoLoginOptions::Enable, AutoLoginOptions::Disable],
                                )
                                .prompt()?;

                                match choice {
                                    AutoLoginOptions::Enable => {
                                        let password =
                                            inquire::Password::new("Enter your user password:")
                                                .with_display_mode(
                                                    inquire::PasswordDisplayMode::Masked,
                                                )
                                                .without_confirmation()
                                                .prompt()?;
                                        enable_autologon(&session_username, &password)?;
                                    }
                                    AutoLoginOptions::Disable => {
                                        disable_autologon()?;
                                    }
                                }
                            }
                        }

                        // print msg when no errors
                        println!("Changes have been successfully made!");
                        return Ok(());
                    }

                    if let Some(require_username) = login.require_username {
                        set_username_login_requirement(require_username)?;
                    }
                    if let Some(auto_login) = login.auto_login {
                        if auto_login {
                            enable_autologon(
                                &session_username,
                                &login.password.expect(
                                    "--password will always be passed along with --auto-login true",
                                ),
                            )?;
                        } else {
                            disable_autologon()?;
                        }
                    }

                    // print msg when no errors
                    println!("Changes have been successfully made!");
                }
            }
        }
    }

    Ok(())
}
