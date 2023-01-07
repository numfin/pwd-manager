use cuid::cuid;
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::cipher::EncodedMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordRecord {
    pub id: String,
    pub login: String,
    pub title: String,
    pub resources: Vec<ResourceDefinition>,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
}

impl PasswordRecord {
    pub fn new(
        title: &str,
        login: &str,
        msg: EncodedMessage,
        resources: Vec<ResourceDefinition>,
    ) -> Result<Self> {
        let id = cuid()?;
        let record = Self {
            id,
            title: title.to_string(),
            login: login.to_string(),
            resources,
            password: msg.content.0,
            salt: msg.iv.0,
        };
        Ok(record)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceDefinition {
    URL(String),
}
