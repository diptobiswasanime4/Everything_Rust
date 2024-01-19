use axum::{
    response::Html,
    routing::{get, post},
    http::StatusCode,
    Json, Router
}
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener()::bind("127.0.0.1:8000")
    .await
    .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap);

    axum::serve(listener, app).await.unwrap();



}

async fn handler () -> HTML<&'static str> {
    Html("<h1>Todo List Axum</h1>")
}