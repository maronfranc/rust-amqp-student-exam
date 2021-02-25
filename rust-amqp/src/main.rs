use amiquip::Connection;
use include_dir::{include_dir, Dir};
use sqlx::postgres::PgPoolOptions;
use sqlx_pg_migrate::migrate;
use std::env;

mod dtos;
mod pattern_queue;
mod patterns;

const URL: &str = "amqp://guest:guest@localhost:5672";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").unwrap();
    const MIGRATIONS: Dir = include_dir!("migrations");
    migrate(&db_url, &MIGRATIONS).await.unwrap();
    let mut pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Unable to connect to database");
    let mut connection = match Connection::insecure_open(URL) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };

    pattern_queue::pattern_queue(&mut connection, &mut pool);

    connection.close().unwrap()
}
