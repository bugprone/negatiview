use axum::extract::{Query, State};
use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde_json::{json, Value};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::config::AppState;
use crate::dto::post::NewPostRequest;
use crate::dto::user::*;
use crate::model::post::PostModel;
use crate::model::user::{TokenClaims, UserModel};
use crate::schema::FilterOptions;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "negatiview server is working!";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn me_handler(
    Extension(user): Extension<UserModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let json_response = json!({
        "status": "success",
        "data": UserDto {
            email: user.email,
            display_name: user.display_name.unwrap_or("User".to_string()),
            token: "token".to_string()
        }
    });

    Ok(Json(json_response))
}


pub async fn user_list_handler(
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

pub async fn new_user_handler(
    State(data): State<Arc<AppState>>,
    Json(user): Json<SignUpDto>,
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

    let json_response = json!({
        "status": "success".to_string(),
        "message": "User created".to_string(),
        "data": UserDto {
            email: user.email,
            display_name: user.display_name,
            token: "token".to_string()
        }
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(user): Json<LoginDtoWrapper>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE email = $1",
        user.data.email
    )
        .fetch_optional(&data.db)
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "error",
                "message": format!("Login failed: {err}")
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?
        .ok_or_else(|| {
            let error_response = json!({
                "status": "error",
                "message": "Login failed: Invalid credentials"
            });
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.jwt_expires_in)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
        .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::minutes(data.env.jwt_expires_in))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({
        "status": "success",
        "message": "Login successful",
        "data": UserDto {
            email: user.email,
            display_name: user.display_name.unwrap_or("User".to_string()),
            token: token,
        },
    }).to_string());
    response.headers_mut().insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn post_list_handler(
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

pub async fn new_post_handler(
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
