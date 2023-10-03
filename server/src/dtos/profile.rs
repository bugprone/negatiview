use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileDto {
    pub display_name: String,
    pub biography: Option<String>,
    pub profile_image_url: Option<String>,
    pub following: bool,
}

