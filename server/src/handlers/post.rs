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
use crate::middlewares::auth::JwtClaims;
use crate::models::post::{Post, PostFromQuery};

#[derive(Deserialize, Default)]
pub struct PostQuery {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn get_post(
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = &jwt_claims.user;
    let post = sqlx::query_as!(
        PostFromQuery,
        r#"
            SELECT
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
            WHERE slug = $2
        "#,
        user.id,
        slug,
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
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    query: Query<PostQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = &jwt_claims.user;
    let posts: Vec<PostDto> = sqlx::query_as!(
        PostFromQuery,
        r#"
            SELECT
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
        user.id,
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
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    query: Query<PostQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = &jwt_claims.user;
    let posts: Vec<PostDto> = sqlx::query_as!(
        PostFromQuery,
        r#"
            SELECT
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
        user.id,
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
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Wrapper<NewPostDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = &jwt_claims.user;
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
        user.id,
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

pub async fn delete_post(
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let result = sqlx::query!(
        r#"
            WITH the_post AS (
                DELETE FROM posts WHERE slug = $1 AND user_id = $2
                RETURNING 1
            )
            SELECT
                EXISTS (SELECT 1 FROM posts WHERE slug = $1) "existed!",
                EXISTS (SELECT 1 FROM the_post) "deleted!"
        "#,
        slug,
        jwt_claims.user.id
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
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = &jwt_claims.user;
    let post = sqlx::query_as!(
        Post,
        r#"
            WITH the_post AS (
                SELECT * FROM posts WHERE slug = $1
            ),
            favorite AS (
                INSERT INTO post_favorites (user_id, post_id)
                SELECT $2, id FROM the_post
                ON CONFLICT DO NOTHING
            )
            SELECT * FROM the_post
        "#,
        slug,
        user.id
    )
        .fetch_optional(&data.db)
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
        "data": post
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn unfavorite_post(
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = &jwt_claims.user;
    let post = sqlx::query_as!(
        Post,
        r#"
            WITH the_post AS (
                SELECT * FROM posts WHERE slug = $1
            ),
            unfavorite AS (
                DELETE FROM post_favorites
                WHERE post_id = (SELECT id FROM the_post) AND user_id = $2
            )
            SELECT * FROM the_post
        "#,
        slug,
        user.id
    )
        .fetch_optional(&data.db)
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
        "data": post
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
