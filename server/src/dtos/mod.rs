use serde::{Deserialize, Serialize};

pub mod post;
pub mod user;
pub mod profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wrapper<T> {
    pub data: T,
}
