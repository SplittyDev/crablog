use serde::{Deserialize, Serialize};

/// Configuration for a theme project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub author: Option<String>,
    pub features: Vec<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "My theme".into(),
            author: Some("".into()),
            features: Vec::default(),
        }
    }
}
