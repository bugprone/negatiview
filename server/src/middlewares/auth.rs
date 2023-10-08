use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    Json,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::config::AppState;
use crate::middlewares::token;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUserClaims {
    pub user: Option<User>,
    pub access_token_uuid: Option<Uuid>,
}

impl AuthUserClaims {
    pub fn user_id(&self) -> Option<Uuid> {
        self.user.as_ref().map(|user| user.id)
    }
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let get_token = cookie_jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    if get_token.is_none() {
        req.extensions_mut().insert(AuthUserClaims {
            user: None,
            access_token_uuid: None,
        });
        return Ok(next.run(req).await)
    }

    let access_token = get_token.unwrap();

    let access_token_data =
        match token::verify_token(data.env.access_token_public_key.to_owned(), &access_token) {
            Ok(data) => data,
            Err(err) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status": "fail",
                        "message": format!("Invalid access token: {err}"),
                    })),
                ))
            }
        };

    let access_token_uuid =
        Uuid::parse_str(&access_token_data.token_uuid.to_string()).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "fail",
                    "message": "Invalid access token",
                })),
            )
        })?;

    let mut redis_client = data
        .redis_client
        .get_async_connection()
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": format!("Error connecting to redis: {err}"),
                })),
            )
        })?;

    let access_token_in_redis = redis_client
        .get::<_, String>(access_token_uuid.clone().to_string())
        .await
        .map_err(|err| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "fail",
                    "message": format!("Error fetching token from redis: {err}"),
                })),
            )
        })?;

    if access_token_in_redis != access_token {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "fail",
                "message": "Invalid access token",
            })),
        ));
    }

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        access_token_data.user_id
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "fail",
                "message": format!("Error fetching user from database: {err}"),
            })),
        )
    })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "fail",
                "message": "User not found",
            })),
        )
    })?;

    req.extensions_mut().insert(AuthUserClaims {
        user: Some(user),
        access_token_uuid: Some(access_token_uuid),
    });
    Ok(next.run(req).await)
}
