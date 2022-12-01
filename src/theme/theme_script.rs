use anyhow::Result;
use std::{fs::read_to_string, path::PathBuf};

use crate::traits::TryFromFile;

#[derive(Debug)]
pub struct ThemeScript {
    path: PathBuf,
    source: String,
}

impl TryFromFile for ThemeScript {
    fn try_from_file(path: PathBuf) -> Result<Self>
    where
        Self: Sized,
    {
        let source = read_to_string(&path)?;
        Ok(Self { path, source })
    }
}
