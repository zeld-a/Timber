use axum::{
    extract::State,
    middleware::Next,
    response::Response,
    http::Request,
    body::Body,
};
use tower_cookies::Cookies;

use crate::models::user::User;
use crate::state::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    cookies: Cookies,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let user = async {
        let cookie = cookies.get("session")?;
        let token = cookie.value().to_string();
        
        let session = sqlx::query!(
            "SELECT user_id FROM sessions WHERE token = ? AND expires_at > datetime('now')",
            token
        )
        .fetch_optional (&state.db)
        .await
        .ok()??;
        
        let found_user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at FROM users WHERE id = ?",
            session.user_id
        )
        .fetch_optional(&state.db)
        .await
        .ok()??;
        
        Some(found_user)
    }
    .await;
    
    request.extensions_mut().insert(user);
    next.run(request).await
}