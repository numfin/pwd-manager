use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppErr {
    Internal(eyre::Error),
}
pub type AppResult<T> = Result<T, AppErr>;

impl IntoResponse for AppErr {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            Self::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };
        let body = Json(json!({ "error": msg }));
        (code, body).into_response()
    }
}

impl From<eyre::Error> for AppErr {
    fn from(err: eyre::Error) -> Self {
        Self::Internal(err)
    }
}
impl From<meilisearch_sdk::errors::Error> for AppErr {
    fn from(err: meilisearch_sdk::errors::Error) -> Self {
        Self::Internal(eyre::Error::from(err))
    }
}
