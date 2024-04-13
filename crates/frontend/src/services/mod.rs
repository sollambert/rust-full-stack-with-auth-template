use gloo_storage::{Storage, errors::StorageError};
use once_cell::sync::OnceCell;
use reqwest::{Client, StatusCode};
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

pub struct AuthStorage<'a> {
    pub token_string: &'a str
}

impl <'a>AuthStorage<'a> {
    const TOKEN_KEY: &'a str = "AUTH_TOKEN";
    const REQUESTER_TOKEN_KEY: &'a str = "AUTH_REQUESTER_TOKEN";
    fn store(&self, token_key: &str) {
        gloo_storage::LocalStorage::set(token_key, &self.token_string).unwrap();
    }
    fn clear() {
        gloo_storage::LocalStorage::delete(Self::TOKEN_KEY);
        gloo_storage::LocalStorage::delete(Self::REQUESTER_TOKEN_KEY);
    }
    fn get(token_key: &str) -> Result<AuthToken, StorageError>  {
        match gloo_storage::LocalStorage::get(token_key) {
            Ok(token_string) => {
                Ok(AuthToken::from_string(token_string))
            },
            Err(storage_error) => Err(storage_error)
        }
    }
    pub fn get_requester_token() -> Result<AuthToken, StorageError> {
        Self::get(Self::REQUESTER_TOKEN_KEY)
    }
    pub fn get_auth_token() -> Result<AuthToken, StorageError> {
        Self::get(Self::TOKEN_KEY)
    }
    pub fn store_requester_token(&self) {
        self.store(Self::REQUESTER_TOKEN_KEY);
    }
    pub fn store_auth_token(&self) {
        self.store(Self::TOKEN_KEY);
    }
    pub fn new(token_string: &'a str) -> Self {
        Self {
            token_string
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
        AuthError  {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: AuthErrorBody {
                error_type: AuthErrorType::ServerError,
                message: String::from("Auth service unavailable.")
            }
        }
    }
}