use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginDtoWrapper {
    pub data: LoginDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SignUpDto {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignUpDtoWrapper {
    pub data: SignUpDto,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserDto {
    pub email: String,
    pub display_name: String,
    pub token: String,
    pub biography: String,
    pub profile_image_url: String,
}

impl UserDto {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserDtoWrapper {
    pub data: UserDto,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserUpdateDto {
    pub email: String,
    pub display_name: String,
    pub password: Option<String>,
    pub biography: String,
    pub profile_image_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateDtoWrapper {
    pub data: UserUpdateDto,
}
