use include_dir::{include_dir, Dir};
use sqlx_pg_migrate::migrate;
use std::env::var;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url: String = var("DATABASE_URL").expect("DATABASE_URL is not set");
    const MIGRATIONS: Dir = include_dir!("migrations");
    migrate(&db_url, &MIGRATIONS).await.unwrap();
}
