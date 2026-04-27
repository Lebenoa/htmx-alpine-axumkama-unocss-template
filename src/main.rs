mod routes;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/dashboard", get(routes::dashboard))
        .nest_service("/public", ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7348")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
