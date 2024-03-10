use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, time::Duration};
use once_cell::sync::OnceCell;

// global POOL singleton
static POOL: OnceCell<PgPool> = OnceCell::new();

// function for initializing the POOL singleton
pub async fn create_pool() {

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(error) => {
            panic!("Error getting DATABASE_URL: {}", error);
        }
    };

    let pool = match PgPoolOptions::new()
        .max_connections(100)
        .idle_timeout(Some(Duration::from_millis(1000)))
        .connect(&database_url).await {
            Ok(pool) => pool,
            Err(error) => {
                panic!("Could not create pool: {}", error);
            }
        };
    POOL.set(pool).unwrap();
}

// getter for accessing global POOL singleton in other modules
pub fn get_pool() -> PgPool {
    POOL.get().unwrap().to_owned()
}