use std::{env, time::{SystemTime, UNIX_EPOCH}};

use axum::{
    Json, RequestPartsExt, http::{
        StatusCode, request::Parts
    }, response::{
        IntoResponse, Response
    }, extract::FromRequestParts, async_trait, body::Body
};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};
use jsonwebtoken::{EncodingKey, DecodingKey, Validation, decode, encode, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use types::auth::AuthToken;

// Keys for encoding/decoding authorization tokens with JWT_SECRET
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be configured.");
    Keys::new(secret.as_bytes())
});

static TOKEN_LIFETIME: Lazy<u64> = Lazy::new(|| {
    u64::from_str_radix(env::var("JWT_EXPIRE").expect("JWT_EXPIRE must be configured").as_str(), 10).expect("Cannot parse JWT_EXPIRE as u64")
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret)
        }
    }
}

/**
 * Implement FromRequestParts trait for Claims struct to allow extracting from request body
 */
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::TokenExpired => (StatusCode::BAD_REQUEST, "Token is expired"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

/**
 * Generate a new token and return it as AuthToken object
 */
pub fn generate_new_token(uuid: String) -> Result<AuthToken, AuthError> {
    let claims = Claims {
        uuid,
        // issuer domain
        sub: env::var("JWT_SUB").unwrap(),
        // issuer company
        com: env::var("JWT_COMPANY").unwrap(),
        iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        // expiration timestamp from unix epoch
        exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + *TOKEN_LIFETIME
    };
    match encode(&Header::default(), &claims, &KEYS.encoding) {
        Ok(encoded_string) => {
            Ok(AuthToken::new(encoded_string))
        },
        Err(error) => {
            println!("Error creating token: {}", error);
            Err(AuthError::TokenCreation)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    uuid: String,
    sub: String,
    com: String,
    iat: u64,
    exp: u64
}

impl Claims {
    pub fn validate(&self) -> Result<(), AuthError> {
        // iat validation
        let lifetime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - self.iat;
        if lifetime > *TOKEN_LIFETIME {
            return Err(AuthError::TokenExpired)
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    TokenExpired,
    InvalidToken
}