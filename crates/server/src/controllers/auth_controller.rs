use axum::{
    http::StatusCode, middleware, routing::post, Json, Router
};
use bcrypt::verify;
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use types::{auth::AuthToken, user::{LoginUser, UserInfo}};

use crate::{strategies::{users, authentication::{AuthError, generate_requester_token}}, middleware::token_authentication};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .route("/protected", post(protected))
        .layer(middleware::from_fn(token_authentication::authenticate_token))
        .route("/login", post(login_user))
}

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
) -> Result<(StatusCode, HeaderMap, Json<UserInfo>), (StatusCode, AuthError)> {
    // check if supplied credentials are not empty
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err((StatusCode::FORBIDDEN, AuthError::WrongCredentials));
    }
    // get user by username from database
    let result = users::get_db_user_by_username(payload.username).await;
    // if can't get user by username, return 400
    if let Err(_) = result {
        return Err((StatusCode::FORBIDDEN, AuthError::WrongCredentials));
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
        let token_result = generate_requester_token(user_info.uuid.clone());
        let auth_token: AuthToken;
        match token_result {
            Ok(token) => auth_token = token,
            Err(error) => {
                println!("Error creating token for UUID {}: {:?}", user_info.uuid, error);
                return Err((StatusCode::FORBIDDEN, error))
            }
        }
        let mut header_map = HeaderMap::new();
        header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());
        Ok((StatusCode::CREATED, header_map.clone(), axum::Json(user_info)))
    } else {
        // send 400 response with JSON response
            return Err((StatusCode::FORBIDDEN, AuthError::WrongCredentials));
    }
}