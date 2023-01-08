use eframe::egui::ScrollArea;
use essentials::password::ResourceDefinition;
use server::password::{
    edit_password::EditPasswordPayload, get_password, new_password::NewPasswordPayload,
};

use crate::{
    app::AppState,
    components::{
        button_action::button_action, input_with_label::input_with_label,
        record_list_item::set_active_record, AppComponent, AppComponentWithProps,
    },
    store::{
        forms::{EditRecordForm, NewRecordForm},
        settings::StoreSettings,
    },
    tools::lazy_loader::{LazyLoader, LazyValue},
};

use super::Page;

pub struct EditPassword {
    pub record_id: String,
}
impl AppComponentWithProps for EditPassword {
    type State = AppState;

    fn add(&self, state: &mut Self::State, ui: &mut eframe::egui::Ui) {
        {
            let server = state.store.settings.server.clone();
            let key = state.store.settings.key.clone();
            let record_id = self.record_id.to_owned();
            state.store.forms.edit_record.load(async move {
                let req_body = get_password::GetPasswordPayload { key };
                let client = reqwest::Client::new();
                let private_record = client
                    .post(format!("{}/password/{}", server, record_id))
                    .json(&req_body)
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<get_password::PrivateRecord>()
                    .await?;
                Ok(EditRecordForm::new(private_record))
            });
        }
        match state.store.forms.edit_record.check() {
            LazyValue::Loading => {
                ui.spinner();
            }
            LazyValue::Loaded(_) => {
                // Used lower
            }
            LazyValue::Error(err) => {
                ui.label(err.to_string());
            }
            LazyValue::Unset => {}
        }
        let edit_record = &mut state.store.forms.edit_record;
        if let Some(form) = edit_record.extract_data() {
            ScrollArea::new([false, true]).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.);
                    ui.heading("Edit password");
                    ui.add_space(30.);

                    input_with_label(&mut form.record.title, "Title:", ui);
                    ui.add_space(4.);
                    input_with_label(&mut form.record.login, "Login:", ui);
                    ui.add_space(4.);
                    input_with_label(&mut form.record.password, "Password:", ui);

                    let resources = &mut form.record.resources;

                    for (pos, resource) in resources.iter_mut().enumerate() {
                        ui.add_space(4.);
                        match resource {
                            ResourceDefinition::URL(url) => {
                                input_with_label(url, &format!("Resource #{pos}"), ui);
                            }
                        }
                    }
                    ui.add_space(30.);
                    if button_action("Add resource", ui).clicked() {
                        form.new_resource();
                    }
                    ui.add_space(4.);

                    match state.store.forms.edit_record_save.check() {
                        LazyValue::Loading => {
                            ui.add_enabled_ui(false, |ui| {
                                button_action("Loading", ui);
                            });
                            ui.ctx().request_repaint()
                        }
                        LazyValue::Unset => {
                            if button_action("Save", ui).clicked() {
                                save_password(
                                    &mut state.store.forms.edit_record_save,
                                    &state.store.settings,
                                    form,
                                    self.record_id.clone(),
                                );
                            }
                        }
                        LazyValue::Loaded(record_id) => {
                            set_active_record(
                                state.store.vault.active_record.clone(),
                                &state.store.settings.server,
                                &state.store.settings.key,
                                &record_id,
                            );
                            state.store.vault.password_records.lock().cancel();
                            state.page = Page::Main;
                        }
                        LazyValue::Error(err) => {
                            ui.label(&err.to_string());
                        }
                    };
                    ui.add_space(4.);
                    if button_action("Back", ui).clicked() {
                        state.store.vault.password_records.lock().cancel();
                        // state.store.forms.edit_record_save.cancel();
                        // state.store.forms.edit_record.cancel();
                        state.page = Page::Main;
                    }

                    ui.add_space(30.);
                });
            });
        }
    }
}

pub fn save_password(
    edit_save: &mut LazyLoader<String>,
    settings: &StoreSettings,
    form: &EditRecordForm,
    record_id: String,
) {
    let key = settings.key.clone();
    let server = settings.server.clone();
    let payload = EditPasswordPayload {
        title: form.record.title.clone(),
        login: form.record.login.clone(),
        password: form.record.password.clone(),
        resources: form.record.resources.clone(),
        key,
    };
    edit_save.update(async move {
        let client = reqwest::Client::new();
        let record_id = client
            .put(format!("{}/password/{}", server, record_id))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(record_id)
    });
}
