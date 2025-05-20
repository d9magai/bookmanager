use anyhow::Result;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use axum::{extract::State, response::IntoResponse};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

async fn health_check() -> StatusCode {
    StatusCode::OK
}
async fn health_check_with_db(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&*pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn create_connection_pool() -> Result<PgPool> {
    dotenv::dotenv().ok();
    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");
    let pool: sqlx::Pool<sqlx::Postgres> = PgPool::connect(&database_url).await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = Arc::new(create_connection_pool().await?);
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_with_db))
        .with_state(pool);

    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 3000)).await?;

    Ok(axum::serve(listener, app).await?)
}

#[tokio::test]
async fn test_health_check() {
    let response: StatusCode = health_check().await;
    assert_eq!(response, StatusCode::OK);
}

#[sqlx::test]
async fn test_health_check_with_db(pool: PgPool) {
    let pool = Arc::new(pool);
    let response = health_check_with_db(State(pool)).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
}
