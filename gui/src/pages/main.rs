
use eframe::egui::CentralPanel;

use crate::{
    app::{ AppState},
    components::{left_panel::PasswordList, record_view::RecordView, AppComponent},
};

pub struct MainPage;
impl AppComponent for MainPage {
    type State = AppState;

    fn add(state: &mut Self::State, ui: &mut eframe::egui::Ui) {
        PasswordList::add(state, ui);
        CentralPanel::default().show_inside(ui, |ui| {
            RecordView::add(state, ui);
        });
    }
}
