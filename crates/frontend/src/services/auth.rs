use gloo_console::log;
use serde_json::json;
use types::user::{LoginUser, ResponseUser};

pub async fn login_user(user: LoginUser) -> ResponseUser {
    let empty_user = ResponseUser {
        uuid: String::new(),
        username: String::new(),
        email: String::new()
    };
    let client = reqwest::Client::new();
    let response = match client.post("http://localhost:3001/auth/login")
        .json(&json!(user))
        .send()
        .await {
            Ok(response) => Some(response),
            Err(error) => {
                log!("Error signing in: {}", error.to_string());
                None
            }
        };
    
    match response {
        Some(data) => match data.json::<ResponseUser>().await {
            Ok(user) => user,
            Err(error) => {
                log!("Error parsing user: {}", error.to_string());
                empty_user
            }
        },
        None => {
            empty_user
        }
    }
}