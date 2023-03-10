use self::{
    confirms::StoreConfirms, forms::StoreForms, settings::StoreSettings, vault::StoreVault,
};

pub mod confirms;
pub mod forms;
pub mod settings;
pub mod vault;

#[derive(Default)]
pub struct Store {
    pub vault: StoreVault,
    pub settings: StoreSettings,
    pub forms: StoreForms,
    pub confirms: StoreConfirms,
}
