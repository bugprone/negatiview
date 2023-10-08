use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::dtos::comment::CommentDto;
use crate::dtos::profile::ProfileDto;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub post_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CommentFromQuery {
    pub id: uuid::Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author_display_name: String,
    pub author_biography: Option<String>,
    pub author_profile_image_url: Option<String>,
    pub following_author: bool,
}

impl CommentFromQuery {
    pub fn into_comment_dto(self) -> CommentDto {
        CommentDto {
            id: self.id,
            body: self.body,
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: ProfileDto {
                display_name: self.author_display_name,
                biography: self.author_biography,
                profile_image_url: self.author_profile_image_url,
                following: self.following_author,
            },
        }
    }
}
