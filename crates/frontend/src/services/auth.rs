use gloo_console::{error, log};
// use gloo_net::http::Request;

use gloo_storage::Storage;
use reqwest::{Client, StatusCode};
use types::user::{LoginUser, RegisterUser, UserInfo};

pub async fn register_user(user: RegisterUser) -> Result<UserInfo, StatusCode> {
    // let response = Request::post("http://localhost:3001/user/create")
    //     .json(&user).unwrap()
    //     .send()
    //     .await;
    let response = Client::builder()
        .build()
        .unwrap().post("http://localhost:3001/user/create").json(&user).send().await;
    // let response = Request::post("http://localhost:3001/user/create")
    //     .json(&user).unwrap().send().await;
    let user = match response {
        Ok(data) => {
            log!(format!("Data: {:?}", data));
            // let status = StatusCode::from_u16(data.status()).unwrap();
            let status = data.status();
            data.headers().into_iter().for_each(|header| {
                log!(format!("{} = {:?}", header.0, header.1));
                if header.0 == "cookie" {
                    let cookie: Vec<&str> = header.1.to_str().unwrap().split("=").collect();
                    gloo_storage::LocalStorage::set(cookie[0],cookie[1]).unwrap();
                }
            });
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
            Err(StatusCode::BAD_REQUEST)
        }
    };
    return user;
}

pub async fn login_user(user: LoginUser) -> Result<UserInfo, StatusCode>  {
    // let response = Request::post("http://localhost:3001/user/create")
    // .json(&user).unwrap()
    // .send()
    // .await;
    let response = Client::builder()
        .build()
        .unwrap().post("http://localhost:3001/user/create").json(&user).send().await;
    let user = match response {
        Ok(data) => {
            // let status = StatusCode::from_u16(data.status()).unwrap();
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
            Err(StatusCode::BAD_REQUEST)
        }
    };
    return user;
}