use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::profile::ProfileDto;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PostDto {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub favorited: bool,
    pub favorites_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: ProfileDto,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Posts {
    pub posts: Vec<PostDto>,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostUpdateDto {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tags {
    pub tags: Vec<String>,
}
