use gloo_console::{error, log};
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION}, Method, StatusCode, Url};
use types::user::UserInfo;

use super::{get_http_auth_client, get_http_client, AuthStorage};

pub async fn get_user_info() -> UserInfo {
    let auth_token = AuthStorage::get_requester_token();
    if let Err(_) = auth_token {
        return UserInfo::new();
    }
    let auth_token = auth_token.unwrap();
    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, HeaderValue::from_str(auth_token.to_string().as_str()).unwrap());
    let request_builder = get_http_client().request(Method::GET, Url::parse("http://localhost:3001/user/info").unwrap())
        .headers(header_map);
    let response = request_builder.send().await;
    match response {
        Ok(data) => {
            let status = data.status();
            if status.is_success() {
                match data.json::<UserInfo>().await {
                    Ok(auth_body) => {
                        auth_body
                    },
                    Err(error) => {
                        error!("Error parsing body: {}", error.to_string());
                        UserInfo::new()
                    }
                }
            } else {
                UserInfo::new()
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            UserInfo::new()
        }
    }
}

pub async fn get_all_users() -> Result<(StatusCode, Vec<UserInfo>), StatusCode> {
    let response = get_http_auth_client().get("http://localhost:3001/user/all").send().await;
    match response {
        Ok(data) => {
            let status = data.status().clone();
            match data.json::<Vec<UserInfo>>().await {
                Ok(users) => {
                    Ok((status, users))
                },
                Err(_error) => {
                    Err(status)
                }
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            Err(error.status().unwrap())
        }
    }
}

pub async fn delete_user(uuid: String) -> Result<StatusCode, StatusCode> {
    let response = get_http_auth_client().delete("http://localhost:3001/user").body(uuid).send().await;
    match response {
        Ok(response) => {
            Ok(response.status())
        },
        Err(error) => {
            Err(error.status().unwrap())
        }
    }
}