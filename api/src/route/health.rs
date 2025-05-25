use axum::Router;
use registry::AppRegistry;

pub fn build_health_check_routes() -> Router<AppRegistry> {
    use crate::handler::health::{health_check, health_check_with_db};
    use axum::{Router, routing::get};

    Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_with_db))
}
