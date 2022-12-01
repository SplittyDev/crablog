use serde::{Deserialize, Serialize};

use super::{BlogConfig, ThemeConfig};
use crate::{Blog, Theme};

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
        self.theme_config
            .clone()
            .and_then(|config| Theme::from_config(config).ok())
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
