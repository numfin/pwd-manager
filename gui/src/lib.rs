use std::ops::Div;

mod app;
mod components;
mod egui_app;
mod mutex;
mod pages;
mod store;
mod tools;

pub async fn run() {
    tracing_subscriber::fmt::init();

    let screen_size: (f32, f32) = (2560.0, 1440.0);
    let window_size: (f32, f32) = (800.0, 480.0);
    let options = eframe::NativeOptions {
        initial_window_size: Some(window_size.into()),
        initial_window_pos: Some(
            (
                screen_size.0.div(2.0) - window_size.0.div(2.0),
                screen_size.1.div(2.0) - window_size.1.div(2.0),
            )
                .into(),
        ),
        #[cfg(debug_assertions)]
        always_on_top: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<app::App>::default()),
    )
}
