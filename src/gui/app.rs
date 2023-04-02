use eframe::egui;
use libamect_common::utils::pick_image_file;

#[derive(Debug, Default)]
pub struct AmectApp {
    // user states
    username: String,
    user_pass: String,
    admin_pass: String,
    username_login: bool,
    autologin: bool,
}

impl AmectApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        configure_fonts(&cc.egui_ctx);

        Default::default()
    }
}

fn configure_fonts(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    // * Other ways to increase font size but is set for every element
    // style.override_font_id = Some(egui::FontId::proportional(24.));
    // for (_text_style, font_id) in style.text_styles.iter_mut() {
    //     font_id.size = 16.;
    // }

    style
        .text_styles
        .get_mut(&egui::TextStyle::Body)
        .unwrap()
        .size = 16.;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Heading)
        .unwrap()
        .size = 28.;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Button)
        .unwrap()
        .size = 18.;
    ctx.set_style(style);
}

impl eframe::App for AmectApp {
    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            user_pass: test_pass,
            admin_pass,
            username,
            username_login,
            autologin,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Apply changes").clicked() {}
            ui.separator();

            ui.label("GUI is not really functional atm; WIP");

            ui.collapsing("User", |ui| {
                egui::Grid::new("user_grid")
                    .num_columns(2)
                    .spacing([40., 4.])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("New username:");
                        ui.text_edit_singleline(username);
                        ui.end_row();

                        ui.label("New Password:");
                        ui.add(super::widgets::Password::new("user-hidepass", test_pass));
                        ui.end_row();

                        ui.label("New Admin Password:");
                        ui.add(super::widgets::Password::new("admin-hidepass", admin_pass));
                        ui.end_row();
                    });
            });

            ui.collapsing("Appearence", |ui| {
                egui::Grid::new("appearence_grid")
                    .num_columns(2)
                    .spacing([40., 4.])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Set new lockscreen image");
                        if ui.button("🗁 Browse").clicked() {
                            dbg!(pick_image_file());
                        }
                        ui.end_row();

                        ui.label("Set new profile image");
                        if ui.button("🗁 Browse").clicked() {
                            dbg!(pick_image_file());
                        }
                        ui.end_row();
                    });
            });

            ui.collapsing("Permissions", |ui| {
                ui.label("WIP");
            });

            ui.collapsing("Login", |ui| {
                egui::Grid::new("login_grid")
                    .num_columns(2)
                    .spacing([40., 4.])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Username prompt on login");
                        ui.add(super::widgets::Switch::new(username_login));
                        ui.end_row();

                        ui.label("Auto login");
                        ui.add(super::widgets::Switch::new(autologin));
                        ui.end_row();
                    });
            });
        });
    }
}
