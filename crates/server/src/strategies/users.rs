use std::env;
use bcrypt::{DEFAULT_COST, hash_with_salt};
use sqlx::any::AnyRow;
use types::user::{RegisterUser, User, UserInfo};
use uuid::Uuid;

use crate::pool;

pub async fn get_db_user_by_username_or_email(username_or_email: String) -> Result<User, sqlx::Error> {
    // query for getting all data from users table where user row matches given user ID
    sqlx::query_as::<_, User>(
        "SELECT * FROM \"users\" WHERE username = $1 OR email = $1;")
    .bind(username_or_email)
    .fetch_one(&pool::get_pool()).await
}

pub async fn get_db_user_by_uuid(uuid: String) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_,User>(
        "SELECT * FROM \"users\" WHERE uuid = $1;")
        .bind(uuid)
        .fetch_one(&pool::get_pool()).await
}

pub async fn get_all_users() -> Result<Vec<UserInfo>, sqlx::Error> {
    sqlx::query_as::<_, UserInfo>("SELECT * FROM \"users\";")
        .fetch_all(&pool::get_pool()).await
}

pub async fn delete_user_by_uuid(uuid: String) -> Result<AnyRow, sqlx::Error> {
    sqlx::query("DELETE FROM \"users\" WHERE uuid = $1;")
        .bind(uuid)
        .fetch_one(&pool::get_pool()).await
}

pub async fn insert_db_user(register_user: RegisterUser) -> Result<User, sqlx::Error> {
    // generate new user id
    let id = Uuid::new_v4();
    // initialize salt str slice
    let mut salt: [u8; 16] = [0;16];
    // load 16 bytes from PASSWORD_SALT env variable to salt str slice
    salt.copy_from_slice(&env::var("PASSWORD_SALT").unwrap().as_bytes()[0..16]);
    // perform query to insert new user with hashed password and bind all payload object fields
    sqlx::query_as::<_, User>(
        "INSERT INTO \"users\" (uuid, username, pass, email, is_admin)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;")
        .bind(id.to_string())
        .bind(register_user.username)
        // hash password with salt
        .bind(hash_with_salt(
            register_user.pass,
            DEFAULT_COST,
            salt
        ).unwrap().to_string())
        .bind(register_user.email)
        .bind(false)
        .fetch_one(&pool::get_pool()).await
}

pub async fn update_db_user(user: User) -> Result<AnyRow, sqlx::Error> {
    // initialize salt str slice
    let mut salt: [u8; 16] = [0;16];
    // load 16 bytes from PASSWORD_SALT env variable to salt str slice
    salt.copy_from_slice(&env::var("PASSWORD_SALT").unwrap().as_bytes()[0..16]);
    // perform query to insert new user with hashed password and bind all payload object fields
    sqlx::query(
        "UPDATE \"users\"
        SET uuid = $2, username = $3, pass = $4, email = $5, is_admin = $6
        WHERE id = $1
        RETURNING *;")
        .bind(user.id)
        .bind(user.uuid)
        .bind(user.username)
        // hash password with salt
        .bind(hash_with_salt(
            user.pass,
            DEFAULT_COST,
            salt
        ).unwrap().to_string())
        .bind(user.email.to_string())
        .bind(user.is_admin)
        .fetch_one(&pool::get_pool()).await
}