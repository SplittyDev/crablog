use serde::Serialize;

use crate::blog::config::BlogMetadataConfig;

#[derive(Debug, Serialize)]
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
