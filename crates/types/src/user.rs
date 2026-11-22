use std::{fmt, str::FromStr};

use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlx")]
use sqlx::{any::AnyRow, FromRow, Row};

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub pass: String,
    pub email: EmailAddress,
    pub is_admin: bool
}

/// Decode a boolean column through the `Any` driver. Postgres yields a native bool, while
/// SQLite stores booleans as INTEGER 0/1 (the `Any` driver cannot map SQLite's BOOLEAN type).
#[cfg(feature = "sqlx")]
fn try_get_bool(row: &AnyRow, column: &str) -> Result<bool, sqlx::Error> {
    match row.try_get::<bool, _>(column) {
        Ok(value) => Ok(value),
        Err(bool_error) => row
            .try_get::<i64, _>(column)
            .map(|value| value != 0)
            .map_err(|_| bool_error),
    }
}

#[cfg(feature = "sqlx")]
impl <'r> FromRow<'r, AnyRow> for User {
    fn from_row(row: &AnyRow) ->  Result<Self, sqlx::Error> {
        let id: i32 = row.try_get("id")?;
        let uuid: String = row.try_get("uuid")?;
        let username: String = row.try_get("username")?;
        let pass: String = row.try_get("pass")?;
        let email: EmailAddress = match row.try_get::<String, &str>("email") {
            Ok(email_str) => {
                EmailAddress::new_unchecked(email_str)
            },
            Err(e) => {
                println!("{}", e);
                EmailAddress::new_unchecked("")
            }
        };
        let is_admin: bool = try_get_bool(row, "is_admin")?;

        Ok(Self {
            id, uuid, username, pass, email, is_admin
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
    pub fn update_field(&self, key: &str, value: String) -> Result<Self,String> {
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
    pub fn update_field(&self, key: &str, value: String) -> Result<Self,String> {
        let mut new_self = self.clone();
        match key {
            "username" => new_self.username = value,
            "pass" => new_self.pass = value,
            _ => return Err(format!("Key not found: {}", key))
        }
        Ok(new_self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResetUser {
    pub email_address: EmailAddress,
    pub pass: String
}

impl ResetUser {
    pub fn new(&self) -> ResetUser {
        Self {
            email_address: EmailAddress::new_unchecked(""),
            pass: String::new()
        }
    }
    pub fn update_field(&self, key: &str, value: String) -> Result<Self,String> {
        let mut new_self = self.clone();
        match key {
            "pass" => new_self.pass = value,
            "email" => new_self.email_address = EmailAddress::from_str(&value).unwrap(),
            _ => return Err(format!("Key not found: {}", key))
        }
        Ok(new_self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct UserInfo {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool
}

#[cfg(feature = "sqlx")]
impl <'r> FromRow<'r, AnyRow> for UserInfo {
    fn from_row(row: &AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            uuid: row.try_get("uuid")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            is_admin: try_get_bool(row, "is_admin")?,
        })
    }
}

impl fmt::Display for UserInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID: {}\nUsername: {}\nEmail: {}\nIs Admin: {}", self.uuid, self.username, self.email, self.is_admin)
    }
}

impl UserInfo {
    pub fn from_user(user: User) -> Self {
        Self {
            uuid: user.uuid,
            username: user.username,
            email: user.email.to_string(),
            is_admin: user.is_admin
        }
    }
    pub fn new() -> Self {
        Self {
            uuid: String::new(),
            username: String::new(),
            email: String::new(),
            is_admin: false
        }
    }
}