use axum::response::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate;

pub async fn get_register() -> Html<String> {
    let template = RegisterTemplate;
    Html(template.render().unwrap())
}

pub async fn get_login() -> Html<String> {
    let template = LoginTemplate;
    Html(template.render().unwrap())
}