use std::{env, time::{SystemTime, UNIX_EPOCH}};

use axum::{
    Json, RequestPartsExt, http::{
        StatusCode, request::Parts
    }, response::{
        IntoResponse, Response
    }, extract::FromRequestParts, async_trait, body::Body
};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use types::auth::AuthToken;

// Keys for encoding/decoding authorization tokens with JWT_SECRET
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("AUTH_TOKEN_SECRET").expect("AUTH_TOKEN_SECRET must be configured.");
    Keys::new(secret.as_bytes())
});

static TOKEN_LIFETIME: Lazy<u64> = Lazy::new(|| {
    u64::from_str_radix(env::var("AUTH_TOKEN_EXPIRE")
        .expect("AUTH_TOKEN_EXPIRE must be configured").as_str(), 10)
        .expect("Cannot parse AUTH_TOKEN_EXPIRE as u64")
});

static TOKEN_REQUESTER_LIFETIME: Lazy<u64> = Lazy::new(|| {
    u64::from_str_radix(env::var("AUTH_REQUEST_TOKEN_EXPIRE")
        .expect("AUTH_REQUEST_TOKEN_EXPIRE must be configured").as_str(), 10)
        .expect("Cannot parse AUTH_REQUEST_TOKEN_EXPIRE as u64")
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
 * Generate a new token and return it as AuthToken object
 */
pub fn generate_auth_token(uuid: String) -> Result<AuthToken, AuthError> {
    let auth_claims = AuthClaims {
        sub: uuid,
        // issuer domain
        aud: env::var("COMPANY_DOMAIN").unwrap(),
        // issuer company
        com: env::var("COMPANY_NAME").unwrap(),
        iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        // expiration timestamp from unix epoch
        exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + *TOKEN_LIFETIME,
        acc: 0
    };
    match encode(&Header::default(), &auth_claims, &KEYS.encoding) {
        Ok(encoded_string) => {
            Ok(AuthToken::new(encoded_string))
        },
        Err(error) => {
            println!("Error creating token: {}", error);
            Err(AuthError::TokenCreation)
        }
    }
}


/**
 * Generate a new token and return it as AuthToken object
 */
pub fn generate_requester_token(uuid: String) -> Result<AuthToken, AuthError> {
    let auth_claims = AuthRequesterClaims {
        sub: uuid,
        // issuer domain
        aud: env::var("COMPANY_DOMAIN").unwrap(),
        // issuer company
        com: env::var("COMPANY_NAME").unwrap(),
        iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        // expiration timestamp from unix epoch
        exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + *TOKEN_LIFETIME
    };
    println!("{:?}", auth_claims);
    match encode(&Header::default(), &auth_claims, &KEYS.encoding) {
        Ok(encoded_string) => {
            Ok(AuthToken::new(encoded_string))
        },
        Err(error) => {
            println!("Error creating token: {}", error);
            Err(AuthError::TokenCreation)
        }
    }
}

pub trait Claims {
    fn validate(&self) -> Result<(), AuthError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub aud: String,
    pub com: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub acc: u64
}

impl Claims for AuthClaims {
    fn validate(&self) -> Result<(), AuthError> {
        // iat validation
        let lifetime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - self.iat;
        if lifetime > *TOKEN_LIFETIME {
            return Err(AuthError::TokenExpired)
        }
        Ok(())
    }
}

/**
 * Implement FromRequestParts trait for AuthClaims struct to allow extracting from request body
 */
#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[env::var("COMPANY_DOMAIN").unwrap()]);
        let token_data = decode::<AuthClaims>(bearer.token(), &KEYS.decoding, &validation)
        .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequesterClaims {
    pub aud: String,
    pub com: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64
}

impl Claims for AuthRequesterClaims {
    fn validate(&self) -> Result<(), AuthError> {
        // iat validation
        let lifetime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - self.iat;
        if lifetime > *TOKEN_LIFETIME {
            return Err(AuthError::TokenExpired)
        }
        Ok(())
    }
}

/**
 * Implement FromRequestParts trait for AuthRequesterClaims struct to allow extracting from request body
 */
#[async_trait]
impl<S> FromRequestParts<S> for AuthRequesterClaims
where
    S: Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|error| {
                println!("{}", error);
                AuthError::InvalidToken
            })?;
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[env::var("COMPANY_DOMAIN").unwrap()]);
        // Decode the user data
        let token_data = decode::<AuthRequesterClaims>(bearer.token(), &KEYS.decoding, &validation)
        .map_err(|error| {
            println!("{}", error);
            AuthError::InvalidToken
        })?;
        Ok(token_data.claims)
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