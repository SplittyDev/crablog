use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    config::CommonProjectConfig,
    traits::{ToTheme, TryLoadConfig},
};

use super::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeSource {
    Crablog { source: CrablogThemeSource },
    Git { source: GitThemeSource },
    Local { source: LocalThemeSource },
}

impl ThemeSource {
    pub fn default_local() -> Self {
        Self::Local {
            source: LocalThemeSource {
                path: PathBuf::from("."),
            },
        }
    }
}

impl ToTheme for ThemeSource {
    fn to_theme(&self) -> Result<Theme> {
        match self {
            Self::Local { source } => source.to_theme(),
            _ => todo!(),
        }
    }
}

impl Default for ThemeSource {
    fn default() -> Self {
        Self::Crablog {
            source: CrablogThemeSource {
                id: "@crablog/default".into(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrablogThemeSource {
    #[serde(rename = "theme")]
    id: String,
}

impl Default for CrablogThemeSource {
    fn default() -> Self {
        Self {
            id: "crablog/pure-html".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitThemeSource {
    git: String,
    branch: Option<String>,
    rev: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalThemeSource {
    path: PathBuf,
}

impl ToTheme for LocalThemeSource {
    fn to_theme(&self) -> Result<super::Theme> {
        CommonProjectConfig::try_load_from(&self.path)?
            .to_theme()
            .context("Unable to load theme description from config file")
    }
}
