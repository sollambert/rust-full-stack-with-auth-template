use std::str::FromStr;

use gloo_console::{error, log};

use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION}, Method, Request, Response, StatusCode, Url};
use types::user::{LoginUser, RegisterUser, UserInfo};
use yew_router::history::{HashHistory, History};
use reqwest_middleware::{Middleware, Next};
use task_local_extensions::Extensions;

use super::{
    get_http_auth_client,
    get_http_client, AuthStorage};

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
        let auth_requester_token_result = AuthStorage::get_requester_token();
        if auth_requester_token_result.is_ok() {
            request_auth_token().await.unwrap();
        }
        let auth_token = AuthStorage::get_auth_token().unwrap_or_default();
        req.headers_mut().append(AUTHORIZATION, HeaderValue::from_str(auth_token.to_string().as_str()).unwrap());
        next.run(req, extensions).await
    }
}

pub async fn test_auth_route() -> Result<StatusCode, StatusCode> {
    let response = get_http_auth_client().get("http://localhost:3001/auth/test").send().await;
    match response {
        Ok(data) => {
            let status = data.status().clone();
            match data.text().await {
                Ok(text) => {
                    log!(format!("{}", text));
                    Ok(status)
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

pub async fn request_auth_token() -> Result<StatusCode, StatusCode> {
    let auth_token = AuthStorage::get_requester_token().unwrap();
    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, HeaderValue::from_str(auth_token.to_string().as_str()).unwrap());
    let request_builder = get_http_client().request(Method::GET, Url::from_str("http://localhost:3001/auth/request").unwrap())
        .headers(header_map);
    let response = request_builder.send().await;
    match response {
        Ok(data) => {
            let status = data.status();
            let headers = data.headers();
            match headers.get(AUTHORIZATION) {
                Some(header) => {
                    AuthStorage::new(header.to_str().unwrap()).store_auth_token();
                    Ok(status)
                },
                None => {
                    Err(StatusCode::NO_CONTENT)
                }
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            let status = error.status().unwrap();
            match status {
                StatusCode::FORBIDDEN=> {
                    HashHistory::new().push("/login");
                }
                _=>{}
            }
            Err(status)
        }
    }
}

pub async fn register_user(user: RegisterUser) -> Result<UserInfo, StatusCode> {
    let response = get_http_client().post("http://localhost:3001/auth/register").json(&user).send().await;
    match response {
        Ok(data) => {
            let status = data.status();
            let headers = data.headers();
            headers.get_all(AUTHORIZATION).into_iter().for_each(|header| {
                AuthStorage::new(header.to_str().unwrap()).store_requester_token();
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
    }
}

pub async fn login_user(user: LoginUser) -> Result<UserInfo, StatusCode>  {
    let response = get_http_client().post("http://localhost:3001/auth/login").json(&user).send().await;
    match response {
        Ok(data) => {
            let status = data.status();
            let headers = data.headers();
            headers.get_all(AUTHORIZATION).into_iter().for_each(|header| {
                AuthStorage::new(header.to_str().unwrap()).store_requester_token();
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
    }
}