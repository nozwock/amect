// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// but also gets rid of stdout :-/
use anyhow::Result;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    use amect::{args::AmectCli, AmectApp};
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
            amect::args::Cli::Users(users) => {}
            amect::args::Cli::Visuals(visuals) => {}
            amect::args::Cli::Login(login) => {}
        },
    }

    Ok(())
}
