use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use essentials::{
    cipher::{Cipher, EncodedMessage},
    key::Key,
    password::{PasswordRecord, ResourceDefinition},
};
use serde::{Deserialize, Serialize};

use crate::{err::AppResult, meili::AppIndex, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateRecord {
    pub id: String,
    pub title: String,
    pub login: String,
    pub password: String,
    pub resources: Vec<ResourceDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPasswordPayload {
    pub key: String,
}

pub async fn route(
    State(state): State<Arc<AppState>>,
    Path(document_id): Path<String>,
    Json(payload): Json<GetPasswordPayload>,
) -> AppResult<Json<PrivateRecord>> {
    let index = state.meili.index(AppIndex::PasswordRecords);
    let record = index.get_document::<PasswordRecord>(&document_id).await?;
    let encoded_msg = EncodedMessage::from_record(&record);
    let key = Key::new(&payload.key);
    let decoded = Cipher::decrypt(&encoded_msg, &key)?;

    let private_record = PrivateRecord {
        id: record.id,
        title: record.title,
        login: record.login,
        password: decoded,
        resources: record.resources,
    };

    Ok(Json(private_record))
}
