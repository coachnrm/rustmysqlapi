use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use dotenv::dotenv;
use std::env;

pub async fn get_db_pool() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
