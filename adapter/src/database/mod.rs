use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with() -> ConnectionPool {
    dotenv::dotenv().ok();
    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");
    let connect_options =
        PgConnectOptions::from_str(&database_url).expect("Invalid DATABASE_URL format");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy_with(connect_options);
    ConnectionPool(pool)
}
