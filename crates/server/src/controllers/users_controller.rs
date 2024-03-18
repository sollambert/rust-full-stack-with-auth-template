use axum::{
    http::StatusCode,
    routing::get,
    Json,Router
};

use types::user::UserInfo;

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .route("/", get(default_user))
}

//default route
async fn default_user() -> (StatusCode, Json<UserInfo>) {
    return (StatusCode::OK, Json(UserInfo {
        uuid: "Empty user".to_owned(),
        username: "Empty user".to_owned(),
        email: "Empty user".to_owned(),
        is_admin: false
    }));
}
