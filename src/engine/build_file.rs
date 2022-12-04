use anyhow::{Context, Result};
use std::{
    borrow::Cow,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct BuildFile {
    virtual_path: PathBuf,
    content: String,
}

impl BuildFile {
    pub fn new(virtual_path: Cow<Path>, content: Cow<str>) -> Self {
        Self {
            virtual_path: virtual_path.into(),
            content: content.into(),
        }
    }

    pub fn write_to_disk(&self) -> Result<()> {
        // Construct target path
        let target_path = {
            let mut target_dir = Path::new("./build").to_path_buf();
            target_dir.push(&self.virtual_path);
            target_dir
        };

        // Create all directories
        {
            let target_dir = target_path
                .parent()
                .context("Unable to find parent directory of build target")?;
            std::fs::create_dir_all(target_dir)?;
        }

        // Write file to disk
        let mut file = File::create(target_path)?;
        file.write_all(self.content.as_bytes())?;

        Ok(())
    }
}
