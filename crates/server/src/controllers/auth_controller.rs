use axum::{
    extract::Request, http::StatusCode, middleware, routing::post, Json, Router
};
use bcrypt::verify;
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use types::{auth::{AuthError, AuthToken}, user::{LoginUser, RegisterUser, UserInfo}};

use crate::{middleware::token_authentication, strategies::{authentication::{AuthClaims, AuthRequesterClaims, Claims}, users}};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .route("/request", post(request_auth_token))
        .layer(middleware::from_fn(token_authentication::authenticate_requester_token::<AuthRequesterClaims>))
        .route("/login", post(login_user))
        .route("/register", post(register_user))
}

async fn request_auth_token(request: Request) -> Result<(StatusCode, HeaderMap), (StatusCode, AuthError)> {
    let mut uuid: String = String::new();
    request.headers().get("User-UUID").into_iter().for_each(|header| {
        uuid = String::from(header.to_str().unwrap());
    });
    let token_result = AuthClaims::new(uuid.clone()).generate_token();
    let auth_token: AuthToken;
    match token_result {
        Ok(token) => auth_token = token,
        Err(error) => {
            println!("Error creating token for UUID {}: {:?}", uuid, error);
            return Err((StatusCode::FORBIDDEN, error))
        }
    }
    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());
    Ok((StatusCode::CREATED, header_map.clone()))
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
        let token_result = AuthRequesterClaims::new(user_info.uuid.clone()).generate_token();
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


// handler for creating a new user
async fn register_user(
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, HeaderMap, Json<UserInfo>), (StatusCode, AuthError)> {
    // insert user into table
    // if successful return a valid ResponseUser and 201 CREATED
    // if unsuccessful return an empty ResponseUser object and a 400 BAD REQUEST
    match users::insert_db_user(payload).await {
        Ok(id) => {
            // query to select user by given ID return by insert_user function
            // then return populated ResponseUser with data from table
            let result = users::get_db_user_by_id(id).await;
            // check result for error and return error code if necessary
            if let Err(_) = result {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, AuthError::InvalidToken));
            }
            let user = result.unwrap();
            // re-create user_info with populated fields
            let user_info = UserInfo {
                uuid: user.uuid,
                email: user.email,
                username: user.username
            };
            let token_result = AuthRequesterClaims::new(user_info.uuid.clone()).generate_token();
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
        },
        Err(_) => {
            // send 500 SERVICE UNAVAILABLE with empty ResponseUser
            return Err((StatusCode::SERVICE_UNAVAILABLE, AuthError::InvalidToken));
        }
    }
}