use axum::{routing::get, Router};

use crate::handlers::index;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index::index))
}