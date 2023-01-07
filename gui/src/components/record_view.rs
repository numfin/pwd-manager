use eframe::egui::{Grid, Ui};
use essentials::password::ResourceDefinition;
use server::password::get_password::PrivateRecord;

use crate::{app::AppState, store::vault::VisibleFields, tools::lazy_loader::LazyValue};

use super::{copy_text::copy_text, AppComponent};

pub struct RecordView;
impl AppComponent for RecordView {
    type State = AppState;

    fn add(state: &mut Self::State, ui: &mut eframe::egui::Ui) {
        match state.store.vault.active_record.lock().check() {
            LazyValue::Loaded(data) => {
                Grid::new("credentials")
                    .num_columns(2)
                    .spacing([0., 10.])
                    .show(ui, |ui| {
                        loaded_record_view(ui, data, &mut state.store.vault.visible_fields);
                    });
            }
            LazyValue::Error(err) => {
                ui.label(&err.to_string());
            }
            LazyValue::Loading => {
                ui.spinner();
            }
            LazyValue::Unset => {
                ui.label("Choose record to see password");
            }
        }
    }
}

fn loaded_record_view(ui: &mut Ui, data: &PrivateRecord, visible_records: &mut VisibleFields) {
    ui.label("Title: ");
    ui.label(&data.title);
    ui.end_row();
    ui.separator();
    ui.separator();
    ui.end_row();

    ui.label("Login: ");
    copy_text(ui, &data.login, None);
    ui.end_row();

    ui.label("Password: ");
    // let state = state.lock();
    // let mut show_password = state.borrow_mut();
    let show_password = visible_records
        .get_mut()
        .entry(data.id.clone())
        .or_default()
        .entry("password".to_string())
        .or_insert(false);
    copy_text(ui, &data.password, Some(show_password));
    ui.end_row();
    ui.separator();
    ui.separator();
    ui.end_row();

    for (pos, resource) in data.resources.iter().enumerate() {
        ui.label(format!("Resource #{pos}: "));
        match resource {
            ResourceDefinition::URL(url) => {
                ui.label(url);
            }
        }
        ui.end_row();
    }
}
