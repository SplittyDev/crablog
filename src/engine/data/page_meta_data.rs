use serde::{Deserialize, Serialize};

use crate::config::BlogMetadataConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageMetaData {
    title: String,
    description: String,
    keywords: String,
}

impl From<BlogMetadataConfig> for PageMetaData {
    fn from(config: BlogMetadataConfig) -> Self {
        Self {
            title: config.title,
            description: config.description,
            keywords: config.keywords.join(", "),
        }
    }
}
