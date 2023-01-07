use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{extract::State, Json};
use essentials::{
    cipher::Cipher,
    key::Key,
    password::{PasswordRecord, ResourceDefinition},
};
use serde::{Deserialize, Serialize};

use crate::{err::AppResult, meili::AppIndex, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPasswordPayload {
    pub title: String,
    pub key: String,
    pub login: String,
    pub password: String,
    pub resources: Vec<ResourceDefinition>,
}

pub async fn route(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewPasswordPayload>,
) -> AppResult<String> {
    let index = state.meili.index(AppIndex::PasswordRecords);
    let time = Instant::now();
    let key = Key::new(&payload.key);
    println!("key encrypt: {:#?}", time.elapsed());
    let time = Instant::now();
    let encoded_password = Cipher::encrypt(&payload.password, &key)?;
    println!("password encrypt: {:#?}", time.elapsed());

    let time = Instant::now();
    let record = PasswordRecord::new(
        &payload.title,
        &payload.login,
        encoded_password,
        payload.resources,
    )?;
    let record_id = record.id.clone();

    let _ = index
        .add_documents(&[record], None)
        .await?
        .wait_for_completion(
            &state.meili.client,
            Some(Duration::from_secs_f32(0.1)),
            None,
        )
        .await?;

    println!("db write: {:#?}", time.elapsed());

    Ok(record_id)
}
