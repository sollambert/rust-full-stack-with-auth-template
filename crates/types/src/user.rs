use std::fmt;

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
pub struct RegisterUser {
    pub username: String,
    pub pass: String,
    pub email: String
}

impl fmt::Display for RegisterUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Username: {}\nPass: {}\nEmail: {}", self.username, self.pass, self.email)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LoginUser {
    pub username: String,
    pub pass: String
}

impl fmt::Display for LoginUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Username: {}\nPass: {}", self.username, self.pass)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct UserInfo {
    pub uuid: String,
    pub username: String,
    pub email: String
}

impl fmt::Display for UserInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID: {}\nUsername: {}\nEmail: {}", self.uuid, self.username, self.email)
    }
}