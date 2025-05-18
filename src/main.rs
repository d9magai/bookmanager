use anyhow::Result;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use sqlx::PgPool;
use tokio::net::TcpListener;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_with_db(pool: PgPool) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn create_connection_pool() -> Result<PgPool> {
    dotenv::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = create_connection_pool().await?;
    let app = Router::new().route("/health", get(health_check)).route(
        "/health/db",
        get({
            let pool = pool.clone();
            move || health_check_with_db(pool.clone())
        }),
    );

    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 3000)).await?;

    Ok(axum::serve(listener, app).await?)
}

#[tokio::test]
async fn test_health_check() {
    let response = health_check().await;
    assert_eq!(response, StatusCode::OK);
}
