use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::config::CONFIG;

pub type DbPool = PgPool;

pub async fn init_pool() -> DbPool {
    PgPoolOptions::new()
        .max_connections(4)
        .min_connections(2)
        .connect(&CONFIG.db_url)
        .await
        .expect("Failed to create Postgres pool")
}
