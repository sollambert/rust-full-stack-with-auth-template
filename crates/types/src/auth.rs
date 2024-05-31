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
                message: String::from("Generic auth error")
            }
        }
    }
    pub fn from_error_type(error_type: AuthErrorType) -> Self {
        let (status, message) = match error_type {
            AuthErrorType::WrongCredentials => (StatusCode::UNAUTHORIZED, String::from("Wrong credentials")),
            AuthErrorType::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Token creation error")),
            AuthErrorType::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Server error")),
            AuthErrorType::UserAlreadyExists => (StatusCode::CONFLICT, String::from("Username or email taken")),
            AuthErrorType::UserDoesNotExist => (StatusCode::NOT_FOUND, String::from("User does not exist")),
            AuthErrorType::InvalidToken => (StatusCode::FORBIDDEN, String::from("Invalid token")),
            AuthErrorType::AccessDenied => (StatusCode::FORBIDDEN, String::from("Access denied")),
            AuthErrorType::MissingFields => (StatusCode::BAD_REQUEST, String::from("Missing required fields")),
            AuthErrorType::BadRequest => (StatusCode::BAD_REQUEST, String::from("Bad request")),
            AuthErrorType::InvalidEmail => (StatusCode::BAD_REQUEST, String::from("Email address is invalid"))
        };
        Self {
            status,
            body: AuthErrorBody {
                error_type,
                message
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
    MissingFields,
    InvalidEmail
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthErrorBody {
    pub error_type: AuthErrorType,
    pub message: String
}