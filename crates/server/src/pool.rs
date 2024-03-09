use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, time::Duration};
use once_cell::sync::OnceCell;

// global POOL singleton
static POOL: OnceCell<PgPool> = OnceCell::new();

// function for initializing the POOL singleton
pub async fn create_pool() -> Result<(), sqlx::Error> {
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(100)
        .idle_timeout(Some(Duration::from_millis(1000)))
        .connect(&env::var("DATABASE_URL").unwrap()).await?;
    POOL.set(pool).unwrap();
    Ok(())
}

// getter for accessing global POOL singleton in other modules
pub fn get_pool() -> PgPool {
    POOL.get().unwrap().to_owned()
}