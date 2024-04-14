use std::env;
use axum::{async_trait, body::Body, extract::FromRequestParts, http::request::Parts, response::{IntoResponse, Response}, Json, RequestPartsExt};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};
use http::{HeaderMap, StatusCode};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use types::auth::{AuthErrorBody, AuthErrorType, AuthToken};
use struct_iterable::Iterable;
use base64::prelude::*;

use super::users::get_db_user_by_uuid;

// Keys for encoding/decoding authorization tokens with JWT_SECRET
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("AUTH_TOKEN_SECRET").expect("AUTH_TOKEN_SECRET must be configured.");
    Keys::new(secret.as_bytes())
});

// Auth token lifetime
static TOKEN_LIFETIME: Lazy<u64> = Lazy::new(|| {
    u64::from_str_radix(env::var("AUTH_TOKEN_EXPIRE")
        .expect("AUTH_TOKEN_EXPIRE must be configured").as_str(), 10)
        .expect("Cannot parse AUTH_TOKEN_EXPIRE as u64")
});

// Auth request token lifetime
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

// trait for JWT claims
pub trait Claims {
    // create empty claim
    fn default() -> Self;
    // create claim from UUID
    async fn new(uuid: String) -> Result<Self, AuthError> where Self: Sized;
    // generate AuthToken from Claims
    fn generate_token(&self) -> Result<AuthToken, AuthError>
    where Self: Serialize {
        match encode(&Header::default(), &self, &KEYS.encoding) {
            Ok(encoded_string) => {
                Ok(AuthToken::new(encoded_string))
            },
            Err(error) => {
                println!("Error creating token: {}", error);
                Err(AuthError::from_error_type(AuthErrorType::TokenCreation))
            }
        }
    }
    // generate claims from X-Claims header
    fn from_header(headers: &HeaderMap) -> Self
    where Self: for<'de> Deserialize<'de> {
        let value = headers.get("X-Claims").unwrap();
        return serde_json::from_str(&String::from_utf8(BASE64_STANDARD.decode(value).unwrap()).unwrap()).unwrap();
    }
    fn from_string(encoded_str: &str) -> Result<Self, AuthError>
    where Self: Sized,Self: for<'de> Deserialize<'de> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 5;
        validation.set_audience(&[env::var("COMPANY_DOMAIN").unwrap()]);
        validation.set_issuer(&[env::var("COMPANY_NAME").unwrap()]);
        match decode::<Self>(encoded_str, &KEYS.decoding, &validation) {
            Ok(claims) => {
                Ok(claims.claims)
            },
            Err(_) => {
                Err(AuthError::from_error_type(AuthErrorType::InvalidToken))
            }
        }
    }
}

// build claims from request Authorization header
async fn claims_from_request<T>(parts: &mut Parts) -> Result<T, AuthError>
where T: for<'de> Deserialize<'de> {
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthError::from_error_type(AuthErrorType::InvalidToken))?;
    // Build validation strategy
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 5;
    validation.set_audience(&[env::var("COMPANY_DOMAIN").unwrap()]);
    validation.set_issuer(&[env::var("COMPANY_NAME").unwrap()]);
    // Decode the user data
    let token_data = decode::<T>(bearer.token(), &KEYS.decoding, &validation)
    .map_err(|_| AuthError::from_error_type(AuthErrorType::InvalidToken))?;
    Ok(token_data.claims)
}

// Struct for JWT with access level
#[derive(Debug, Serialize, Deserialize, Iterable)]
pub struct AuthClaims {
    pub aud: String,
    pub com: String,
    pub sub: String,
    pub exp: u64,
    pub acc: bool
}

impl Claims for AuthClaims {
    fn default() -> AuthClaims {
        Self {
            // user uuid
            sub: String::new(),
            // issuer domain
            aud: env::var("COMPANY_DOMAIN").unwrap(),
            // issuer company
            com: env::var("COMPANY_NAME").unwrap(),
            // expiration timestamp from unix epoch
                exp: jsonwebtoken::get_current_timestamp() + *TOKEN_LIFETIME,
            // access level
            acc: false
        }
    }
    async fn new(uuid: String) -> Result<AuthClaims, AuthError> {
        match get_db_user_by_uuid(uuid).await {
            Ok(user) => Ok(Self {
                // user uuid
                sub: user.uuid,
                // issuer domain
                aud: env::var("COMPANY_DOMAIN").unwrap(),
                // issuer company
                com: env::var("COMPANY_NAME").unwrap(),
                // expiration timestamp from unix epoch
                exp: jsonwebtoken::get_current_timestamp() + *TOKEN_LIFETIME,
                // access level
                acc: user.is_admin
            }),
            Err(_) => {
                Err(AuthError::from_error_type(AuthErrorType::TokenCreation))
            }
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

// Struct for JWT claims for requesting auth tokens with access level
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequesterClaims {
    pub aud: String,
    pub com: String,
    pub sub: String,
    pub exp: u64
}

impl Claims for AuthRequesterClaims {
    fn default() -> AuthRequesterClaims {
        Self {
            // user uuid
            sub: String::new(),
            // issuer domain
            aud: env::var("COMPANY_DOMAIN").unwrap(),
            // issuer company
            com: env::var("COMPANY_NAME").unwrap(),
            // expiration timestamp from unix epoch
            exp: jsonwebtoken::get_current_timestamp() + *TOKEN_REQUESTER_LIFETIME,
            // access level
        }
    }
    async fn new(uuid: String) -> Result<AuthRequesterClaims, AuthError> {
        Ok(Self {
            // user uuid
            sub: uuid,
            // issuer domain
            aud: env::var("COMPANY_DOMAIN").unwrap(),
            // issuer company
            com: env::var("COMPANY_NAME").unwrap(),
            // expiration timestamp from unix epoch
            exp: jsonwebtoken::get_current_timestamp() + *TOKEN_REQUESTER_LIFETIME,
        })
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

#[derive(Debug)]
pub struct AuthError(types::auth::AuthError);

impl AuthError {
    pub fn from_error_type(error_type: AuthErrorType) -> Self {
        Self {
            0: types::auth::AuthError::from_error_type(error_type)
        }
    }
    pub fn status(&self) -> StatusCode {
        self.0.status.to_owned()
    }
    pub fn body(&self) -> AuthErrorBody {
        self.0.body.to_owned()
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        (self.status(), Json(json!(self.body()))).into_response()
    }
}