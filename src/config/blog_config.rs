use serde::{Deserialize, Serialize};

use crate::theme::ThemeSource;

/// Configuration for a blog project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogConfig {
    pub name: String,
    #[serde(rename = "theme")]
    #[serde(flatten)]
    pub theme_source: ThemeSource,
    #[serde(rename = "metadata")]
    pub meta: BlogMetadataConfig,
}

impl Default for BlogConfig {
    fn default() -> Self {
        Self {
            name: "My blog".into(),
            meta: Default::default(),
            theme_source: Default::default(),
        }
    }
}

/// Configuration for blog metadata.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BlogMetadataConfig {
    pub title: String,
    pub description: String,
}
