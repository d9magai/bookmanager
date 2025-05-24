use crate::database::ConnectionPool;
use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::HealthCheckRepository;

#[derive(new)]
pub struct HealthCheckRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn db_check(&self) -> bool {
        sqlx::query("SELECT 1")
            .execute(self.db.inner_ref())
            .await
            .is_ok()
    }
}
