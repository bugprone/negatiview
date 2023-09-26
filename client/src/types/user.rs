use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoginDto {
    pub email: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginDtoWrapper {
    pub data: LoginDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SignUpDto {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
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
    // pub image: String,
    // pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateDtoWrapper {
    pub data: UserUpdateDto,
}
