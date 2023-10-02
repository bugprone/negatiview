use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::profile::Profile;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub starred: bool,
    pub starred_count: u32,
    pub author: Profile,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PostWrapper {
    pub data: Post,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Posts {
    pub posts: Vec<Post>,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostUpdate {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostUpdateWrapper {
    pub data: PostUpdate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagList {
    pub tags: Vec<String>,
}
