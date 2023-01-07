use eframe::{
    egui::{Button, TextFormat, Ui},
    epaint::{text::LayoutJob, Color32, FontFamily, FontId, Vec2},
};

pub fn button_with_subtext(ui: &mut Ui, main_text: &str, secondary_text: &str) -> Button {
    ui.style_mut().spacing.button_padding = Vec2::new(10., 8.);

    let mut job = LayoutJob::default();
    job.append(
        main_text,
        0.0,
        TextFormat {
            color: Color32::LIGHT_GRAY,
            ..Default::default()
        },
    );
    job.append("\n", 0.0, TextFormat::default());
    job.append(
        secondary_text,
        0.0,
        TextFormat {
            font_id: FontId::new(12.0, FontFamily::default()),
            color: Color32::GRAY,
            ..Default::default()
        },
    );
    Button::new(job).min_size([ui.available_width(), 24.].into())
}
