use crate::middlewares::error::Error;
use crate::middlewares::request::{request_delete, request_get, request_post};
use crate::types::profile::ProfileDtoWrapper;

pub async fn get(display_name: String) -> Result<ProfileDtoWrapper, Error> {
    request_get::<ProfileDtoWrapper>(format!("/profile/{}", display_name)).await
}

pub async fn follow(display_name: String) -> Result<ProfileDtoWrapper, Error> {
    request_post::<(), ProfileDtoWrapper>(format!("/profile/{}/follow", display_name),()).await
}

pub async fn unfollow(display_name: String) -> Result<ProfileDtoWrapper, Error> {
    request_delete::<ProfileDtoWrapper>(format!("/profile/{}/follow", display_name)).await
}
