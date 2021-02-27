use amiquip::Connection;
use include_dir::{include_dir, Dir};
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx_pg_migrate::migrate;
use std::env;

mod dtos;
mod models;
mod pattern_queue;
mod patterns;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let amqp_url: String = env::var("AMQP_URL").expect("AMQP_URL is not set");
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    const MIGRATIONS: Dir = include_dir!("migrations");
    migrate(&db_url, &MIGRATIONS).await.unwrap();
    let mut pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Unable to connect to database");
    let mut connection = match Connection::insecure_open(&amqp_url) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };

    pattern_queue::pattern_queue(&mut connection, &mut pool).await;

    connection.close().unwrap()
}
