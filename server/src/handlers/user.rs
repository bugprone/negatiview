use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use axum::{Extension, Json};
use axum::extract::State;
use axum::http::{HeaderMap, Response, StatusCode};
use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, SameSite};
use rand_core::OsRng;
use redis::aio::Connection;
use redis::AsyncCommands;
use serde_json::{json, Value};

use crate::config::AppState;
use crate::dtos::user::*;
use crate::dtos::Wrapper;
use crate::middlewares::auth::AuthUserClaims;
use crate::middlewares::token;
use crate::middlewares::token::TokenData;
use crate::models::user::User;

pub async fn me(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    if let Some(user) = auth_user_claims.user {
        let access_token_uuid = auth_user_claims.access_token_uuid.unwrap_or_default();
        let access_token = find_access_token_in_redis(&data, access_token_uuid).await?;

        let json_response = json!({
            "status": "success",
            "data": UserDto {
                email: user.email,
                display_name: user.display_name,
                access_token,
                biography: user.biography.unwrap_or_default(),
                profile_image_url: user.profile_image_url.unwrap_or_default(),
            }
        });

        Ok(Json(json_response))
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "fail",
                "message": "Unauthorized"
            })),
        ));
    }
}

pub async fn update_me(
    Extension(auth_user_claims): Extension<AuthUserClaims>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Wrapper<UserUpdateDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_id = &auth_user_claims.user_id().unwrap_or_default();
    let req = body.data;

    let query = match req.password {
        Some(password) => {
            let hashed_password = get_hashed_password(&password)?;
            sqlx::query_as::<_, User>(
                r#"
                UPDATE users
                SET email = $1, display_name = $2, biography = $3, profile_image_url = $4, password = $5
                WHERE id = $6
                RETURNING *
                "#,
            )
                .bind(req.email)
                .bind(req.display_name)
                .bind(req.biography)
                .bind(req.profile_image_url)
                .bind(hashed_password)
                .bind(user_id)
        }
        None => sqlx::query_as::<_, User>(
            r#"
                UPDATE users
                SET email = $1, display_name = $2, biography = $3, profile_image_url = $4
                WHERE id = $5
                RETURNING *
                "#,
        )
            .bind(req.email)
            .bind(req.display_name)
            .bind(req.biography)
            .bind(req.profile_image_url)
            .bind(user_id),
    };

    let user = query.fetch_one(&data.db).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "fail",
                "message": format!("Something bad happened while updating user: {err}")
            })),
        )
    })?;

    let mut access_token = String::default();

    if let Some(access_token_uuid) = auth_user_claims.access_token_uuid {
        access_token = find_access_token_in_redis(&data, access_token_uuid).await?
    }

    let json_response = json!({
        "status": "success",
        "message": "User updated successfully",
        "data": UserDto {
            email: user.email,
            display_name: user.display_name,
            access_token,
            biography: user.biography.unwrap_or_default(),
            profile_image_url: user.profile_image_url.unwrap_or_default(),
        }
    });

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn sign_up(
    State(data): State<Arc<AppState>>,
    Json(body): Json<Wrapper<SignUpDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let req = body.data;
    let hashed_password = get_hashed_password(&req.password)?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password, display_name) VALUES ($1, $2, $3) returning *",
        req.email,
        hashed_password,
        req.display_name
    )
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while creating user: {err}")
                })),
            )
        })?;

    let access_token_data = issue_access_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;
    let refresh_token_data = issue_access_token(
        user.id,
        data.env.refresh_token_max_age,
        data.env.refresh_token_private_key.to_owned(),
    )?;

    save_access_token_to_redis(&data, &access_token_data, data.env.access_token_max_age).await?;
    save_access_token_to_redis(&data, &refresh_token_data, data.env.refresh_token_max_age).await?;

    let headers = set_cookies(data, &access_token_data, &refresh_token_data);

    let json_response = json!({
        "status": "success",
        "message": "User created",
        "data": UserDto {
            email: user.email,
            display_name: user.display_name,
            access_token: access_token_data.access_token.unwrap(),
            biography: user.biography.unwrap_or_default(),
            profile_image_url: user.profile_image_url.unwrap_or_default(),
        }
    });

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(json_response.to_string())
        .unwrap();

    response.headers_mut().extend(headers);

    Ok(response)
}

