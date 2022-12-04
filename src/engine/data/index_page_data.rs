use serde::{Deserialize, Serialize};

use super::post_data::PostData;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexPageData {
    pub posts: Vec<PostData>,
}
