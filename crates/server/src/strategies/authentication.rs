use std::{env, time::{SystemTime, UNIX_EPOCH}};

use axum::{RequestPartsExt, http::request::Parts, extract::FromRequestParts, async_trait};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use types::auth::{AuthError, AuthToken};

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

pub trait Claims {
    fn new(uuid: String) -> Self;
    // fn validate(&self) -> Result<(), AuthError>;
    fn generate_token(&self) -> Result<AuthToken, AuthError>
    where Self: Serialize {
        match encode(&Header::default(), &self, &KEYS.encoding) {
            Ok(encoded_string) => {
                Ok(AuthToken::new(encoded_string))
            },
            Err(error) => {
                println!("Error creating token: {}", error);
                Err(AuthError::TokenCreation)
            }
        }
    }
}

async fn claims_from_request<T>(parts: &mut Parts) -> Result<T, AuthError>
where
    T: for<'de> Deserialize<'de>,
{
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthError::InvalidToken)?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[env::var("COMPANY_DOMAIN").unwrap()]);
    validation.set_issuer(&[env::var("COMPANY_NAME").unwrap()]);
    // Decode the user data
    let token_data = decode::<T>(bearer.token(), &KEYS.decoding, &validation)
    .map_err(|_| AuthError::InvalidToken)?;
    Ok(token_data.claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub aud: String,
    pub com: String,
    pub sub: String,
    // pub iat: u64,
    pub exp: u64,
    pub acc: u64
}

impl Claims for AuthClaims {
    fn new(uuid: String) -> AuthClaims {
        Self {
            // user uuid
            sub: uuid,
            // issuer domain
            aud: env::var("COMPANY_DOMAIN").unwrap(),
            // issuer company
            com: env::var("COMPANY_NAME").unwrap(),
            // iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            // expiration timestamp from unix epoch
            exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + *TOKEN_LIFETIME,
            acc: 0
        }
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
        claims_from_request::<AuthClaims>(parts).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequesterClaims {
    pub aud: String,
    pub com: String,
    pub sub: String,
    // pub iat: u64,
    pub exp: u64
}

impl Claims for AuthRequesterClaims {
    fn new(uuid: String) -> AuthRequesterClaims {
        Self {
            // user uuid
            sub: uuid,
            // issuer domain
            aud: env::var("COMPANY_DOMAIN").unwrap(),
            // issuer company
            com: env::var("COMPANY_NAME").unwrap(),
            // expiration timestamp from unix epoch
            exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + *TOKEN_REQUESTER_LIFETIME
        }
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
        claims_from_request::<AuthRequesterClaims>(parts).await
    }
}