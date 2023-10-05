use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::profile::ProfileDto;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PostDtoWrapper {
    pub data: PostDto,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PostsDto {
    pub posts: Vec<PostDto>,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PostUpdateDto {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tags {
    pub tags: Vec<String>,
}
