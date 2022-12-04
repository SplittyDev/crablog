use serde::{Deserialize, Serialize};

use super::post_data::PostData;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostPageData {
    pub base_url: String,
    pub post: Option<PostData>,
}
