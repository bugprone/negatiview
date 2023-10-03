use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::{json, Value};

use crate::config::AppState;
use crate::dtos::post::NewPostDto;
use crate::dtos::Wrapper;
use crate::middlewares::auth::JwtClaims;
use crate::models::post::{Post, PostFromQuery};

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
                EXISTS
                    (SELECT 1 FROM post_favorites WHERE user_id = $1) "favorited!",
                COALESCE(
                    (SELECT COUNT(*) FROM post_favorites WHERE post_id = posts.id),
                    0
                ) "favorites_count!",
                author.display_name AS author_display_name,
                author.biography AS author_biography,
                author.profile_image_url AS author_profile_image_url,
                EXISTS
                    (SELECT 1 FROM user_follows WHERE followee_user_id = author.id AND follower_user_id = $1) "following_author!"
            FROM posts
            INNER JOIN "users" AS author ON author.id = posts.user_id
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
        Post,
        r#"
            INSERT INTO posts (user_id, slug, title, description, body, tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
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
