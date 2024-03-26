use gloo_console::error;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION}, Method, StatusCode, Url};
use types::user::UserInfo;

use super::{get_http_auth_client, get_http_client, AuthStorage};

pub async fn get_user_info() -> UserInfo {
    // Attempt to get auth requester token from storage
    let auth_token_result = AuthStorage::get_requester_token();
    if let Err(_) = auth_token_result {
        return UserInfo::new();
    }
    let auth_token = auth_token_result.unwrap();

    // Build auth header from token
    let auth_header_result = HeaderValue::from_str(auth_token.to_string().as_str());
    if let Err(error) = auth_header_result {
        error!("Could not create header from token: {}", error.to_string());
        return UserInfo::new();
    }
    let auth_header = auth_header_result.unwrap();

    // Build request using header map with auth header
    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, auth_header);
    let request_builder = get_http_client()
        .request(Method::GET, Url::parse("http://localhost:3001/user/info").unwrap())
        .headers(header_map);

    // Request user info from server
    let request_result = request_builder.send().await;
    if let Err(error) = request_result {
        error!("Error with request: {}", error.to_string());
        return UserInfo::new();
    }
    let response = request_result.unwrap();
    
    // Parse response body as JSON
    let json_result = response.json::<UserInfo>().await;
    if let Err(error) = json_result {
        error!("Failed to parse response body as json: {}", error.to_string());
        return UserInfo::new();
    }

    // Return UserInfo
    let data = json_result.unwrap();
    return data;
}

pub async fn get_all_users() -> Result<(StatusCode, Vec<UserInfo>), StatusCode> {
    // Request all users from server
    let request_result = get_http_auth_client().get("http://localhost:3001/user/all").send().await;
    if let Err(error) = request_result {
        error!("Error with request: {}", error.to_string());
        return Err(error.status().unwrap());
    }

    // Unwrap request and extract status as owned value
    let response = request_result.unwrap();
    let status = response.status();

    // Parse body as JSON
    let json_result = response.json::<Vec<UserInfo>>().await;
    if let Err(error) = json_result {
        error!("Failed to parse response body as json: {}", error.to_string());
        return Err(error.status().unwrap());
    }

    // Return vec of users
    let users = json_result.unwrap();
    Ok((status, users))
}

pub async fn delete_user(uuid: String) -> Result<StatusCode, StatusCode> {
    // Request to delete user with uuid
    let request_result = get_http_auth_client().delete("http://localhost:3001/user").body(uuid).send().await;
    if let Err(error) = request_result {
        error!("Error with request: {}", error.to_string());
        return Err(error.status().unwrap());
    }

    // Unwrap request and extract status as owned value
    let response = request_result.unwrap();

    // Return status of response
    Ok(response.status())
}