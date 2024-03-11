use std::env;

use std::time::Duration;
use once_cell::sync::OnceCell;

#[cfg(not(any(feature = "postgres", feature = "sqlite")))]
use sqlx::{any::{Any, AnyPoolOptions}, Pool};

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
use sqlx::{PgPool, postgres::PgPoolOptions};

#[cfg(all(feature = "sqlite", not(feature = "postgres")))]
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

#[cfg(not(any(feature = "postgres", feature = "sqlite")))]
// global POOL singleton
static POOL: OnceCell<Pool<Any>> = OnceCell::new();

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
// global POOL singleton
static POOL: OnceCell<PgPool> = OnceCell::new();
#[cfg(all(feature = "sqlite", not(feature = "postgres")))]
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

#[cfg(not(any(feature = "postgres", feature = "sqlite")))]
async fn init_pool(database_url: String) {
    let pool = match AnyPoolOptions::new()
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

pub fn get_pool() -> Pool<Any> {
    POOL.get().unwrap().to_owned()
}

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
// getter for accessing global POOL singleton in other modules
pub fn get_pool() -> PgPool {
    POOL.get().unwrap().to_owned()
}

#[cfg(all(feature = "sqlite", not(feature = "postgres")))]
// getter for accessing global POOL singleton in other modules
pub fn get_pool() -> SqlitePool {
    POOL.get().unwrap().to_owned()
}