use std::env;
use bcrypt::{DEFAULT_COST, hash_with_salt};
use types::user::{CreateUser, User};
use uuid::Uuid;

use crate::pool;

pub async fn get_db_user_by_id(id: i64) -> Result<User, sqlx::Error> {
    // query for getting all data from users table where user row matches given user ID
    let rows = sqlx::query_as::<_, User>(
        "SELECT * FROM \"users\" WHERE id = $1")
    .bind(id)
    .fetch_all(&pool::get_pool()).await.unwrap();
    // if row[0] exists return the User otherwise return RowNotFound error
    if rows.len() > 0 {
        Ok(rows[0].to_owned())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

pub async fn get_db_user_by_username(username: String) -> Result<User, sqlx::Error> {
    // query for getting all data from users table where user row matches given user ID
    let rows = sqlx::query_as::<_, User>(
        "SELECT * FROM \"users\" WHERE username = $1")
    .bind(username)
    .fetch_all(&pool::get_pool()).await.unwrap();
    // if row[0] exists return the User otherwise return RowNotFound error
    if rows.len() > 0 {
        Ok(rows[0].to_owned())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

pub async fn insert_db_user(create_user: CreateUser) -> Result<i64, sqlx::Error> {
    // generate new user id
    let id = Uuid::new_v4();
    // initialize salt str slice
    let mut salt: [u8; 16] = [0;16];
    // load 16 bytes from PASSWORD_SALT env variable to salt str slice
    salt.copy_from_slice(&env::var("PASSWORD_SALT").unwrap().as_bytes()[0..16]);
    // perform query to insert new user with hashed password and bind all payload object fields
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO \"users\" (uuid, username, pass, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id;")
        .bind(id.to_string())
        .bind(create_user.username)
        // hash password with salt
        .bind(hash_with_salt(
            create_user.pass,
            DEFAULT_COST,
            salt
        ).unwrap().to_string())
        .bind(create_user.email)
        .fetch_one(&pool::get_pool()).await?;
    // return id that was returned by sql query
    Ok(row.0)
}