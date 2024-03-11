use gloo_console::{error, log};
use reqwest::StatusCode;
use serde_json::json;
use types::user::{LoginUser, RegisterUser, UserInfo};

pub async fn register_user(user: RegisterUser) -> Result<UserInfo, StatusCode> {
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3001/user/create")
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