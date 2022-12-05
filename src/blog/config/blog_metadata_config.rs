use serde::{Deserialize, Serialize};

/// Configuration for blog metadata.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BlogMetadataConfig {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub keywords: Vec<String>,
}
