use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct AuthError {
    pub status: StatusCode,
    pub body: AuthErrorBody
}

impl AuthError {
    pub fn default() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: AuthErrorBody {
                error_type: AuthErrorType::ServerError,
                message: String::from("Auth service unavailable.")
            }
        }
    }
    pub fn body(&self) -> AuthErrorBody {
        self.body.to_owned()
    }
    pub fn status(&self) -> StatusCode {
        self.status.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthErrorType {
    WrongCredentials,
    TokenCreation,
    UserAlreadyExists,
    UserDoesNotExist,
    InvalidToken,
    BadRequest,
    ServerError,
    AccessDenied,
    MissingFields
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthErrorBody {
    pub error_type: AuthErrorType,
    pub message: String
}