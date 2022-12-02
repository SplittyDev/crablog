use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::traits::ToThemeBundle;

use super::ThemeBundle;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeSource {
    Crablog {
        #[serde(flatten)]
        source: CrablogThemeSource,
    },
    Git {
        #[serde(rename = "theme")]
        source: GitThemeSource,
    },
    Local {
        #[serde(rename = "theme")]
        source: LocalThemeSource,
    },
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

impl ToThemeBundle for ThemeSource {
    fn to_theme_bundle(&self) -> Result<ThemeBundle> {
        match self {
            Self::Local { source } => source.to_theme_bundle(),
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

impl ToThemeBundle for LocalThemeSource {
    fn to_theme_bundle(&self) -> Result<super::ThemeBundle> {
        ThemeBundle::load_from_path(&self.path)
    }
}
