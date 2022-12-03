use eframe::egui;

pub struct Password<'a> {
    id_source: egui::Id,
    buf: &'a mut String,
}

impl<'a> Password<'a> {
    /// I've no idea how to make unique ids...so here we go
    pub fn new(id_source: impl std::hash::Hash, password: &'a mut String) -> Self {
        Self {
            id_source: egui::Id::new(id_source),
            buf: password,
        }
    }
}

impl egui::Widget for Password<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self {
            id_source,
            buf: password,
        }: Password = self;

        let state_id = ui.make_persistent_id(id_source);
        let mut show_plaintext = ui.data().get_temp::<bool>(state_id).unwrap_or(false);

        let widget_ui = ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.add(egui::TextEdit::singleline(password).password(!show_plaintext));

            let response = ui
                .selectable_label(show_plaintext, "👁")
                .on_hover_text("Show/hide password");

            if response.clicked() {
                show_plaintext = !show_plaintext;
            }
        });

        ui.data().insert_temp(state_id, show_plaintext);

        widget_ui.response
    }
}
