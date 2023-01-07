use eframe::egui::{TextBuffer, TextEdit, Ui};

pub fn input_with_label(value: &mut dyn TextBuffer, label: &str, ui: &mut Ui) {
    // ui.add_visible_ui(value.as_str().len() > 0, |ui| {
    //     ui.label(label);
    // });
    let input = TextEdit::singleline(value)
        .hint_text(label)
        .margin([10., 10.].into());
    ui.add(input);
}
