use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures::TryStreamExt;
use serde_json::{json, Value};

use crate::config::AppState;
use crate::dtos::comment::*;
use crate::dtos::Wrapper;
use crate::middlewares::auth::AuthUserClaims;
use crate::models::comment::CommentFromQuery;

pub async fn get_comments(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();

    let post_id = sqlx::query_scalar!("SELECT id FROM posts WHERE id = $1", post_id)
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to get post: {err}"),
                }))
            )
        })?;

    let comments: Vec<CommentDto> = sqlx::query_as!(
        CommentFromQuery,
        r#"
            SELECT
                comments.id,
                body,
                comments.created_at,
                comments.updated_at,
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                EXISTS (SELECT 1 FROM user_follows WHERE followee_user_id = author.id AND follower_user_id = $1) "following_author!"
            FROM comments
            INNER JOIN users AS author ON author.id = comments.user_id
            WHERE post_id = $2
            ORDER BY created_at
        "#,
        user_id,
        post_id,
    )
        .fetch(&data.db)
        .map_ok(|comment| comment.into_comment_dto())
        .try_collect()
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to get comments: {err}"),
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Comments fetched",
        "data": CommentsDto {
            count: comments.len(),
            comments
        }
    });

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn new_comment(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<uuid::Uuid>,
    Json(body): Json<Wrapper<NewCommentDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let comment = sqlx::query_as!(
        CommentFromQuery,
        r#"
            WITH the_comment AS (
                INSERT INTO comments (post_id, user_id, body)
                SELECT id, $1, $2
                FROM posts
                WHERE posts.id = $3
                RETURNING id, body, created_at, updated_at
            )
            SELECT
                the_comment.id,
                body,
                the_comment.created_at,
                the_comment.updated_at,
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                FALSE "following_author!"
            FROM the_comment
            INNER JOIN users AS author ON author.id = $1
        "#,
        user_id,
        body.data.body,
        post_id,
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to create comment: {err}"),
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Comment created",
        "data": comment.into_comment_dto()
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn delete_comment(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path((post_id, comment_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let result = sqlx::query!(
        r#"
            WITH the_comment AS (
                DELETE FROM comments
                WHERE
                    id = $1 AND
                    post_id IN (SELECT id FROM posts WHERE id = $2) AND
                    user_id = $3
                RETURNING 1
            )
            SELECT
                EXISTS (
                    SELECT 1 FROM comments
                    INNER JOIN posts ON posts.id = comments.post_id
                    WHERE comments.id = $1 AND posts.id = $2
                ) "existed!",
                EXISTS (
                    SELECT 1 FROM the_comment
                ) "deleted!"
        "#,
        comment_id,
        post_id,
        user_id,
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to delete comment: {err}"),
                }))
            )
        })?;

    if result.deleted {
        Ok((StatusCode::OK, Json(json!({
            "status": "success",
            "message": "Comment deleted",
            "data": comment_id,
        }))))
    } else if result.existed {
        Err((StatusCode::FORBIDDEN, Json(json!({
            "status": "fail",
            "message": "Comment is not yours",
        }))))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({
            "status": "fail",
            "message": "Comment not found",
        }))))
    }
}
