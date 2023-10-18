use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures::{TryFutureExt, TryStreamExt};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::config::AppState;
use crate::dtos::post::*;
use crate::dtos::Wrapper;
use crate::middlewares::auth::AuthUserClaims;
use crate::models::post::PostFromQuery;

#[derive(Deserialize, Default)]
pub struct PostQuery {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn get_post(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let post = sqlx::query_as!(
        PostFromQuery,
        r#"
            SELECT
                posts.id,
                slug,
                title,
                description,
                body,
                tags,
                posts.created_at,
                posts.updated_at,
                EXISTS (SELECT 1 FROM post_favorites WHERE user_id = $1) "favorited!",
                COALESCE ( (SELECT COUNT(*) FROM post_favorites WHERE post_id = posts.id), 0) "favorites_count!",
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                EXISTS (SELECT 1 FROM user_follows WHERE followee_user_id = author.id AND follower_user_id = $1) "following_author!"
            FROM posts
            INNER JOIN users AS author ON author.id = posts.user_id
            WHERE posts.id = $2
        "#,
        user_id,
        post_id,
    )
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

    let json_response = json!({
        "status": "success",
        "message": "Post fetched",
        "data": post.into_post_dto()
    });

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn post_list(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    query: Query<PostQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let posts: Vec<PostDto> = sqlx::query_as!(
        PostFromQuery,
        r#"
            SELECT
                posts.id,
                slug,
                title,
                description,
                body,
                tags,
                posts.created_at,
                posts.updated_at,
                EXISTS (SELECT 1 FROM post_favorites WHERE post_id = posts.id AND user_id = $1) "favorited!",
                COALESCE ( (SELECT COUNT(*) FROM post_favorites WHERE post_id = posts.id), 0) "favorites_count!",
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                EXISTS ( SELECT 1 FROM user_follows WHERE followee_user_id = author.id AND follower_user_id = $1) "following_author!"
            FROM posts
            INNER JOIN users AS author ON author.id = posts.user_id
            WHERE ( $2::TEXT IS NULL OR tags @> array[$2] )
                AND ( $3::TEXT IS NULL OR author.display_name = $3 )
                AND (
                    $4::TEXT IS NULL OR EXISTS (
                        SELECT 1 FROM users
                        INNER JOIN post_favorites ON users.id = post_favorites.user_id
                        WHERE display_name = $4 AND posts.id = post_favorites.post_id
                    )
                )
            ORDER BY posts.created_at DESC
            LIMIT $5
            OFFSET $6
        "#,
        user_id,
        query.tag,
        query.author,
        query.favorited,
        query.limit.unwrap_or(10),
        query.offset.unwrap_or(0),
    )
        .fetch(&data.db)
        .map_ok(|post| post.into_post_dto())
        .try_collect()
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to get posts: {err}"),
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Posts fetched",
        "data": PostsDto {
            count: posts.len(),
            posts,
        }
    });

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn feed_list(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    query: Query<PostQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let posts: Vec<PostDto> = sqlx::query_as!(
        PostFromQuery,
        r#"
            SELECT
                posts.id,
                slug,
                title,
                description,
                body,
                tags,
                posts.created_at,
                posts.updated_at,
                EXISTS (SELECT 1 FROM post_favorites WHERE user_id = $1) "favorited!",
                COALESCE ( (SELECT COUNT(*) FROM post_favorites WHERE post_id = posts.id), 0) "favorites_count!",
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                TRUE "following_author!"
            FROM user_follows
            INNER JOIN posts ON followee_user_id = posts.user_id
            INNER JOIN users AS author ON author.id = user_id
            WHERE follower_user_id = $1
            LIMIT $2
            OFFSET $3
        "#,
        user_id,
        query.limit.unwrap_or(10),
        query.offset.unwrap_or(0),
    )
        .fetch(&data.db)
        .map_ok(|post| post.into_post_dto())
        .try_collect()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to get posts: {err}"),
                }))
            )
        })
        .await?;

    let json_response = json!({
        "status": "success",
        "message": "Posts fetched",
        "data": PostsDto {
            count: posts.len(),
            posts,
        }
    });

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn new_post(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Wrapper<NewPostDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let mut dto = body.data;
    let slug = slugify(dto.title.as_str());
    dto.tags.sort();

    let post = sqlx::query_as!(
        PostFromQuery,
        r#"
            WITH the_post AS (
                INSERT INTO posts (user_id, slug, title, description, body, tags)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING
                    id,
                    slug,
                    title,
                    description,
                    body,
                    tags,
                    created_at,
                    updated_at
            )
            SELECT
                the_post.*,
                FALSE "favorited!",
                0::INT "favorites_count!",
                display_name AS author_display_name,
                biography AS author_biography,
                profile_image_url AS author_profile_image_url,
                FALSE "following_author!"
            FROM the_post
            INNER JOIN users ON users.id = $1
        "#,
        user_id,
        slug,
        dto.title,
        dto.description,
        dto.body,
        &dto.tags[..]
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to create post: {err}"),
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Post created",
        "data": post.into_post_dto()
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn update_post(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<uuid::Uuid>,
    Json(body): Json<Wrapper<UpdatePostDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();

    let post = sqlx::query!(
        "SELECT id, user_id FROM posts WHERE id = $1",
        post_id
    )
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

    if post.user_id != *user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "status": "fail",
                "message": "You are not allowed to edit this post",
            }))
        ))
    }

    let slug = body.data.title.as_deref().map(slugify);

    let post = sqlx::query_as!(
        PostFromQuery,
        r#"
            WITH the_post AS (
                UPDATE posts
                SET
                    slug = COALESCE($1, slug),
                    title = COALESCE($2, title),
                    description = COALESCE($3, description),
                    body = COALESCE($4, body),
                    tags = COALESCE($5, tags)
                WHERE id = $6
                RETURNING
                    id,
                    slug,
                    title,
                    description,
                    body,
                    tags,
                    created_at,
                    updated_at
            )
            SELECT
                the_post.*,
                EXISTS (SELECT 1 FROM post_favorites WHERE user_id = $7) "favorited!",
                COALESCE ( (SELECT COUNT(*) FROM post_favorites WHERE post_id = the_post.id), 0) "favorites_count!",
                display_name AS author_display_name,
                biography AS author_biography,
                profile_image_url AS author_profile_image_url,
                FALSE "following_author!"
            FROM the_post
            INNER JOIN users ON users.id = $7
        "#,
        slug,
        body.data.title,
        body.data.description,
        body.data.body,
        body.data.tags.as_ref().map(|tags| &tags[..]),
        post_id,
        user_id)
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to update post: {err}"),
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Post updated",
        "data": post.into_post_dto()
    });

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn delete_post(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let result = sqlx::query!(
        r#"
            WITH the_post AS (
                DELETE FROM posts WHERE posts.id = $1 AND user_id = $2
                RETURNING 1
            )
            SELECT
                EXISTS (SELECT 1 FROM posts WHERE posts.id = $1) "existed!",
                EXISTS (SELECT 1 FROM the_post) "deleted!"
        "#,
        post_id,
        user_id
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to delete post: {err}"),
                }))
            )
        })?;

    if result.deleted {
        Ok((StatusCode::OK, Json(json!({
            "status": "success",
            "message": "Post deleted",
            "data": post_id
        }))))
    } else if result.existed {
        Err((StatusCode::FORBIDDEN, Json(json!({
            "status": "fail",
            "message": "Post is not yours",
        }))))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({
            "status": "fail",
            "message": "Post not found",
        }))))
    }
}

