use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::dtos::profile::ProfileDto;

#[derive(Deserialize)]
pub struct NewPostDto {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct PostDto {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<String>,
    pub favorited: bool,
    pub favorites_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: ProfileDto,
}
