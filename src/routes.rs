use axum::{routing::{get, post}, Router};
use axum::middleware as axum_middleware;
use crate::state::AppState;
use crate::handlers::{index, auth};
use tower_cookies::CookieManagerLayer;
use crate::middleware::auth_middleware;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/register", get(auth::get_register).post(auth::post_register))
        .route("/login", get(auth::get_login).post(auth::post_login))
        .route("/logout", post(auth::post_logout))
        .with_state(state.clone())
        .layer(axum_middleware::from_fn_with_state(state, auth_middleware))
        .layer(CookieManagerLayer::new())
}