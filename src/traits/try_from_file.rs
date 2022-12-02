use std::{borrow::Cow, path::Path};

use anyhow::Result;

pub trait TryFromFile {
    /// Try to create `Self` from the specified file.
    fn try_from_file(path: Cow<Path>) -> Result<Self>
    where
        Self: Sized;
}
