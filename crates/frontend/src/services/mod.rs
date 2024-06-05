use std::str::FromStr;

use gloo_console::error;
use gloo_storage::{Storage, errors::StorageError};
use once_cell::sync::OnceCell;
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Client, Response, StatusCode};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use types::auth::{AuthErrorBody, AuthErrorType, AuthToken};

use self::auth::AuthMiddleware;

pub mod auth;
pub mod user;

static HTTP_CLIENT: OnceCell<Client> = OnceCell::new();
static HTTP_CLIENT_WITH_AUTH: OnceCell<ClientWithMiddleware> = OnceCell::new();

pub fn create_http_clients() {
    HTTP_CLIENT.set(Client::builder()
    .build()
    .unwrap()).unwrap();
    HTTP_CLIENT_WITH_AUTH.set(ClientBuilder::new(Client::builder().build().unwrap())
        .with(AuthMiddleware)
        .build()).unwrap();
}

pub fn get_http_client() -> Client {
    HTTP_CLIENT.get().unwrap().to_owned()
}

pub fn get_http_auth_client() -> ClientWithMiddleware {
    HTTP_CLIENT_WITH_AUTH.get().unwrap().to_owned()
}

pub struct AuthStorage;

impl AuthStorage {
    const TOKEN_KEY: &'static str = "AUTH_TOKEN";
    const REQUESTER_TOKEN_KEY: &'static str = "AUTH_REQUESTER_TOKEN";
    fn store(token_key: &str, token_string: &str) {
        gloo_storage::LocalStorage::set(token_key, token_string).unwrap();
    }
    fn store_from_headers(headers: &HeaderMap) {
        headers.get_all(AUTHORIZATION).into_iter().for_each(|header| {
            let header_str_result = header.to_str();
            if let Err(error) = &header_str_result {
                error!("Error converting header to str: {}", error.to_string());
            }
            let header_str = header_str_result.unwrap();
            AuthStorage::store_requester_token(AuthToken::from_string(header_str.to_string()));
        });
    }
    fn get(token_key: &str) -> Result<AuthToken, StorageError>  {
        match gloo_storage::LocalStorage::get(token_key) {
            Ok(token_string) => {
                Ok(AuthToken::from_string(token_string))
            },
            Err(storage_error) => Err(storage_error)
        }
    }
    pub fn clear() {
        gloo_storage::LocalStorage::delete(Self::TOKEN_KEY);
        gloo_storage::LocalStorage::delete(Self::REQUESTER_TOKEN_KEY);
    }
    pub fn get_requester_token() -> Result<AuthToken, StorageError> {
        Self::get(Self::REQUESTER_TOKEN_KEY)
    }
    pub fn get_auth_token() -> Result<AuthToken, StorageError> {
        Self::get(Self::TOKEN_KEY)
    }
    pub fn store_requester_token(token: AuthToken) {
        Self::store(Self::REQUESTER_TOKEN_KEY, &token.to_string());
    }
    pub fn store_auth_token(token: AuthToken) {
        Self::store(Self::TOKEN_KEY, &token.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct AuthError(types::auth::AuthError);

impl AuthError {
    pub async fn from_response(response: Response) -> Self {
        let status: StatusCode = response.status();
        let error_body = response.json::<AuthErrorBody>().await;
        if let Err(_) = error_body {
            return Self {
                0: types::auth::AuthError::default()
            }
        }
        return Self  {
            0: types::auth::AuthError {
                status: http::StatusCode::from_str(status.as_str()).unwrap(),
                body: error_body.unwrap()
            }
        };
    }
    pub fn from_error_type(error_type: AuthErrorType) -> Self {
        Self {
            0: types::auth::AuthError::from_error_type(error_type)
        }
    }
    pub fn body(&self) -> AuthErrorBody {
        self.0.body.to_owned()
    }
    pub fn default() -> Self {
        Self {
            0: types::auth::AuthError::default()
        }
    }
}