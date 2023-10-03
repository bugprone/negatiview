use crate::middlewares::error::Error;
use crate::middlewares::pagination::limit;
use crate::middlewares::request::{request_delete, request_get, request_post, request_put};
use crate::types::post::{PostDto, Posts, PostUpdateDto};
use crate::types::Wrapper;

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

pub async fn favorite(slug: String) -> Result<Wrapper<PostDto>, Error> {
    request_get::<Wrapper<PostDto>>(format!("/posts/{}/favorite", slug)).await
}

pub async fn unfavorite(slug: String) -> Result<Wrapper<PostDto>, Error> {
    request_delete::<Wrapper<PostDto>>(format!("/posts/{}/favorite", slug)).await
}

pub async fn get(slug: String) -> Result<Wrapper<PostDto>, Error> {
    request_get::<Wrapper<PostDto>>(format!("/posts/{}", slug)).await
}

pub async fn create(post: Wrapper<PostUpdateDto>) -> Result<Wrapper<PostDto>, Error> {
    request_post::<Wrapper<PostUpdateDto>, Wrapper<PostDto>>(
        "/posts".to_string(),
        post,
    )
        .await
}

pub async fn update(slug: String, post: Wrapper<PostUpdateDto>) -> Result<Wrapper<PostDto>, Error> {
    request_put::<Wrapper<PostUpdateDto>, Wrapper<PostDto>>(
        format!("/posts/{}", slug),
        post,
    )
        .await
}
