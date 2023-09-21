use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SignUpRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoginRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserInfo {
    pub email: String,
    pub username: String,
    pub token: String,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserInfoWrapper {
    pub user_info: UserInfo,
}
