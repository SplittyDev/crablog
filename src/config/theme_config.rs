use serde::{Serialize, Deserialize};

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