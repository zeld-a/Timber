use axum::{routing::get, Router};
use crate::state::AppState;
use crate::handlers::index;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .with_state(state)
}