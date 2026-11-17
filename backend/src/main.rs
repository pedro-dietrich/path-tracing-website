use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Serialize)]
struct Message {
    content: &'static str
}

async fn test() -> Json<Message> {
    Json(Message { content: "API test message." })
}

async fn api_not_found() -> (StatusCode, Json<Message>) {
    (
        StatusCode::NOT_FOUND,
        Json(Message { content: "API endpoint not found." })
    )
}

#[tokio::main]
async fn main() {
    let distribution_directory = ServeDir::new("frontend/dist");
    let index_file = ServeFile::new("frontend/dist/index.html");

    let api = Router::new()
        .route("/test", get(test))
        .fallback(api_not_found);

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(distribution_directory.fallback(index_file));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
