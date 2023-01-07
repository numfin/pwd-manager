use std::{collections::HashMap, sync::Arc};

use server::password::{get_password, list_passwords};

use crate::{mutex::Mutex, tools::lazy_loader::LazyLoader};

#[derive(Default)]
pub struct StoreVault {
    pub search_query: String,
    pub password_records: Arc<Mutex<LazyLoader<Vec<list_passwords::PublicRecord>>>>,
    pub active_record: Arc<Mutex<LazyLoader<get_password::PrivateRecord>>>,
    pub visible_fields: VisibleFields,
}

#[derive(Default)]
pub struct VisibleFields(HashMap<String, HashMap<String, bool>>);
impl VisibleFields {
    pub fn get_mut(&mut self) -> &mut HashMap<String, HashMap<String, bool>> {
        &mut self.0
    }
}
