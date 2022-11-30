use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
