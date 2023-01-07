use crate::{pages::Page, store::Store};

#[derive(Default)]
pub struct App {
    pub state: AppState,
}

#[derive(Default)]
pub struct AppState {
    pub store: Store,
    pub page: Page,
}
