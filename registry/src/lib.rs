use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn kernel::repository::book::BookRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repository: Arc<HealthCheckRepositoryImpl> =
            Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository: Arc<dyn kernel::repository::book::BookRepository> =
            Arc::new(adapter::repository::book::BookRepositoryImpl::new(pool));
        Self {
            health_check_repository,
            book_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<dyn kernel::repository::book::BookRepository> {
        self.book_repository.clone()
    }
}
