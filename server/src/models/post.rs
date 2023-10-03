use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::dtos::post::PostDto;
use crate::dtos::profile::ProfileDto;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct PostFromQuery {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author_display_name: String,
    pub author_biography: Option<String>,
    pub author_profile_image_url: Option<String>,
    pub following_author: bool,
}

impl PostFromQuery {
    pub fn into_post_dto(self) -> PostDto{
        PostDto {
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            tags: self.tags,
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited: self.favorited,
            favorites_count: self.favorites_count,
            author: ProfileDto {
                display_name: self.author_display_name,
                biography: self.author_biography,
                profile_image_url: self.author_profile_image_url,
                following: self.following_author,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Follow {
    pub post_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
