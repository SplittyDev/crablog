use anyhow::Result;

use crate::blog::Blog;

#[derive(Debug)]
pub struct BuildEngine {
    blog: Blog,
}

impl BuildEngine {
    pub fn new(blog: Blog) -> Self {
        Self { blog }
    }

    pub fn build(&self) -> Result<()> {
        Ok(())
    }
}
