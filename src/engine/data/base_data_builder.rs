use std::collections::HashMap;

use serde::Serialize;

use super::{BaseData, PageMetaData};

#[derive(Debug, Default, Serialize)]
pub struct BaseDataBuilder {
    /// Base URL of the blog
    base_url: String,
    /// Blog metadata
    #[serde(rename = "meta")]
    metadata: Option<PageMetaData>,
    /// Enabled theme features
    features: HashMap<String, bool>,
    /// Source code of the current page
    content: Option<String>,
}

impl BaseDataBuilder {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_metadata(mut self, metadata: PageMetaData) -> Self {
        self.metadata = Some(metadata);
        self
    }

    #[must_use]
    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.features = features
            .into_iter()
            .fold(HashMap::new(), |mut map, feature| {
                map.entry(feature).or_insert(true);
                map
            });
        self
    }

    #[must_use]
    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    #[must_use]
    pub fn build<T>(self, data: T) -> BaseData<T>
    where
        T: Serialize,
    {
        BaseData {
            base_data: self,
            child_data: data,
        }
    }
}
