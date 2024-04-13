use std::str::FromStr;

use gloo_console::{error, log};

use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION}, Method, Request, Response, StatusCode, Url};
use types::user::{LoginUser, RegisterUser, UserInfo};
use reqwest_middleware::{Middleware, Next};
use task_local_extensions::Extensions;

use super::{
    get_http_auth_client, get_http_client, AuthError, AuthStorage};

pub struct AuthMiddleware;

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
impl Middleware for AuthMiddleware {
    #![allow(trait_impl_incorrect_safety)]
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        // Get auth requester token from storage
        let auth_requester_token_result = AuthStorage::get_requester_token();
        if let Err(error) =  auth_requester_token_result {
            error!(format!("{:?}", error));
        }

        // Request an auth token
        let auth_token_request_result = request_auth_token().await;
        if let Err(error) = auth_token_request_result {
            error!(format!("Error requesting auth token: {}", error.body().message));
        }

        // Get auth token from storage
        let auth_token = AuthStorage::get_auth_token().unwrap_or_default();
        
        // Create auth header and add to request headers
        let header_value_result = HeaderValue::from_str(auth_token.to_string().as_str());
        if let Err(error) = header_value_result {
            error!(format!("Could not create auth header: {error}"));
        } else {
            req.headers_mut().append(AUTHORIZATION, header_value_result.unwrap());
        }

        // Run next operation of HTTP request
        next.run(req, extensions).await
    }
}

pub async fn test_auth_route() -> Result<StatusCode, AuthError> {
    // Send request to test auth route
    let request_result = get_http_auth_client().get("http://localhost:3001/auth/test").send().await;
    if let Err(error) = request_result {
        error!("Error with request: {}", error.to_string());
        return Err(AuthError::default());
    }
    // Unwrap resonse from request_result
    let response = request_result.unwrap();
    let status = response.status();

    // Check if status is success
    if !status.is_success() {
        return Err(AuthError::from_response(response).await);
    }

    // Extract text from body and log to console
    let text_result = response.text().await;
    if let Err(error) = text_result {
        error!(format!("Error with parsing body as text: {error}"));
        Err(AuthError::default())
    } else {
        let text = text_result.unwrap();
        log!(format!("{text}"));
        Ok(status)
    }
}

pub async fn request_auth_token() -> Result<StatusCode, AuthError> {
    // Build auth header from token
    let auth_token_result = AuthStorage::get_requester_token();
    if let Err(error) = &auth_token_result {
        error!(format!("{:?}", error));
    }
    let auth_token = auth_token_result.unwrap();

    // Build request using header map with auth header
    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, HeaderValue::from_str(auth_token.to_string().as_str()).unwrap());
    let request_builder = get_http_client()
        .request(Method::GET, Url::from_str("http://localhost:3001/auth/request").unwrap())
        .headers(header_map);
    let request_result = request_builder.send().await;
    if let Err(error) = request_result {
        error!("Error with request: {}", error.to_string());
        return Err(AuthError::default());
    }

    // Unwrap response from request_result
    let response = request_result.unwrap();

    // Get status and match for responsive behavior
    let status = response.status();

    // Check if status is success
    if !status.is_success() {
        return Err(AuthError::from_response(response).await);
    }

    // Extract auth header from headers
    let headers = response.headers();
    let auth_header_result = headers.get(AUTHORIZATION);
    if let None = auth_header_result {
        return Err(AuthError::default());
    }
    let header = auth_header_result.unwrap();
    let header_str = header.to_str().unwrap_or("");

    // Store auth token
    AuthStorage::new(header_str).store_auth_token();
    Ok(status)
}

pub async fn register_user(user: RegisterUser) -> Result<UserInfo, AuthError> {
    // Send register data to server
    let request_result = get_http_client().post("http://localhost:3001/auth/register").json(&user).send().await;
    if let Err(error) = request_result {
        error!("Error with request: {}", error.to_string());
        return Err(AuthError::default());
    }

    // Unwrap response from request_result
    let response = request_result.unwrap();
    let status = response.status();

    // Check if status is success
    if !status.is_success() {
        return Err(AuthError::from_response(response).await);
    }

    // Extract auth requester token from headers and store in local browser storage
    let headers = response.headers();
    headers.get_all(AUTHORIZATION).into_iter().for_each(|header| {
        let header_str_result = header.to_str();
        if let Err(error) = &header_str_result {
            error!("Error converting header to str: {}", error.to_string());
        }
        let header_str = header_str_result.unwrap();
        AuthStorage::new(&header_str).store_requester_token();
    });

    // Extract user info from json body
    let json_result = response.json::<UserInfo>().await;
    if let Err(error) = json_result {
        error!("Error parsing body: {}", error.to_string());
        return Err(AuthError::default());
    }

    // Unwrap JSON result and return as OK result
    let data = json_result.unwrap();
    return Ok(data);
}

pub async fn login_user(user: LoginUser) -> Result<UserInfo, AuthError>  {
    // Send login data to server
    let request_result = get_http_client().post("http://localhost:3001/auth/login").json(&user).send().await;
    if let Err(error) = request_result {
        error!("Error with request: {}", error.to_string());
        return Err(AuthError::default());
    }

    // Unwrap response from request_result
    let response = request_result.unwrap();
    let status = response.status();

    // Check if status is success
    if !status.is_success() {
        return Err(AuthError::from_response(response).await);
    }

    // Extract auth requester token from headers and store in local browser storage
    let headers = response.headers();
    headers.get_all(AUTHORIZATION).into_iter().for_each(|header| {
        let header_str_result = header.to_str();
        if let Err(error) = &header_str_result {
            error!("Error converting header to str: {}", error.to_string());
        }
        let header_str = header_str_result.unwrap();
        AuthStorage::new(&header_str).store_requester_token();
    });

    // Extract user info from json body
    let json_result = response.json::<UserInfo>().await;
    if let Err(error) = json_result {
        return Err(AuthError::default())
    }

    // Unwrap JSON result and return as OK result
    let data = json_result.unwrap();
    return Ok(data);
}

pub fn logout_user() {
    // Clear local auth storage to remove auth tokens
    AuthStorage::clear();
}