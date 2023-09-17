use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub google_id: Option<String>,
    pub email: String,
    pub display_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct SignUpRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub user_id: Option<i32>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct NewPostRequest {
    pub title: String,
    pub content: String,
}
