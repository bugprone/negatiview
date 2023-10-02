use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SignUpDto {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserDto {
    pub email: String,
    pub display_name: String,
    pub access_token: String,
    pub biography: String,
    pub profile_image_url: String,
}

impl UserDto {
    pub fn is_authenticated(&self) -> bool {
        !self.access_token.is_empty()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserUpdateDto {
    pub email: String,
    pub display_name: String,
    pub password: Option<String>,
    pub biography: String,
    pub profile_image_url: String,
}
