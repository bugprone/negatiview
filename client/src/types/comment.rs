use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::profile::ProfileDto;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommentDto {
    pub id: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: ProfileDto,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NewCommentDto {
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentsDto {
    pub comments: Vec<CommentDto>,
    pub count: usize,
}
