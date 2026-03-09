use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use rand_core::OsRng;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate {
    error: Option<String>
}

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate {
    error: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn get_register() -> Html<String> {
    let template = RegisterTemplate { error: None };
    Html(template.render().unwrap())
}

pub async fn get_login() -> Html<String> {
    let template = LoginTemplate { error: None };
    Html(template.render().unwrap())
}

pub async fn post_register(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    // Check if username or email is already in use
    let existing = sqlx::query!(
        "SELECT id FROM users WHERE username = ? OR email = ?",
        form.username,
        form.email
    )
    .fetch_optional(&state.db)
    .await
    .unwrap();
    
    if existing.is_some() {
        let template = RegisterTemplate {
            error: Some("Username or email already taken".into()),
        };
        return Html(template.render().unwrap()).into_response()
    }
    
    // Password Hashing
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    
    // INSERT new user
    sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
        form.username,
        form.email,
        password_hash
    )
    .execute(&state.db)
    .await
    .unwrap();
    
    Redirect::to("/login").into_response()
}

pub async fn post_login(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = ?",
        form.username
    )
    .fetch_optional(&state.db)
    .await
    .unwrap();
    
    let user = match user {
        Some(u) => u,
        None => {
            let template = LoginTemplate {
                error: Some("Invalid username or password".into()),
            };
            return Html(template.render().unwrap()).into_response();
        }
    };
    
    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        let template = LoginTemplate {
            error: Some("Invalid username or password".into()),
        };
        return Html(template.render().unwrap()).into_response();
    }
    let token = Uuid::new_v4().to_string();
    
    sqlx::query!(
        "INSERT INTO sessions (user_id, token, expires_at) VALUES (?, ?, datetime('now', '+7 days'))",
        user.id,
        token
    )
    .execute(&state.db)
    .await
    .unwrap();
    
    let mut cookie = Cookie::new("session", token);
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);
    
    Redirect::to("/").into_response()
}

pub async fn post_logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    if let Some(cookie) = cookies.get("session") {
        let token = cookie.value().to_string();
        
        sqlx::query!("DELETE FROM sessions WHERE token = ?", token)
            .execute(&state.db)
            .await
            .unwrap();
        
        cookies.remove(Cookie::new("session", ""));
    }
    
    Redirect::to("/login").into_response()
}