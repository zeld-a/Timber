use tower_http::services::ServeDir;

mod db;
mod routes;
mod handlers;
mod state;
mod middleware;
mod models;

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    let state = state::AppState { db: pool };
    
    let app = routes::create_router(state)
        .nest_service("/static", ServeDir::new("static"));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running at http://localhost:3000/");
    
    axum::serve(listener, app).await.unwrap();
}