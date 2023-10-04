use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::config::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagsDto {
    pub tags: Vec<String>,
}

pub async fn get_tags(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)>  {
    let tags = sqlx::query_scalar!(
        r#"
            SELECT DISTINCT tag "tag!"
            FROM posts, unnest(tags) as tags(tag)
            ORDER BY tag
        "#
    )
        .fetch_all(&data.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("error fetching tags: {err}")
                })),
            )
        })?;

    let json_response = json!({
        "status": "success",
        "message": "Tags fetched successfully",
        "data": TagsDto {
            tags
        }
    });

    Ok(Json(json_response))
}
