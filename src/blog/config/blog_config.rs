use serde::{Deserialize, Serialize};

use crate::engine::BuildEnvironment;

use super::{BlogMetadataConfig, BlogThemeConfig};

/// Configuration for a blog project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogConfig {
    pub name: String,
    pub base_url: Option<String>,
    #[serde(rename = "theme")]
    pub theme_config: BlogThemeConfig,
    #[serde(rename = "metadata")]
    pub meta: BlogMetadataConfig,
}

impl BlogConfig {
    pub fn base_url(&self, env: BuildEnvironment) -> String {
        let local_base_url = std::env::current_dir()
            .map(|path| path.join("build").to_string_lossy().to_string())
            .unwrap_or_else(|_| "./build".to_string());
        match env {
            BuildEnvironment::Development => local_base_url,
            BuildEnvironment::Production => {
                self.base_url.clone().unwrap_or_else(|| "/".to_string())
            }
        }
    }
}

impl Default for BlogConfig {
    fn default() -> Self {
        Self {
            name: "My blog".into(),
            base_url: None,
            meta: Default::default(),
            theme_config: Default::default(),
        }
    }
}
