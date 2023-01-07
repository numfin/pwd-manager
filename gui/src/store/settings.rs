#[derive(Debug)]
pub struct StoreSettings {
    pub key: String,
    pub server: String,
}

impl Default for StoreSettings {
    fn default() -> Self {
        Self {
            server: "http://localhost:3000".to_string(),
            key: "qweqweqwe".to_string(),
        }
    }
}
