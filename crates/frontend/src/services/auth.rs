use gloo_console::{error, log};
use reqwest::{header::{HeaderMap, SET_COOKIE}, StatusCode};
use serde_json::json;
use cookie::{Cookie, CookieBuilder, CookieJar};
use types::user::{LoginUser, RegisterUser, UserInfo};

pub async fn register_user(user: RegisterUser) -> Result<UserInfo, StatusCode> {
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3001/user/create")
        .json(&json!(user))
        .send()
        .await;
    let user = match response {
        Ok(data) => {
            let mut jar = CookieJar::new();
            let status = data.status();
            // let headers: HeaderMap = data.headers().clone();
            // data.headers().values().for_each(|header| {
            //     log!(format!("{}", header.to_str().unwrap()));
            // });
            let cookies = data.cookies();
            for cookie in cookies {
                jar.add(Cookie::build((cookie.name().clone(), cookie.value().clone())));
            }
            // for cookie in cookies {
            //     match cookie.to_str() {
            //         Ok(cookie_str) => {
            //             jar.add_original(Cookie::parse(cookie_str.to_string()).unwrap());
            //             log!(format!("Cookie header: {}", cookie_str));
            //         },
            //         Err(error) => {
            //             log!(format!("Error parsing cookie: {}", error));
            //         }
            //     }
            // }
            log!("Request returned ", status.to_string());
            // Ok(UserInfo::default())
            match data.json::<UserInfo>().await {
                Ok(user) => Ok(user),
                Err(error) => {
                    error!("Error parsing body: {}", error.to_string());
                    Err(status)
                }
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            Err(StatusCode::BAD_GATEWAY)
        }
    };
    return user;
}

pub async fn login_user(user: LoginUser) -> Result<UserInfo, StatusCode>  {
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3001/auth/login")
        .json(&json!(user))
        .send()
        .await;
    let user = match response {
        Ok(data) => {
            let status = data.status();
            match data.json::<UserInfo>().await {
                Ok(user) => Ok(user),
                Err(error) => {
                    error!("Error parsing body: {}", error.to_string());
                    Err(status)
                }
            }
        },
        Err(error) => {
            error!("Error with request: {}", error.to_string());
            Err(StatusCode::BAD_GATEWAY)
        }
    };
    return user;
}