use gloo_console::error;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION}, Method, Url};
use types::user::UserInfo;

use super::{get_http_client, AuthStorage};

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