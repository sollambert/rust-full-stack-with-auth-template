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
        .route("/create", post(create_user))
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


// handler for creating a new user
async fn create_user(
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
        },
        Err(_) => {
            // send 500 SERVICE UNAVAILABLE with empty ResponseUser
            return Err((StatusCode::SERVICE_UNAVAILABLE, AuthError::InvalidToken));
        }
    }
}
