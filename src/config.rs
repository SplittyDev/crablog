pub const CONFIG_FILENAME: &str = "Crablog.toml";

mod config_error;
mod common_project_config;
mod blog_config;
mod theme_config;

pub use config_error::ConfigError;
pub use common_project_config::CommonProjectConfig;
pub use blog_config::{BlogConfig, BlogMetadataConfig};
pub use theme_config::ThemeConfig;
