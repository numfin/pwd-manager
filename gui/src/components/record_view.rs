use eframe::{
    egui::{Grid, Ui, Window},
    emath::Align2,
};
use essentials::password::ResourceDefinition;
use server::password::get_password::PrivateRecord;

use crate::{
    app::AppState,
    pages::Page,
    store::{confirms::Confirmation, settings::StoreSettings, vault::VisibleFields},
    tools::lazy_loader::{LazyLoader, LazyValue},
};

use super::{copy_text::copy_text, AppComponent};

pub struct RecordView;
impl AppComponent for RecordView {
    type State = AppState;

    fn add(state: &mut Self::State, ui: &mut eframe::egui::Ui) {
        match state.store.vault.active_record.clone().lock().check() {
            LazyValue::Loaded(data) => {
                ui.horizontal(|ui| {
                    if ui.button("Change").clicked() {
                        state.page = Page::EditPassword(data.id.clone());
                    }
                    if ui.button("Delete").clicked() {
                        state.store.confirms.delete_password.ask();
                    }
                });

                let delete_password = &mut state.store.confirms.delete_password;
                match delete_password {
                    Confirmation::Asked => {
                        Window::new("Delete window")
                            .resizable(false)
                            .title_bar(false)
                            .anchor(Align2::CENTER_CENTER, [0., 0.])
                            .show(ui.ctx(), |ui| {
                                ui.label(format!("Delete record {}", data.title));
                                ui.add_space(10.);
                                ui.horizontal(|ui| {
                                    if ui.button("Delete").clicked() {
                                        delete_password.answer(true);
                                    }
                                    if ui.button("Cancel").clicked() {
                                        delete_password.answer(false);
                                        state.store.confirms.delete_password_req.cancel();
                                    }
                                });
                            });
                    }
                    Confirmation::Answered(response) => {
                        if *response {
                            Window::new("Delete window")
                                .resizable(false)
                                .title_bar(false)
                                .enabled(!state.store.confirms.delete_password_req.is_loading())
                                .anchor(Align2::CENTER_CENTER, [0., 0.])
                                .show(ui.ctx(), |ui| {
                                    let delete_password_req =
                                        &mut state.store.confirms.delete_password_req;
                                    match delete_password_req.check() {
                                        LazyValue::Unset => {
                                            delete_password_api(
                                                delete_password_req,
                                                &mut state.store.settings,
                                                &data.id,
                                            );
                                        }
                                        LazyValue::Loading => {
                                            ui.spinner();
                                        }
                                        LazyValue::Loaded(_) => {
                                            delete_password.close();
                                            state.store.vault.active_record = Default::default();
                                            if let Some(inner_data) = state
                                                .store
                                                .vault
                                                .password_records
                                                .lock()
                                                .extract_data()
                                            {
                                                let found = inner_data
                                                    .iter()
                                                    .enumerate()
                                                    .find(|(_, rec)| rec.id == data.id);
                                                if let Some((pos, _)) = found {
                                                    inner_data.remove(pos);
                                                }
                                            }
                                        }
                                        LazyValue::Error(err) => {
                                            ui.label(err.to_string());

                                            ui.horizontal(|ui| {
                                                if ui.button("Delete").clicked() {
                                                    delete_password_api(
                                                        delete_password_req,
                                                        &mut state.store.settings,
                                                        &data.id,
                                                    );
                                                }
                                                if ui.button("Cancel").clicked() {
                                                    delete_password.answer(false);
                                                }
                                            });
                                        }
                                    }
                                });
                        } else {
                            delete_password.close();
                        }
                    }
                    Confirmation::Closed => {
                        state.store.confirms.delete_password_req.cancel();
                        delete_password.cancel();
                    }
                    Confirmation::Unset => {}
                }

                ui.add_space(10.);

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
                ui.ctx().request_repaint();
            }
            LazyValue::Unset => {
                ui.label("Choose record to see password");
            }
        }
    }
}

fn delete_password_api(loader: &mut LazyLoader<()>, settings: &mut StoreSettings, record_id: &str) {
    let server = settings.server.clone();
    let id = record_id.to_owned();
    loader.update(async move {
        let client = reqwest::Client::new();
        let _ = client
            .delete(format!("{server}/password/{id}"))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    });
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
