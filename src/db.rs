use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub type PgPool = Pool<Postgres>;

pub async fn get_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url).await.expect("Failed to create DB Pool")
}