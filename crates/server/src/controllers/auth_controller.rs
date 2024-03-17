use axum::{
    extract::Request, http::StatusCode, middleware, routing::{get, post}, Json, Router
};
use bcrypt::verify;
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use types::{auth::AuthToken, user::{LoginUser, RegisterUser, UserInfo}};

use crate::{middleware::token_authentication, strategies::{authentication::{AuthClaims, AuthError, AuthRequesterClaims, Claims}, users}};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .nest("/test", Router::new()
            .route("/", get(test_auth_route)))
            .layer(middleware::from_fn(token_authentication::authenticate_token::<AuthClaims>))
        .nest("/request", Router::new()
            .route("/",get(request_auth_token))
            .layer(middleware::from_fn(token_authentication::authenticate_token::<AuthRequesterClaims>)))
        .route("/login", post(login_user))
        .route("/register", post(register_user))
}

async fn test_auth_route(request: Request) -> Result<(StatusCode, String), (StatusCode, AuthError)> {
    let claims = AuthClaims::from_header(request.headers()).await;
    println!("Claims in route: {:?}", claims);
    Ok((StatusCode::OK, "Auth verified".to_string()))
}

async fn request_auth_token(request: Request) -> Result<(StatusCode, HeaderMap), (StatusCode, AuthError)> {
    let claims = AuthRequesterClaims::from_header(request.headers()).await;
    let token_result = AuthClaims::new(claims.sub.clone()).await.unwrap().generate_token();
    let auth_token: AuthToken;
    match token_result {
        Ok(token) => auth_token = token,
        Err(error) => {
            println!("Error creating token for UUID {}: {:?}", claims.sub, error);
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
        return Err((StatusCode::FORBIDDEN, AuthError::UserDoesNotExist));
    }
    // unwrap result from DB as user object
    let user = result.unwrap();
    // verify supplied password is validated
    if verify(payload.pass, &user.pass).is_ok() {
        // build response user
        let user_info = UserInfo {
            uuid: user.uuid,
            username: user.username,
            email: user.email
        };
    let token_result = AuthRequesterClaims::new(user_info.uuid.clone()).await.unwrap().generate_token();
        let auth_token: AuthToken;
        match token_result {
            Ok(token) => auth_token = token,
            Err(error) => {
                println!("Error creating token for UUID {}: {:?}", user_info.uuid, error);
                return Err((StatusCode::UNAUTHORIZED, error))
            }
        }
        let mut header_map = HeaderMap::new();
        header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());
        Ok((StatusCode::CREATED, header_map.clone(), axum::Json(user_info)))
    } else {
        // send 400 response with JSON response
        return Err((StatusCode::UNAUTHORIZED, AuthError::WrongCredentials));
    }
}


// handler for creating a new user
async fn register_user(
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, HeaderMap, Json<UserInfo>), (StatusCode, AuthError)> {
    // insert user into table
    match users::insert_db_user(payload).await {
        Ok(user) => {
            // re-create user_info with populated fields
            let user_info = UserInfo {
                uuid: user.uuid.clone(),
                email: user.email,
                username: user.username
            };
            let token_result = AuthRequesterClaims::new(user_info.uuid.clone()).await.unwrap().generate_token();
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
        Err(error) => {
            println!("Error creating user: {}", error);
            if error.to_string().contains("duplicate key") {
                return Err((StatusCode::BAD_REQUEST, AuthError::UserAlreadyExists))
            }
            Err((StatusCode::INTERNAL_SERVER_ERROR, AuthError::InvalidToken))
        }
    }
}