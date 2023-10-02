use crate::middlewares::error::Error;
use crate::middlewares::request::{request_get, request_post, request_put};
use crate::types::user::{LoginDto, SignUpDto, UserDto, UserUpdateDto};
use crate::types::Wrapper;

pub async fn current() -> Result<Wrapper<UserDto>, Error> {
    request_get::<Wrapper<UserDto>>(
        "/user".to_string()
    ).await
}

pub async fn login(dto: Wrapper<LoginDto>) -> Result<Wrapper<UserDto>, Error> {
    request_post::<Wrapper<LoginDto>, Wrapper<UserDto>>(
        "/user/login".to_string(),
        dto
    ).await
}

pub async fn sign_up(dto: Wrapper<SignUpDto>) -> Result<Wrapper<UserDto>, Error> {
    request_post::<Wrapper<SignUpDto>, Wrapper<UserDto>>(
        "/user/sign_up".to_string(),
        dto
    ).await
}

pub async fn save(dto: Wrapper<UserUpdateDto>) -> Result<Wrapper<UserDto>, Error> {
    request_put::<Wrapper<UserUpdateDto>, Wrapper<UserDto>>(
        "/user".to_string(),
        dto
    ).await
}
