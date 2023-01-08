use eframe::egui::ScrollArea;
use essentials::password::ResourceDefinition;
use server::password::new_password::NewPasswordPayload;

use crate::{
    app::AppState,
    components::{
        button_action::button_action, input_with_label::input_with_label,
        record_list_item::set_active_record, AppComponent,
    },
    store::{forms::NewRecordForm, settings::StoreSettings},
    tools::lazy_loader::{LazyLoader, LazyValue},
};

use super::Page;

pub struct NewPassword;
impl AppComponent for NewPassword {
    type State = AppState;

    fn add(state: &mut Self::State, ui: &mut eframe::egui::Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.);
                ui.heading("Create new password");
                ui.add_space(30.);

                input_with_label(&mut state.store.forms.new_record.title, "Title:", ui);
                ui.add_space(4.);
                input_with_label(&mut state.store.forms.new_record.login, "Login:", ui);
                ui.add_space(4.);
                input_with_label(&mut state.store.forms.new_record.password, "Password:", ui);

                let resources = &mut state.store.forms.new_record.resources;

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
                    state.store.forms.new_record.new_resource();
                }
                ui.add_space(4.);

                match state.store.forms.new_record_save.check() {
                    LazyValue::Loading => {
                        ui.add_enabled_ui(false, |ui| {
                            button_action("Loading", ui);
                        });
                        ui.ctx().request_repaint()
                    }
                    LazyValue::Unset => {
                        if button_action("Save", ui).clicked() {
                            save_password(
                                &mut state.store.forms.new_record_save,
                                &state.store.settings,
                                &state.store.forms.new_record,
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
                        state.store.forms.new_record_save.cancel();
                        state.page = Page::Main;
                    }
                    LazyValue::Error(err) => {
                        ui.label(&err.to_string());
                    }
                };
                ui.add_space(4.);
                if button_action("Back", ui).clicked() {
                    state.store.vault.password_records.lock().cancel();
                    state.page = Page::Main;
                }

                ui.add_space(30.);
            });
        });
    }
}

pub fn save_password(
    new_record_save: &mut LazyLoader<String>,
    settings: &StoreSettings,
    form: &NewRecordForm,
) {
    let key = settings.key.clone();
    let server = settings.server.clone();
    let payload = NewPasswordPayload {
        title: form.title.clone(),
        login: form.login.clone(),
        password: form.password.clone(),
        resources: form.resources.clone(),
        key,
    };
    new_record_save.update(async move {
        let client = reqwest::Client::new();
        let record_id = client
            .post(format!("{}/password", server))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(record_id)
    });
}
