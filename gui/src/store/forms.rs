use essentials::password::ResourceDefinition;

use crate::tools::lazy_loader::LazyLoader;

#[derive(Default)]
pub struct StoreForms {
    pub new_record: NewRecordForm,
    pub new_record_save: LazyLoader<String>,
}

pub struct NewRecordForm {
    pub title: String,
    pub login: String,
    pub password: String,
    pub resources: Vec<ResourceDefinition>,
}

impl Default for NewRecordForm {
    fn default() -> Self {
        let mut form = Self {
            title: String::new(),
            login: String::new(),
            password: String::new(),
            resources: vec![],
        };
        form.new_resource();
        form
    }
}

impl NewRecordForm {
    pub fn new_resource(&mut self) {
        self.resources.push(ResourceDefinition::URL("".to_string()));
    }
}
