use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{
    blog::{config::BlogConfig, Blog},
    theme::{config::ThemeConfig, Theme},
    traits::TryLoadConfig,
};

/// Minimal subset of valid project configuration for all project kinds.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommonProjectConfig {
    #[serde(skip)]
    pub(crate) path: PathBuf,
    #[serde(rename = "blog")]
    pub(crate) blog_config: Option<BlogConfig>,
    #[serde(rename = "theme")]
    pub(crate) theme_config: Option<ThemeConfig>,
}

impl CommonProjectConfig {
    pub fn to_blog(&self) -> Result<Blog> {
        self.blog_config
            .clone()
            .context("Configuration file does not contain a blog section")
            .and_then(Blog::from_config)
    }

    pub fn to_theme(&self) -> Result<Theme> {
        self.theme_config
            .clone()
            .context("Configuration file does not contain a theme section")
            .and_then(|config| Theme::from_config(config, self.path.clone()))
    }
}

impl TryLoadConfig for CommonProjectConfig {
    fn update_path(&mut self, path: &std::path::Path) {
        self.path = path.to_path_buf();
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
