use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
