use std::sync::Arc;

use axum::{routing::*, Router};

use crate::AppState;

pub mod delete_password;
pub mod edit_password;
pub mod get_password;
pub mod list_passwords;
pub mod new_password;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/password", get(list_passwords::route))
        .route("/password", post(new_password::route))
        .route("/password/:document_id", post(get_password::route))
        .route("/password/:document_id", delete(delete_password::route))
        .route("/password/:document_id", put(edit_password::route))
}
