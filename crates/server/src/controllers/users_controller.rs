use axum::{
    extract::{Json, Request}, http::StatusCode, middleware, routing::{delete, get}, RequestExt, Router
};

use types::{auth::AuthErrorType, user::UserInfo};

use crate::{middleware::token_authentication, strategies::{authentication::{AuthClaims, AuthError, AuthRequesterClaims, Claims}, users::{delete_user_by_uuid, get_all_users, get_db_user_by_uuid}}};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .nest("/info", Router::new()
            .route("/",get(get_user_info))
            .layer(middleware::from_fn(token_authentication::authenticate_token::<AuthRequesterClaims>)))
        .nest("/all", Router::new()
            .route("/",get(get_all_user_info))
            .layer(middleware::from_fn(token_authentication::authenticate_token::<AuthClaims>)))
        .nest("/", Router::new()
            .route("/", delete(delete_user))
            .layer(middleware::from_fn(token_authentication::authenticate_token::<AuthClaims>)))
}


// get user info by JWT claims
async fn get_user_info(request: Request) -> Result<(StatusCode, Json<UserInfo>), AuthError> {
    // generate AuthRequesterClaims from encoded x-claim header
    let claims = AuthRequesterClaims::from_header(request.headers());
    match get_db_user_by_uuid(claims.sub).await {
        Ok(user) => {
            Ok((StatusCode::OK, axum::Json(UserInfo::from_user(user))))
        }, Err(_) => Err(AuthError::from_error_type(AuthErrorType::UserDoesNotExist))
    }
}

// get user info by JWT claims
async fn get_all_user_info(request: Request) -> Result<(StatusCode, Json<Vec<UserInfo>>), AuthError> {
    // generate AuthRequesterClaims from encoded x-claim header
    let claims = AuthClaims::from_header(request.headers());
    if claims.acc {
        match get_all_users().await {
            Ok(users) => {
                Ok((StatusCode::OK, axum::Json(users)))
            }, Err(_) => Err(AuthError::from_error_type(AuthErrorType::UserDoesNotExist))
        }
    } else {
        Err(AuthError::from_error_type(AuthErrorType::AccessDenied))
    }
}

async fn delete_user(request: Request) -> Result<StatusCode, AuthError> {
    // generate AuthRequesterClaims from encoded x-claim header
    let claims = AuthClaims::from_header(request.headers());
    let uuid = request.extract().await;
    match uuid {
        Ok(uuid) => {
            if claims.acc {
                match delete_user_by_uuid(uuid).await {
                    Ok(_) => {
                        Ok(StatusCode::OK)
                    }, Err(_) => Err(AuthError::from_error_type(AuthErrorType::UserDoesNotExist))
                }
            } else {
                Err(AuthError::from_error_type(AuthErrorType::AccessDenied))
            }
        }, Err(error) => {
            println!("{error}");
            Err(AuthError::from_error_type(AuthErrorType::ServerError))
        }
    }
}