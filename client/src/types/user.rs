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
    pub password: Option<String>,
}
