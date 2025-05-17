use axum::{Router, response::Json, routing::get, serve};
use serde_json::{Value, json};
use tokio::net::TcpListener;

// plain text response
async fn plain_text() -> &'static str {
    "foo"
}

// JSON response
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
