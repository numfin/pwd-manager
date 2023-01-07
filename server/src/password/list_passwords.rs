use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use essentials::password::PasswordRecord;
use serde::{Deserialize, Serialize};

use crate::{err::AppResult, meili::AppIndex, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRecord {
    pub id: String,
    pub title: String,
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPasswordsPayload {
    pub search: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListPasswordsResponse {
    pub records: Vec<PublicRecord>,
}

pub async fn route(
    State(state): State<Arc<AppState>>,
    Query(payload): Query<ListPasswordsPayload>,
) -> AppResult<Json<ListPasswordsResponse>> {
    let index = state.meili.index(AppIndex::PasswordRecords);
    let records = index
        .search()
        .with_query(&payload.search)
        .with_sort(&["title:asc"])
        .with_limit(10)
        .execute::<PasswordRecord>()
        .await
        .map_err(|err| {
            println!("{err:?}");
            err
        })?
        .hits;

    let records = records
        .iter()
        .map(|record| PublicRecord {
            id: record.result.id.clone(),
            title: record.result.title.clone(),
            login: record.result.login.clone(),
        })
        .collect();

    let body = ListPasswordsResponse { records };
    Ok(Json(body))
}
