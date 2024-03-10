use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub pass: String,
    pub email: String
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateUser {
    pub username: String,
    pub pass: String,
    pub email: String
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LoginUser {
    pub username: String,
    pub pass: String
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct ResponseUser {
    pub uuid: String,
    pub username: String,
    pub email: String
}