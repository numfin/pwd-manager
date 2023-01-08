pub mod edit_password;
pub mod main;
pub mod new_password;
pub mod settings;
pub mod welcome;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum Page {
    Welcome,
    Main,
    NewPassword,
    EditPassword(String),
    Settings,
}
impl Default for Page {
    fn default() -> Self {
        Self::Welcome
    }
}
