use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String
}

impl AuthToken {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
    pub fn from_string(string: String) -> Self {
        let split: Vec<&str> = string.split(" ").collect();
        AuthToken {
            token_type: split[0].to_string(),
            access_token: split[1].to_string()
        }
    }
    pub fn to_string(self: AuthToken) -> String {
        let mut string = self.token_type;
        string.push_str((" ".to_owned() + self.access_token.as_str()).as_str());
        string
    }
    pub fn default() -> Self {
        Self {
            access_token: String::new(),
            token_type: String::new()
        }
    }
}