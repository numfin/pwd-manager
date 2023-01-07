use crate::{app::AppState, components::AppComponent};
use eframe::{
    egui::{self, Key, Modifiers},
    epaint::{Pos2, Rect, Vec2},
};

use super::Page;

pub struct WelcomePage();
impl AppComponent for WelcomePage {
    type State = AppState;

    fn add(state: &mut Self::State, ui: &mut egui::Ui) {
        let w = ui.available_width();
        let h = ui.available_height();
        // Third window size from center
        let rect = Rect::from_center_size(Pos2::new(w / 2., h / 2.), Vec2::new(w / 3., h / 3.));
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical_centered(|ui| ui.heading("N VAULT"));
            ui.add_space(10.);
            ui.vertical_centered_justified(|ui| {
                // let state_lock = state.lock();
                // let mut state = state_lock.borrow_mut();
                let input1 = egui::TextEdit::singleline(&mut state.store.settings.server)
                    .hint_text("Vault server")
                    .margin(Vec2::new(10., 4.));
                ui.add(input1);
                let input2 = egui::TextEdit::singleline(&mut state.store.settings.key)
                    .hint_text("Your vault key")
                    .password(true)
                    .margin(Vec2::new(10., 10.));
                ui.add(input2);

                if ui.input_mut().consume_key(Modifiers::NONE, Key::Enter) {
                    println!("Pressed enter");
                }

                ui.add_space(4.);
                if ui.button("Enter").clicked() {
                    state.page = Page::Main;
                }
            });
        });
    }
}
