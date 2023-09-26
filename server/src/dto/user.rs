use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub email: String,
    pub display_name: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDtoWrapper {
    pub data: LoginDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpDto {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpDtoWrapper {
    pub data: SignUpDto,
}
