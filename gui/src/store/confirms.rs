use crate::tools::lazy_loader::LazyLoader;

#[derive(Default)]
pub struct StoreConfirms {
    pub delete_password: Confirmation<bool>,
    pub delete_password_req: LazyLoader<()>,
}

#[derive(Default)]
pub enum Confirmation<T> {
    #[default]
    Unset,
    Asked,
    Answered(T),
    Closed,
}
impl<T> Confirmation<T> {
    pub fn ask(&mut self) {
        *self = Self::Asked;
    }
    pub fn answer(&mut self, response: T) {
        *self = Self::Answered(response);
    }
    pub fn close(&mut self) {
        *self = Self::Closed;
    }
    pub fn cancel(&mut self) {
        *self = Self::Unset;
    }
}
