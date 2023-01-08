use std::time::Duration;

use crate::{
    app::AppState,
    pages::Page,
    tools::lazy_loader::{LazyLoader, LazyValue},
};

use super::{
    button_action::button_action, record_list_item::RecordListItem, AppComponent,
    AppComponentWithProps,
};
use eframe::{
    egui::{self, style::Margin, Frame, ScrollArea},
    epaint::Vec2,
};
use eyre::Result;
use server::password::list_passwords::{self, PublicRecord};

pub struct PasswordList;
impl PasswordList {
    pub async fn load_passwords(url: &str, query: String) -> Result<Vec<PublicRecord>> {
        let url = url.to_string();
        let query = query.to_string();
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .query(&[("search", query)])
            .send()
            .await?
            .error_for_status()?
            .json::<list_passwords::ListPasswordsResponse>()
            .await?;

        Ok(response.records)
    }
}
impl AppComponent for PasswordList {
    type State = AppState;

    fn add(state: &mut Self::State, ui: &mut egui::Ui) {
        let loader = LazyLoader::load_sync(state.store.vault.password_records.clone(), async {
            let response = reqwest::get("http://localhost:3000/password?search=")
                .await?
                .error_for_status()?
                .json::<list_passwords::ListPasswordsResponse>()
                .await?;

            Ok(response.records)
        });

        egui::SidePanel::left("password list")
            .resizable(false)
            .exact_width(300.)
            .frame(Frame::default().inner_margin(Margin {
                right: 10.,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                // let state_lock = state.lock();
                // let mut state_lock = state_lock.borrow_mut();
                if ui
                    .add_sized(
                        [ui.available_width(), 0.],
                        egui::TextEdit::singleline(&mut state.store.vault.search_query)
                            .hint_text("Find your password")
                            .margin(Vec2::new(10., 10.)),
                    )
                    .changed()
                {
                    // state.store.vault.password_records.lock().cancel();
                    state
                        .store
                        .vault
                        .password_records
                        .lock()
                        .update(PasswordList::load_passwords(
                            "http://localhost:3000/password",
                            state.store.vault.search_query.clone(),
                        ));
                };
                ui.add_space(10.);

                match &loader.lock().check() {
                    LazyValue::Loaded(data) => {
                        ui.label(format!("{} records", data.len()));
                        ui.add_space(10.);
                        ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .max_height(ui.available_height() - 44.)
                            .show(ui, |ui| {
                                for record in data {
                                    RecordListItem(record).add(state, ui);
                                }
                            });

                        ui.add_space(10.);

                        if button_action("New password", ui).clicked() {
                            state.page = Page::NewPassword;
                        }
                    }
                    LazyValue::Error(err) => {
                        ui.label(err.to_string());
                    }
                    _ => {
                        ui.spinner();
                        ui.ctx()
                            .request_repaint_after(Duration::from_secs_f32(0.05));
                    }
                }
            });
    }
}
