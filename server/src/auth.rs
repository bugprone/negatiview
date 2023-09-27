use std::sync::Arc;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    Json,
    middleware::Next,
    response::IntoResponse
};
use axum::http::header;
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::{json, Value};

use crate::config::AppState;
use crate::model::user::{TokenClaims, User};

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let token = cookie_jar
        .get("token")
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

    let token = token.ok_or_else(|| {
        (StatusCode::UNAUTHORIZED, Json(json!({
            "status": "fail",
            "message": "Token not found",
        })))
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default()
    )
        .map_err(|_| {
            (StatusCode::UNAUTHORIZED, Json(json!({
                "status": "fail",
                "message": "Invalid token",
            })))
        })?
        .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(json!({
            "status": "fail",
            "message": "Invalid token",
        })))
    })?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|err| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "status": "fail",
                "message": format!("Error fetching user from database: {err}"),
            })))
        })?;

    let user = user.ok_or_else(|| {
        (StatusCode::UNAUTHORIZED, Json(json!({
            "status": "fail",
            "message": "User not found",
        })))
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
