use axum::{routing::get, Router};
use crate::state::AppState;
use crate::handlers::{index, auth};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/register", get(auth::get_register))
        .route("/login", get(auth::get_login))
        .with_state(state)
}