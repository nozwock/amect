use eframe::egui;

pub struct Switch<'a>(&'a mut bool);

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self(on)
    }
}

/// from https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/toggle_switch.rs
impl egui::Widget for Switch<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self(on) = self;

        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }
        response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }
}
