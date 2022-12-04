use serde::Serialize;

use super::PageMetaData;

#[derive(Debug, Default, Serialize)]
pub struct BaseData<T: Serialize> {
    #[serde(flatten)]
    base_data: BaseDataBuilder,
    #[serde(flatten)]
    child_data: T,
}

impl<T> BaseData<T>
where
    T: Serialize,
{
    pub fn into_base_data(self) -> BaseDataBuilder {
        self.base_data
    }
}

#[derive(Debug, Default, Serialize)]
pub struct BaseDataBuilder {
    base_url: String,
    #[serde(rename = "meta")]
    metadata: Option<PageMetaData>,
    content: Option<String>,
}

impl BaseDataBuilder {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            ..Default::default()
        }
    }

    pub fn with_metadata(mut self, metadata: PageMetaData) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

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
