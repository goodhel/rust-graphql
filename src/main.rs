use actix_web::{App, HttpServer, middleware::Logger};
// use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use anyhow::Result;

mod db;
mod paths;
mod handler;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    // Connect Database
    // let pool = PgPoolOptions::new()
    // .max_connections(5)
    // .connect(&env::var("DATABASE_URL")?).await?;
    let pool = db::get_db_pool().await;
    // Variable Host dan Port
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::new("%a %r %s %{User-Agent}i"))
            .configure(paths::init) // Routes
    })
    .bind(format!("{}:{}", host, port))?
    // .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}