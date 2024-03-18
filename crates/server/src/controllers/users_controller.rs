use axum::{
    extract::Request, http::StatusCode, middleware, routing::get, Json, Router
};

use types::user::UserInfo;

use crate::{middleware::token_authentication, strategies::{authentication::{AuthError, AuthRequesterClaims, Claims}, users::get_db_user_by_uuid}};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .nest("/info", Router::new()
            .route("/",get(get_user_info))
            .layer(middleware::from_fn(token_authentication::authenticate_token::<AuthRequesterClaims>)))
}


// get user info by JWT claims
async fn get_user_info(request: Request) -> Result<(StatusCode, Json<UserInfo>), AuthError> {
    // generate AuthRequesterClaims from encoded x-claim header
    let claims = AuthRequesterClaims::from_header(request.headers());
    match get_db_user_by_uuid(claims.sub).await {
        Ok(user) => {
            Ok((StatusCode::OK, axum::Json(UserInfo {
                uuid: user.uuid,
                username: user.username,
                email: user.email,
                is_admin: user.is_admin
            })))
        }, Err(_) => Err(AuthError::UserDoesNotExist)
    }
}