use eframe::{
    egui::{Button, Response, RichText, Ui},
    epaint::Vec2,
};

pub fn button_action(label: &str, ui: &mut Ui) -> Response {
    ui.add_sized(
        Vec2::new(ui.available_width().min(300.), 30.),
        Button::new(RichText::new(label).strong()),
    )
}
