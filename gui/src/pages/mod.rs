pub mod main;
pub mod new_password;
pub mod settings;
pub mod welcome;

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Welcome,
    Main,
    NewPassword,
    Settings,
}
