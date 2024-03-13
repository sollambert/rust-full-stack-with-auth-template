use serde::{Deserialize, Serialize};

use crate::user::UserInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    access_token: String,
    token_type: String
}

impl AuthToken {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    pub token: AuthToken,
    pub user_info: UserInfo
}