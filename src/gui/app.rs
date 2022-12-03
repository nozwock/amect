use eframe::{egui, epaint::Vec2};

#[derive(Debug, Default)]
pub struct AMEApp {
    // tmp states
    user_pass: String,
    admin_pass: String,
    username: String,
}

impl AMEApp {
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

        Default::default()
    }
}

impl eframe::App for AMEApp {
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
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
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
                ui.separator();
                if ui.button("Apply changes").clicked() {}
            });

            ui.collapsing("Appearence", |ui| {});

            ui.collapsing("Permissions", |ui| {});

            ui.collapsing("Login", |ui| {});
        });
    }
}
