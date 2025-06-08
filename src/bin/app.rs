use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::{book::build_book_routers, health::build_health_check_routes};
use axum::Router;
use registry::AppRegistry;
use shared::env::{Environment, which};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subsciber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subsciber)
        .with(env_filter)
        .try_init()?;
    Ok(())
}

use anyhow::Context;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};

async fn bootstrap() -> Result<()> {
    let pool = connect_database_with().await;
    let registry = AppRegistry::new(pool);
    let app = Router::new()
        .merge(build_health_check_routes())
        .merge(build_book_routers())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        .with_state(registry);

    let addr = SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 3000);
    let listener = TcpListener::bind(addr).await?;
    println!("Listengin on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happend in server")
        .inspect_err(|e| {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "Unexpected error in server");
        })
}
