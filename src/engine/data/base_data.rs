use serde::{Deserialize, Serialize};

use super::PageMetaData;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BaseDataBuilder {
    #[serde(rename = "meta")]
    metadata: Option<PageMetaData>,
    content: Option<String>,
}

impl BaseDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_metadata(mut self, metadata: PageMetaData) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
}