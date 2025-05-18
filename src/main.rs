use anyhow::Result;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/health", get(health_check));

    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 3000)).await?;

    Ok(axum::serve(listener, app).await?)
}
