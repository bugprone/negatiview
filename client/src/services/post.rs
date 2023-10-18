use crate::middlewares::error::Error;
use crate::middlewares::pagination::limit;
use crate::middlewares::request::{request_delete, request_get, request_post, request_put};
use crate::types::post::{PostDto, PostsDto, PostUpdateDto};
use crate::types::Wrapper;

pub async fn all(page: usize) -> Result<Wrapper<PostsDto>, Error> {
    request_get::<Wrapper<PostsDto>>(format!("/posts?{}", limit(10, page))).await
}

pub async fn by_author(author: String, page: usize) -> Result<Wrapper<PostsDto>, Error> {
    request_get::<Wrapper<PostsDto>>(format!("/posts?author={}&{}", author, limit(10, page))).await
}

pub async fn by_tag(tag: String, page: usize) -> Result<Wrapper<PostsDto>, Error> {
    request_get::<Wrapper<PostsDto>>(format!("/posts?tag={}&{}", tag, limit(10, page))).await
}

pub async fn feed() -> Result<Wrapper<PostsDto>, Error> {
    request_get::<Wrapper<PostsDto>>(format!("/posts/feed?{}", limit(10, 0))).await
}

pub async fn favorited_by(author: String, page: usize) -> Result<Wrapper<PostsDto>, Error> {
    request_get::<Wrapper<PostsDto>>(format!("/posts?favorited={}&{}", author, limit(10, page))).await
}

pub async fn favorite(post_id: String) -> Result<Wrapper<PostDto>, Error> {
    request_post::<(), Wrapper<PostDto>>(format!("/posts/{}/favorite", post_id), ()).await
}

pub async fn unfavorite(post_id: String) -> Result<Wrapper<PostDto>, Error> {
    request_delete::<Wrapper<PostDto>>(format!("/posts/{}/favorite", post_id)).await
}

pub async fn get(post_id: String) -> Result<Wrapper<PostDto>, Error> {
    request_get::<Wrapper<PostDto>>(format!("/posts/{}", post_id)).await
}

pub async fn create(post: Wrapper<PostUpdateDto>) -> Result<Wrapper<PostDto>, Error> {
    request_post::<Wrapper<PostUpdateDto>, Wrapper<PostDto>>(
        "/posts".to_string(),
        post,
    )
        .await
}

pub async fn update(post_id: String, post: Wrapper<PostUpdateDto>) -> Result<Wrapper<PostDto>, Error> {
    request_put::<Wrapper<PostUpdateDto>, Wrapper<PostDto>>(
        format!("/posts/{}", post_id),
        post,
    )
        .await
}

pub async fn del(post_id: String) -> Result<Wrapper<String>, Error> {
    request_delete::<Wrapper<String>>(format!("/posts/{}", post_id)).await
}
