use gloo_console::error;
// use gloo_net::http::Request;

use gloo_storage::{errors::StorageError, Storage};
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Method, Request, RequestBuilder, StatusCode, Url};
use tauri_sys::global_shortcut;
use types::{auth::AuthToken, user::{LoginUser, RegisterUser, UserInfo}};

use super::get_http_client;

pub async fn request_auth_token() -> Result<StatusCode, StatusCode> {
    let response = get_http_client().post("http://localhost:3001/auth/request").send().await;
    match response {
        Ok(data) => {
            let status = data.status();
            let headers = data.headers();
            match headers.get(AUTHORIZATION) {
                Some(header) => {
                    gloo_storage::SessionStorage::set("AUTH_TOKEN", header.to_str().unwrap()).unwrap();
                    Ok(status)
                },
                None => {
                    Err(StatusCode::NO_CONTENT)
                }
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn register_user(user: RegisterUser) -> Result<UserInfo, StatusCode> {
    let response = get_http_client().post("http://localhost:3001/auth/register").json(&user).send().await;
    let auth_body = match response {
        Ok(data) => {
            let status = data.status();
            let headers = data.headers();
            headers.get_all(AUTHORIZATION).into_iter().for_each(|header_value| {
                gloo_storage::SessionStorage::set("AUTH_REQUESTER_TOKEN", header_value.to_str().unwrap()).unwrap();
            });
            match data.json::<UserInfo>().await {
                Ok(auth_body) => {
                    Ok(auth_body)
                },
                Err(error) => {
                    error!("Error parsing body: {}", error.to_string());
                    Err(status)
                }
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            Err(StatusCode::BAD_REQUEST)
        }
    };
    return auth_body;
}

pub async fn login_user(user: LoginUser) -> Result<UserInfo, StatusCode>  {
    let response = get_http_client().post("http://localhost:3001/auth/login").json(&user).send().await;
    let auth_body = match response {
        Ok(data) => {
            let status = data.status();
            let headers = data.headers();
            headers.get_all(AUTHORIZATION).into_iter().for_each(|header_value| {
                gloo_storage::SessionStorage::set("AUTH_REQUESTER_TOKEN", header_value.to_str().unwrap()).unwrap();
            });
            match data.json::<UserInfo>().await {
                Ok(auth_body) => {
                    Ok(auth_body)
                },
                Err(error) => {
                    error!("Error parsing body: {}", error.to_string());
                    Err(status)
                }
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            Err(StatusCode::BAD_REQUEST)
        }
    };
    return auth_body;
}

// fn build_auth_requester_request(method: Method, url:Url ) -> Result<Request, StorageError> {
//     let request = RequestBuilder::from_parts(get_http_client(), Request::new(method, url));
//     let auth_token = get_auth_requester_token();
//     let headers = HeaderMap::new();
// }

fn get_auth_requester_token() -> Result<AuthToken, StorageError> {
    match gloo_storage::SessionStorage::get::<String>("AUTH_REQUESTER_TOKEN") {
        Ok(token_string) => {
            Ok(AuthToken::from_string(token_string))
        },
        Err(error) => {
            Err(error)
        }
    }
}