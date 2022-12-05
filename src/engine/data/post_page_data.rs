use serde::Serialize;

use super::post_data::PostData;

#[derive(Debug, Serialize)]
pub struct PostPageData {
    pub post: Option<PostData>,
}
