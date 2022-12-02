use std::path::Path;

use anyhow::Result;

use crate::{blog::Blog, engine::renderer::Renderer};

use super::{build_file::BuildFile, full_data::FullData, post_data::PostData, BuildEnvironment};

#[derive(Debug)]
pub struct BuildEngine {
    env: BuildEnvironment,
    blog: Blog,
    build_files: Vec<BuildFile>,
}

impl BuildEngine {
    pub fn new(env: BuildEnvironment, blog: Blog) -> Self {
        Self {
            env,
            blog,
            build_files: Vec::new(),
        }
    }

    pub fn build(&mut self) -> Result<()> {
        log::info!("Building blog");
        let posts = self.build_posts()?;
        println!("{:#?}", self.build_files);
        Ok(())
    }

    fn build_posts(&mut self) -> Result<()> {
        let theme = self.blog.theme_bundle();
        let post_layout = theme.post_layout()?;
        for post in self.blog.iter_posts() {
            log::debug!("Building post: {}", post.safe_name());
            let virtual_path = {
                let virtual_path = format!("posts/{}.html", post.safe_name());
                Path::new(&virtual_path).to_path_buf()
            };

            // Build data for handlebars rendering
            let post_data = PostData::try_from(post)?;
            let full_data = FullData {
                post: Some(post_data),
            };

            // Render post
            let output = Renderer::render(self.env, &post_layout.source, &full_data)?;

            // Push build output
            let build_file = BuildFile::new(virtual_path.into(), output.into());
            self.build_files.push(build_file);
        }
        Ok(())
    }
}
