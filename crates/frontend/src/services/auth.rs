use gloo_console::error;
// use gloo_net::http::Request;

use gloo_storage::Storage;
use reqwest::{Client, StatusCode};
use types::{auth::AuthBody, user::{LoginUser, RegisterUser}};

pub async fn register_user(user: RegisterUser) -> Result<AuthBody, StatusCode> {
    let response = Client::builder()
        .build()
        .unwrap().post("http://localhost:3001/user/create").json(&user).send().await;
    let auth_body = match response {
        Ok(data) => {
            let status = data.status();
            match data.json::<AuthBody>().await {
                Ok(auth_body) => {
                    gloo_storage::LocalStorage::set("AUTH_TOKEN", auth_body.clone().token).unwrap();
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

pub async fn login_user(user: LoginUser) -> Result<AuthBody, StatusCode>  {
    let response = Client::builder()
        .build()
        .unwrap().post("http://localhost:3001/user/create").json(&user).send().await;
    let auth_body = match response {
        Ok(data) => {
            let status = data.status();
            match data.json::<AuthBody>().await {
                Ok(auth_body) => {
                    gloo_storage::LocalStorage::set("AUTH_TOKEN", auth_body.clone().token).unwrap();
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