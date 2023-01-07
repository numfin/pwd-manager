use eframe::egui::Ui;

pub mod button_action;
pub mod button_with_subtext;
pub mod copy_text;
pub mod input_with_label;
pub mod left_panel;
pub mod record_list_item;
pub mod record_view;

pub trait AppComponent {
    type State;

    #[allow(unused_variables)]
    fn add(state: &mut Self::State, ui: &mut Ui) {}
}
pub trait AppComponentWithProps {
    type State;

    #[allow(unused_variables)]
    fn add(&self, state: &mut Self::State, ui: &mut Ui) {}
}
