use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::{book::build_book_routers, health::build_health_check_routes};
use axum::Router;
use registry::AppRegistry;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    let pool = connect_database_with().await;
    let registry = AppRegistry::new(pool);
    let app = Router::new()
        .merge(build_health_check_routes())
        .merge(build_book_routers())
        .with_state(registry);
    let addr = SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 3000);
    let listener = TcpListener::bind(addr).await?;
    println!("Listengin on {}", addr);
    axum::serve(listener, app).await.map_err(Error::from)
}
