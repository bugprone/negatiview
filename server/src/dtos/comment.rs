use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::dtos::profile::ProfileDto;

#[derive(Serialize, Deserialize)]
pub struct CommentDto {
    pub id: uuid::Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: ProfileDto,
}

#[derive(Deserialize)]
pub struct NewCommentDto {
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentsDto {
    pub comments: Vec<CommentDto>,
    pub count: usize,
}
