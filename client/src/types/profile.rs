use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProfileDto {
    pub display_name: String,
    pub biography: Option<String>,
    pub profile_image_url: Option<String>,
    pub following: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProfileDtoWrapper {
    pub data: ProfileDto,
}
