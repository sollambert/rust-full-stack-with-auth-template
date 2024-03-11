use sqlx::{Pool, SqlitePool, sqlite::SqlitePoolOptions, PgPool, postgres::PgPoolOptions};
use std::{env, time::Duration};
use once_cell::sync::OnceCell;

#[cfg(feature = "postgres")]
// global POOL singleton
static POOL: OnceCell<PgPool> = OnceCell::new();
#[cfg(feature = "sqlite")]
// global POOL singleton
static POOL: OnceCell<SqlitePool> = OnceCell::new();

// function for initializing the POOL singleton
pub async fn create_pool() {

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(error) => {
            panic!("Error getting DATABASE_URL: {}", error);
        }
    };
    init_pool(database_url).await;
}

#[cfg(not(feature = "postgres"))]
#[cfg(not(feature = "sqlite"))]
async fn init_pool(_database_url: String) {
    panic!("No db features enabled!");
}

#[cfg(feature = "postgres")]
async fn init_pool(database_url: String) {
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

#[cfg(feature = "sqlite")]
async fn init_pool(database_url: String) {
    let pool = match SqlitePoolOptions::new()
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

#[cfg(not(feature = "postgres"))]
#[cfg(not(feature = "sqlite"))]
pub fn get_pool() -> PgPool {
    panic!("No db features enabled!");
}
#[cfg(feature = "postgres")]
// getter for accessing global POOL singleton in other modules
pub fn get_pool() -> PgPool {
    POOL.get().unwrap().to_owned()
}
#[cfg(feature = "sqlite")]
// getter for accessing global POOL singleton in other modules
pub fn get_pool() -> SqlitePool {
    POOL.get().unwrap().to_owned()
}