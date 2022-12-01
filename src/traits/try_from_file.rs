use std::path::PathBuf;

use anyhow::Result;

pub trait TryFromFile {
    /// Try to create `Self` from the specified file.
    fn try_from_file(path: PathBuf) -> Result<Self>
    where
        Self: Sized;
}
