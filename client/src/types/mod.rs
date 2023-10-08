use serde::{Deserialize, Serialize};

pub mod comment;
pub mod post;
pub mod profile;
pub mod user;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Wrapper<T> {
    pub data: T,
}
