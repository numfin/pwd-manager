use std::sync::Arc;

use axum::extract::{Path, State};

use crate::{err::AppResult, meili::AppIndex, AppState};

pub async fn route(
    State(state): State<Arc<AppState>>,
    Path(document_id): Path<String>,
) -> AppResult<&'static str> {
    let index = state.meili.index(AppIndex::PasswordRecords);
    let _ = index
        .delete_document(&document_id)
        .await?
        .wait_for_completion(&state.meili.client, None, None)
        .await?;

    Ok("OK")
}
