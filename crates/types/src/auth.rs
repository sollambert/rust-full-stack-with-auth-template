use serde::{Deserialize, Serialize};

use axum::{Json, http::StatusCode, response::{IntoResponse, Response}, body::Body};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String
}

impl AuthToken {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
    pub fn from_string(string: String) -> Self {
        let split: Vec<&str> = string.split(" ").collect();
        AuthToken {
            token_type: split[0].to_string(),
            access_token: split[1].to_string()
        }
    }
    pub fn to_string(self: AuthToken) -> String {
        let mut string = self.token_type;
        string.push_str((" ".to_owned() + self.access_token.as_str()).as_str());
        string
    }
    pub fn default() -> Self {
        Self {
            access_token: String::new(),
            token_type: String::new()
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    TokenCreation,
    InvalidToken
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}