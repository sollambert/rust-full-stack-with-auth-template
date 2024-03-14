use gloo_console::error;
// use gloo_net::http::Request;

use gloo_storage::Storage;
use reqwest::{header::AUTHORIZATION, StatusCode};
use types::user::{LoginUser, RegisterUser, UserInfo};

use super::get_http_client;

pub async fn register_user(user: RegisterUser) -> Result<UserInfo, StatusCode> {
    let response = get_http_client().post("http://localhost:3001/user/create").json(&user).send().await;
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