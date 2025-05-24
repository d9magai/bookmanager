use async_trait::async_trait;

#[async_trait]
pub trait HealthCheckRepository: Send + Sync {
    async fn db_check(&self) -> bool;
}
