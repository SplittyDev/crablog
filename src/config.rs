use serde::{Deserialize, Serialize};

use crate::{
    blog::Blog,
    theme::{Theme, ThemeSource},
};

pub const CONFIG_FILENAME: &str = "Crablog.toml";

/// Minimal subset of valid project configuration for all project kinds.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommonProjectConfig {
    #[serde(rename = "blog")]
    blog_config: Option<BlogConfig>,
    #[serde(rename = "theme")]
    theme_config: Option<ThemeConfig>,
}

impl CommonProjectConfig {
    pub fn to_blog(&self) -> Option<Blog> {
        self.blog_config.clone().map(Blog::from_config)
    }

    pub fn to_theme(&self) -> Option<Theme> {
        self.theme_config.clone().map(Theme::from_config)
    }
}

impl From<BlogConfig> for CommonProjectConfig {
    fn from(config: BlogConfig) -> Self {
        Self {
            blog_config: Some(config),
            ..Default::default()
        }
    }
}

impl From<ThemeConfig> for CommonProjectConfig {
    fn from(config: ThemeConfig) -> Self {
        Self {
            theme_config: Some(config),
            ..Default::default()
        }
    }
}

impl From<(BlogConfig, ThemeConfig)> for CommonProjectConfig {
    fn from(configs: (BlogConfig, ThemeConfig)) -> Self {
        Self {
            blog_config: Some(configs.0),
            theme_config: Some(configs.1),
        }
    }
}

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

/// Configuration for a theme project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub author: Option<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "My theme".into(),
            author: Some("".into()),
        }
    }
}
