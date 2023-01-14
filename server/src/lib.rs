use std::sync::Arc;

use axum::Router;
use meili::Meili;

pub mod err;
pub mod meili;
pub mod password;

pub struct AppState {
    pub meili: Meili,
}
pub async fn run() -> eyre::Result<()> {
    let meili = Meili::default();
    meili.prepare().await?;
    let shared_state = Arc::new(AppState { meili });

    let router = Router::new()
        .merge(password::router())
        .with_state(shared_state);

    println!("Working on http:://0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