fn set_cookies(
    data: Arc<AppState>,
    access_token_data: &TokenData,
    refresh_token_data: &TokenData,
) -> HeaderMap {
    let access_cookie = Cookie::build(
        "access_token",
        access_token_data.access_token.clone().unwrap_or_default(),
    )
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let refresh_cookie = Cookie::build(
        "refresh_token",
        refresh_token_data.access_token.clone().unwrap_or_default(),
    )
        .path("/")
        .max_age(time::Duration::minutes(data.env.refresh_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let logged_in_cookie = Cookie::build("logged_in", "true")
        .path("/")
        .max_age(time::Duration::minutes(data.env.refresh_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false)
        .finish();

    let mut headers = HeaderMap::new();
    headers.append("Set-Cookie", access_cookie.to_string().parse().unwrap());
    headers.append("Set-Cookie", refresh_cookie.to_string().parse().unwrap());
    headers.append("Set-Cookie", logged_in_cookie.to_string().parse().unwrap());
    headers
}

fn get_hashed_password(password: &str) -> Result<String, (StatusCode, Json<Value>)> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while hashing password: {err}")
                })),
            )
        })
        .map(|hash| hash.to_string())?;
    Ok(hashed_password)
}

pub async fn login(
    State(data): State<Arc<AppState>>,
    Json(body): Json<Wrapper<LoginDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let req = body.data;
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", req.email)
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

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "fail",
                "message": "Login failed: Invalid credentials"
            })),
        ));
    }

    let access_token_data = issue_access_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;
    let refresh_token_data = issue_access_token(
        user.id,
        data.env.refresh_token_max_age,
        data.env.refresh_token_private_key.to_owned(),
    )?;

    save_access_token_to_redis(&data, &access_token_data, data.env.access_token_max_age).await?;
    save_access_token_to_redis(&data, &refresh_token_data, data.env.refresh_token_max_age).await?;

    let headers = set_cookies(data, &access_token_data, &refresh_token_data);
    let json_response = json!({
        "status": "success",
        "message": "Login successful",
        "data": UserDto {
            email: user.email,
            display_name: user.display_name,
            access_token: access_token_data.access_token.unwrap(),
            biography: user.biography.unwrap_or_default(),
            profile_image_url: user.profile_image_url.unwrap_or_default(),
        }
    });

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(json_response.to_string())
        .unwrap();

    response.headers_mut().extend(headers);

    Ok(response)
}

fn issue_access_token(
    user_id: uuid::Uuid,
    max_age: i64,
    private_key: String,
) -> Result<TokenData, (StatusCode, Json<Value>)> {
    token::generate_token(user_id, max_age, private_key).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "fail",
                "message": format!("Something bad happened while generating token: {err}")
            })),
        )
    })
}

async fn save_access_token_to_redis(
    data: &Arc<AppState>,
    token_data: &TokenData,
    max_age: i64,
) -> Result<(), (StatusCode, Json<Value>)> {
    let mut redis_client = get_redis_client(data).await?;

    redis_client
        .set_ex(
            token_data.token_uuid.to_string(),
            token_data.access_token.as_ref(),
            (max_age * 60) as usize,
        )
        .await
        .map_err(|err| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while saving token to redis: {err}")
                })),
            )
        })?;

    Ok(())
}

async fn find_access_token_in_redis(
    data: &Arc<AppState>,
    access_token_uuid: uuid::Uuid,
) -> Result<String, (StatusCode, Json<Value>)> {
    let mut redis_client = get_redis_client(data).await?;

    let access_token = redis_client
        .get::<_, String>(access_token_uuid.to_string())
        .await
        .map_err(|err| {
            (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
                "status": "fail",
                "message": format!("Something bad happened while fetching token from redis: {err}")
            })))
        })?;

    Ok(access_token)
}

async fn get_redis_client(data: &Arc<AppState>) -> Result<Connection, (StatusCode, Json<Value>)> {
    let redis_client = data
        .redis_client
        .get_async_connection()
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Something bad happened while connecting to redis: {err}")
                })),
            )
        })?;
    Ok(redis_client)
}
