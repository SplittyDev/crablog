use serde::Serialize;

use super::post_data::PostData;

#[derive(Debug, Serialize)]
pub struct IndexPageData {
    pub posts: Vec<PostData>,
}
