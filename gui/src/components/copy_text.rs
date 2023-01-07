use std::ops::Not;

use eframe::egui::{self, InnerResponse, RichText, Ui};

pub fn copy_text(ui: &mut Ui, value: &str, secret: Option<&mut bool>) -> InnerResponse<()> {
    egui::Frame::default().show(ui, |ui| {
        match secret {
            Some(false) => {
                ui.label(RichText::new("*".repeat(value.len())).monospace());
            }
            _ => {
                ui.label(RichText::new(value).monospace());
            }
        };
        if ui.button("copy").clicked() {
            ui.output().copied_text = value.to_owned();
        }
        if let Some(hide_secret) = secret {
            let toggle_label = match hide_secret {
                true => "hide",
                false => "show",
            };
            if ui.button(toggle_label).clicked() {
                *hide_secret = hide_secret.not();
            }
        }
    })
}
