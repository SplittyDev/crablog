use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::blog::Post;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    title: String,
    created_at: NaiveDateTime,
    published: bool,
    html: String,
}

impl TryFrom<&Post> for PostData {
    type Error = anyhow::Error;

    fn try_from(post: &Post) -> Result<Self> {
        let metadata = post.metadata();
        let html = post.to_html_minified()?;
        Ok(Self {
            title: metadata.title.clone(),
            created_at: metadata.created_at,
            published: metadata.published,
            html,
        })
    }
}
