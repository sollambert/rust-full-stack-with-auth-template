use axum::{
    http::StatusCode, middleware, response::{IntoResponse, Response}, routing::post, Json, Router
};
use bcrypt::verify;
use http::{header, HeaderMap, HeaderValue};
use serde::Serialize;
use serde_json::json;
use types::user::{UserInfo, LoginUser};

use crate::{strategies::{users, authentication::{AuthError, generate_new_token}}, middleware::token_athentication};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .route("/protected", post(protected))
        .layer(middleware::from_fn(token_athentication::authenticate_token))
        .route("/login", post(login_user))
}

// response struct for login route
// #[derive(Serialize)]
// struct LoginResponse {
//     auth: AuthBody,
//     user: UserInfo
// }

// example route for authentication protection, will be replaced with middleware
// right now, authentication is only required for routes that extract the Claims object from a requests decoded Bearer token
async fn protected() -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)",
    ))
}

// route for logging in user with provided LoginUser json
async fn login_user(
    Json(payload): Json<LoginUser>,
) -> Result<(StatusCode, HeaderMap, Json<UserInfo>), AuthError> {
    // check if supplied credentials are not empty
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(AuthError::MissingCredentials)
    }
    // get user by username from database
    let result = users::get_db_user_by_username(payload.username).await;
    // if can't get user by username, return 400
    if let Err(_) = result {
        // return (StatusCode::BAD_REQUEST, Json(user_info));
        return Err(AuthError::WrongCredentials)
    }
    // unwrap result from DB as user object
    let user = result.unwrap();
    // verify supplied password is validated
    if verify(payload.pass, &user.pass).unwrap() {
        // build response user
        let user_info = UserInfo {
            uuid: user.uuid,
            username: user.username,
            email: user.email
        };
        let mut header_map = HeaderMap::new();
        let token = generate_new_token();
        let header_value = HeaderValue::from_str(("auth_token=".to_string() + json!(token).to_string().as_str()).as_str()).unwrap();
        header_map.insert(header::SET_COOKIE, header_value);
        Ok((StatusCode::OK, header_map.clone(), axum::Json(user_info.clone())))
    } else {
        // send 400 response with JSON response
        Err(AuthError::WrongCredentials)
    }
}