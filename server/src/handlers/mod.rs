use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

pub mod user;
pub mod post;
pub mod profile;
pub mod tag;

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "negatiview server is working!";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
