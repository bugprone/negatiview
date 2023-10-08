use crate::middlewares::error::Error;
use crate::middlewares::request::{request_delete, request_get, request_post};
use crate::types::comment::{CommentDto, CommentsDto, NewCommentDto};
use crate::types::{DeleteWrapper, Wrapper};

pub async fn create(slug: String, comment: Wrapper<NewCommentDto>) -> Result<Wrapper<CommentDto>, Error> {
    request_post::<Wrapper<NewCommentDto>, Wrapper<CommentDto>>(
        format!("/posts/{}/comments", slug),
        comment,
    )
        .await
}

pub async fn delete(slug: String, comment_id: String) -> Result<DeleteWrapper, Error> {
    request_delete::<DeleteWrapper>(format!("/posts/{}/comments/{}", slug, comment_id))
        .await
}

pub async fn get(slug: String) -> Result<Wrapper<CommentsDto>, Error> {
    request_get::<Wrapper<CommentsDto>>(format!("/posts/{}/comments", slug))
        .await
}
