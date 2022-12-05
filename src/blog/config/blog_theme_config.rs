use serde::{Deserialize, Serialize};

use crate::theme::ThemeSource;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BlogThemeConfig {
    #[serde(flatten)]
    pub source: ThemeSource,
    pub features: Option<Vec<String>>,
}

impl BlogThemeConfig {
    pub fn source(&self) -> &ThemeSource {
        &self.source
    }

    pub fn features(&self) -> Vec<String> {
        self.features.clone().unwrap_or_default()
    }
}
