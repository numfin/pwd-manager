use std::sync::Arc;

use server::password::{
    get_password::{self, PrivateRecord},
    list_passwords::PublicRecord,
};

use super::{button_with_subtext::button_with_subtext, AppComponentWithProps};
use crate::{app::AppState, mutex::Mutex, tools::lazy_loader::LazyLoader};

pub struct RecordListItem<'a>(pub &'a PublicRecord);
impl<'a> AppComponentWithProps for RecordListItem<'a> {
    type State = AppState;

    fn add(&self, state: &mut Self::State, ui: &mut eframe::egui::Ui) {
        let btn = button_with_subtext(ui, &self.0.title, &self.0.login);
        let btn = ui.add(btn);
        if btn.clicked() {
            set_active_record(
                state.store.vault.active_record.clone(),
                &state.store.settings.server,
                &state.store.settings.key,
                &self.0.id,
            );
        }
    }
}

pub fn set_active_record(
    active_record_state: Arc<Mutex<LazyLoader<PrivateRecord>>>,
    server: &str,
    key: &str,
    record_id: &str,
) {
    let key = key.to_string();
    let server = server.to_string();
    let record_id = record_id.to_string();
    active_record_state.lock().update(async move {
        let req_body = get_password::GetPasswordPayload { key };
        let client = reqwest::Client::new();
        let private_record = client
            .post(format!("{}/password/{}", server, record_id))
            .json(&req_body)
            .send()
            .await?
            .json::<get_password::PrivateRecord>()
            .await?;
        Ok(private_record)
    });
}
