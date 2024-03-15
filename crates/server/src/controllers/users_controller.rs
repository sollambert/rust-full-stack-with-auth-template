use axum::{
    http::StatusCode,
    routing::{get, post},
    Json,Router
};

use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use types::{auth::AuthToken, user::{RegisterUser, UserInfo}};

use crate::strategies::{authentication::{generate_requester_token, AuthError}, users};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .route("/", get(default_user))
}

//default route
async fn default_user() -> (StatusCode, Json<UserInfo>) {
    return (StatusCode::OK, Json(UserInfo {
        uuid: "".to_owned(),
        username: "".to_owned(),
        email: "".to_owned()
    }));
}
