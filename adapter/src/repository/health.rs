use crate::database::ConnectionPool;
use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::HealthCheckRepository;
use sea_orm::{ConnectionTrait, Statement};

#[derive(new)]
pub struct HealthCheckRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn db_check(&self) -> bool {
        self.db
            .inner_ref()
            .execute(Statement::from_string(
                self.db.inner_ref().get_database_backend(),
                "SELECT 1".to_string(),
            ))
            .await
            .is_ok()
    }
}
