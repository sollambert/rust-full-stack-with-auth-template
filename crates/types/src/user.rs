use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub pass: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub pass: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub pass: String
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ResponseUser {
    pub uuid: String,
    pub username: String,
    pub email: String
}