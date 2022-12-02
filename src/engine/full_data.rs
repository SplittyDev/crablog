use serde::{Deserialize, Serialize};

use super::post_data::PostData;

#[derive(Debug, Serialize, Deserialize)]
pub struct FullData {
    pub post: Option<PostData>,
}
