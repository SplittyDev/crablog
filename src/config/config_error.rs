#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerializationError(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserializationError(#[from] toml::de::Error),
}
