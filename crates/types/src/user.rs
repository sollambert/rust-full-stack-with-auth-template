use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlx")]
use sqlx::{any::AnyRow, FromRow, Row};

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub pass: String,
    pub email: String,
    pub perms: i32
}

#[cfg(feature = "sqlx")]
impl <'r> FromRow<'r, AnyRow> for User {
    fn from_row(row: &AnyRow) ->  Result<Self, sqlx::Error> {
        let id: i32 = row.try_get("id")?;
        let uuid: String = row.try_get("uuid")?;
        let username: String = row.try_get("username")?;
        let pass: String = row.try_get("pass")?;
        let email: String = row.try_get("email")?;
        let perms: i32 = row.try_get("perms")?;

        Ok(Self {
            id, uuid, username, pass, email, perms
        })
    }
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

impl RegisterUser {
    pub fn assign_by_name(&self, key: &str, value: String) -> Result<Self,String> {
        let mut new_self = self.clone();
        match key {
            "username" => new_self.username = value,
            "pass" => new_self.pass = value,
            "email" => new_self.email = value,
            _ => return Err(format!("Key not found: {}", key))
        }
        Ok(new_self)
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

impl LoginUser {
    pub fn assign_by_name(&self, key: &str, value: String) -> Result<Self,String> {
        let mut new_self = self.clone();
        match key {
            "username" => new_self.username = value,
            "pass" => new_self.pass = value,
            _ => return Err(format!("Key not found: {}", key))
        }
        Ok(new_self)
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