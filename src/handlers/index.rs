use axum::response::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    message: String,
}

pub async fn index() -> Html<String> {
    let template = IndexTemplate {
        message: "Welcome to your Git hosting platform".into(),
    };
    
    Html(template.render().unwrap())
}