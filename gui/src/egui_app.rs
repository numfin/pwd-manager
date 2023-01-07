use eframe::egui::CentralPanel;

use crate::{
    app::App,
    components::AppComponent,
    pages::{main::MainPage, new_password::NewPassword, welcome::WelcomePage, Page},
};

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| match self.state.page {
            Page::Welcome => {
                WelcomePage::add(&mut self.state, ui);
            }
            Page::Main => {
                MainPage::add(&mut self.state, ui);
            }
            Page::NewPassword => {
                NewPassword::add(&mut self.state, ui);
            }
            _ => {}
        });
    }
}
