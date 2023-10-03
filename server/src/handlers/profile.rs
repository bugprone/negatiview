use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::{json, Value};

use crate::config::AppState;
use crate::dtos::profile::ProfileDto;
use crate::middlewares::auth::JwtClaims;
use crate::models::user::User;

pub async fn get_user_profile(
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Path(display_name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = jwt_claims.user;
    let profile = sqlx::query_as!(
        ProfileDto,
        r#"
        SELECT display_name, biography, profile_image_url,
            EXISTS (
                SELECT 1 FROM user_follows
                WHERE followee_user_id = "users".id AND follower_user_id = $2
            ) "following!"
            FROM "users"
            WHERE display_name = $1
        "#,
        display_name,
        user.id
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while fetching profile: {err}")
                })),
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Profile found",
        "data": profile
    });

    Ok(Json(json_response))
}

pub async fn follow_user(
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Path(display_name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = jwt_claims.user;

    let followee = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE display_name = $1",
        display_name
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while fetching user: {err}")
                })),
            )
        })?;

    sqlx::query!(
        "INSERT INTO user_follows(follower_user_id, followee_user_id) VALUES($1, $2) ON CONFLICT DO NOTHING",
        user.id,
        followee.id,
    )
        .execute(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while following user: {err}")
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "User followed",
        "data": ProfileDto {
            display_name: followee.display_name,
            biography: followee.biography,
            profile_image_url: followee.profile_image_url,
            following: true
        }
    });

    Ok(Json(json_response))
}

pub async fn unfollow_user(
    Extension(jwt_claims): Extension<JwtClaims>,
    State(data): State<Arc<AppState>>,
    Path(display_name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = jwt_claims.user;

    let followee = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE display_name = $1",
        display_name
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while fetching user: {err}")
                })),
            )
        })?;

    sqlx::query!(
        "DELETE FROM user_follows WHERE follower_user_id = $1 AND followee_user_id = $2",
        user.id,
        followee.id,
    )
        .execute(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while unfollowing user: {err}")
                }))
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "User unfollowed",
        "data": ProfileDto {
            display_name: followee.display_name,
            biography: followee.biography,
            profile_image_url: followee.profile_image_url,
            following: false
        }
    });

    Ok(Json(json_response))
}
