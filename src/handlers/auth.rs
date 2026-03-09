use axum::response::{Html, IntoResponse, Redirect};
use askama::Template;
use argon2::{Argon2, PasswordHasher};
use argon2:password_hash::{SaltString, rand_core:: OsRng};
use axum::extract::State;
use axum::Form;
use crate::state::AppState;

#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate {
    error: Option<String>

}

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate;

#[derive(serde::Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

pub async fn get_register() -> Html<String> {
    let template = RegisterTemplate { error: None }
    Html(template.render().unwrap())
}

pub async fn get_login() -> Html<String> {
    let template = LoginTemplate;
    Html(template.render().unwrap())
}

pub async fn post_register(
    State(state): State<AppState>,
        Form(form): Form<RegisterForm>,
) -> impl IntoResponce {
    // Check if username or email is already in use
    let existing = sqlx::query!(
        "SELECTE id FROM users WHERE username = ? OR email = ?",
        form.username,
        form.email
    )
    .fetch_optional(&state.db)
    .await
    .unwrap();
    
    if existing.is_some() {
        let template = RegisterTemplate {
            error: Some("Username or email already taken".into()),
        }
        return Html(template.render().unwrap()).into_responce()
    }
}