pub async fn favorite_post(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let post = sqlx::query_as!(
        PostFromQuery,
        r#"
            WITH the_post AS (
                SELECT * FROM posts WHERE id = $2
            ),
            favorite AS (
                INSERT INTO post_favorites (user_id, post_id)
                SELECT $1, id FROM the_post
                ON CONFLICT DO NOTHING
            )
            SELECT
                the_post.id,
                slug,
                title,
                description,
                body,
                tags,
                the_post.created_at,
                the_post.updated_at,
                EXISTS (SELECT 1 FROM post_favorites WHERE user_id = $1) "favorited!",
                COALESCE ( (SELECT COUNT(*) FROM post_favorites WHERE post_id = the_post.id), 0) "favorites_count!",
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                EXISTS (SELECT 1 FROM user_follows WHERE followee_user_id = author.id AND follower_user_id = $1) "following_author!"
            FROM the_post
            INNER JOIN users AS author ON author.id = the_post.user_id
            WHERE the_post.id = $2
        "#,
        user_id,
        post_id,
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to favorite post: {err}"),
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Post favorited",
        "data": post.into_post_dto()
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn unfavorite_post(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let post = sqlx::query_as!(
        PostFromQuery,
        r#"
            WITH the_post AS (
                SELECT * FROM posts WHERE id = $2
            ),
            unfavorite AS (
                DELETE FROM post_favorites
                WHERE post_id = (SELECT id FROM the_post) AND user_id = $1
            )
            SELECT
                the_post.id,
                slug,
                title,
                description,
                body,
                tags,
                the_post.created_at,
                the_post.updated_at,
                EXISTS (SELECT 1 FROM post_favorites WHERE user_id = $1) "favorited!",
                COALESCE ( (SELECT COUNT(*) FROM post_favorites WHERE post_id = the_post.id), 0) "favorites_count!",
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                EXISTS (SELECT 1 FROM user_follows WHERE followee_user_id = author.id AND follower_user_id = $1) "following_author!"
            FROM the_post
            INNER JOIN users AS author ON author.id = the_post.user_id
            WHERE the_post.id = $2
        "#,
        user_id,
        post_id,
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Failed to unfavorite post: {err}"),
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Post unfavorited",
        "data": post.into_post_dto()
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

fn slugify(string: &str) -> String {
    const QUOTE_CHARS: &[char] = &['\'', '"'];

    let slug_parts: Vec<String> = string
        .split(|c: char| !(QUOTE_CHARS.contains(&c) || c.is_alphanumeric()))
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s.replace(QUOTE_CHARS, "");
            s.make_ascii_lowercase();
            s
        })
        .collect();

    slug_parts.join("-")
}
