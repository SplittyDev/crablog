pub const CONFIG_FILENAME: &str = "Crablog.toml";

mod common_project_config;
mod config_error;

pub use common_project_config::CommonProjectConfig;
pub use config_error::ConfigError;
