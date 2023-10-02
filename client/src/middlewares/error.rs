use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Deserialization Error")]
    DeserializationError,
    #[error("Bad Request")]
    BadRequest,
}
