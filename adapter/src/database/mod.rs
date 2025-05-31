use sea_orm::Database;
pub mod model;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct ConnectionPool(DatabaseConnection);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &DatabaseConnection {
        &self.0
    }
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

pub async fn connect_database_with() -> ConnectionPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    ConnectionPool(db)
}
