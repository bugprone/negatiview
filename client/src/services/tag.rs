use serde::{Deserialize, Serialize};

use crate::middlewares::error::Error;
use crate::middlewares::request::request_get;
use crate::types::Wrapper;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagsDto {
    pub tags: Vec<String>,
}
pub async fn get() -> Result<Wrapper<TagsDto>, Error> {
    request_get::<Wrapper<TagsDto>>("/tags".to_string()).await
}
