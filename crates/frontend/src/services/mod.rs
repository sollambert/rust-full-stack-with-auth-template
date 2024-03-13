pub mod auth;

use gloo_storage::Storage;
use once_cell::sync::OnceCell;
use reqwest::Client;

static HTTP_CLIENT: OnceCell<Client> = OnceCell::new();
static TOKEN_KEY: &str = "AUTH_TOKEN";

pub fn create_http_client() {
    HTTP_CLIENT.set(Client::builder()
    .build()
    .unwrap()).unwrap();
}

pub fn get_http_client() -> Client {
    HTTP_CLIENT.get().unwrap().to_owned()
}

fn get_auth_token_string() -> String {
    gloo_storage::SessionStorage::get(TOKEN_KEY).unwrap_or(
        gloo_storage::LocalStorage::get(TOKEN_KEY).unwrap_or(String::new()))
}