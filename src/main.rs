use tower_http::services::ServeDir;

mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let app = routes::create_router()
        .nest_service("/static", ServeDir::new("static"));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running at http://localhost:3000/");
    
    axum::serve(listener, app).await.unwrap();
}