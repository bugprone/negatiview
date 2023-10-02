use crate::middlewares::error::Error;
use crate::middlewares::request::{request_get, request_post, request_put};
use crate::types::user::{LoginDtoWrapper, SignUpDtoWrapper, UserDtoWrapper, UserUpdateDtoWrapper};

pub async fn current() -> Result<UserDtoWrapper, Error> {
    request_get::<UserDtoWrapper>(
        "/user".to_string()
    ).await
}

pub async fn login(dto: LoginDtoWrapper) -> Result<UserDtoWrapper, Error> {
    request_post::<LoginDtoWrapper, UserDtoWrapper>(
        "/user/login".to_string(),
        dto
    ).await
}

pub async fn sign_up(dto: SignUpDtoWrapper) -> Result<UserDtoWrapper, Error> {
    request_post::<SignUpDtoWrapper, UserDtoWrapper>(
        "/user/sign_up".to_string(),
        dto
    ).await
}

pub async fn save(dto: UserUpdateDtoWrapper) -> Result<UserDtoWrapper, Error> {
    request_put::<UserUpdateDtoWrapper, UserDtoWrapper>(
        "/user".to_string(),
        dto
    ).await
}
