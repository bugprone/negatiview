use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub content: String,
}
