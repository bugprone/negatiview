use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use serde_json::{json, Value};
use crate::AppState;
use crate::dto::post::NewPostRequest;
use crate::dto::user::*;
use crate::model::post::PostModel;
use crate::model::user::UserModel;

use crate::schema::FilterOptions;

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "negatiview server is working!";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn user_list(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = json!({
            "status": "fail",
            "message": "Something bad happened while fetching all users",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let users = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "users": users
    });

    Ok(Json(json_response))
}

pub async fn new_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<SignUpRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query!(
        "INSERT INTO users (email, first_name, last_name, display_name) VALUES ($1, $2, $3, $4) returning *",
        user.email,
        user.first_name,
        user.last_name,
        user.display_name
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = json!({
            "status": "fail",
            "message": "Something bad happened while creating user",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let json_response = json!(SignUpResponse {
        status: "success".to_string(),
        message: "Login successful".to_string(),
        user_info: UserDto {
            email: user.email,
            username: user.display_name,
            token: "token".to_string()
        }
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn login(
    State(data): State<Arc<AppState>>,
    Json(user): Json<LoginDtoWrapper>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query!(
        "SELECT * FROM users WHERE email = $1",
        user.data.email
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            Ok((
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": "Login successful",
                    "data": UserDto {
                        email: user.email,
                        username: user.display_name.unwrap_or("User".to_string()),
                        token: "token".to_string()
                    }
                })),
            ))
        }
        Err(_) => {
            Ok((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "fail",
                    "message": "Login failed: Invalid credentials",
                })),
            ))
        }
    }
}

pub async fn post_list(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        PostModel,
        "SELECT * FROM posts LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = json!({
            "status": "fail",
            "message": "Something bad happened while fetching all posts",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let posts = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "posts": posts
    });

    Ok(Json(json_response))
}

pub async fn new_post(
    State(data): State<Arc<AppState>>,
    Json(post): Json<NewPostRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query!(
        "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING *",
        post.title,
        post.content
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = json!({
            "status": "fail",
            "message": "Something bad happened while creating post",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let json_response = json!({
        "status": "success",
        "message": "Post created successfully"
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}
