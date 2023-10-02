use crate::middlewares::error::Error;
use crate::middlewares::pagination::limit;
use crate::middlewares::request::{request_delete, request_get};
use crate::types::post::{PostDtoWrapper, Posts};

pub async fn all(page: u32) -> Result<Posts, Error> {
    request_get::<Posts>(format!("/posts?{}", limit(10, page))).await
}

pub async fn by_author(author: String, page: u32) -> Result<Posts, Error> {
    request_get::<Posts>(format!("/posts?author={}&{}", author, limit(10, page))).await
}

pub async fn by_tag(tag: String, page: u32) -> Result<Posts, Error> {
    request_get::<Posts>(format!("/posts?tag={}&{}", tag, limit(10, page))).await
}

pub async fn feed() -> Result<Posts, Error> {
    request_get::<Posts>(format!("/posts/feed?{}", limit(10, 0))).await
}

pub async fn favorited_by(author: String, page: u32) -> Result<Posts, Error> {
    request_get::<Posts>(format!("/posts?favorited={}&{}", author, limit(10, page))).await
}

pub async fn favorite(slug: String) -> Result<PostDtoWrapper, Error> {
    request_get::<PostDtoWrapper>(format!("/posts/{}/favorite", slug)).await
}

pub async fn unfavorite(slug: String) -> Result<PostDtoWrapper, Error> {
    request_delete::<PostDtoWrapper>(format!("/posts/{}/favorite", slug)).await
}
