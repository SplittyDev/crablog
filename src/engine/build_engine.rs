use std::{path::Path, time::Instant};

use anyhow::Result;

use crate::{
    blog::Blog,
    engine::{
        data::{BaseDataBuilder, PostData, PostPageData},
        renderer::Renderer,
    },
    theme::LayoutKind,
};

use super::{build_file::BuildFile, data::IndexPageData, BuildEnvironment};

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
        let start_time = Instant::now();
        log::info!("Building blog");
        self.build_index()?;
        self.build_posts()?;
        log::debug!("Writing build output to disk");
        self.write_to_disk()?;
        log::debug!(
            "Done (took {:.2}s)",
            Instant::now().duration_since(start_time).as_secs_f32()
        );
        Ok(())
    }

    fn build_index(&mut self) -> Result<()> {
        let output = {
            // Get layouts
            let theme = self.blog.theme_bundle();
            let base_layout = theme.get_layout(LayoutKind::Base)?;
            let index_layout = theme.get_layout(LayoutKind::Index)?;

            // Build post data from posts
            let posts = self
                .blog
                .iter_posts()
                .filter_map(|post| PostData::try_from(post).ok())
                .collect::<Vec<_>>();

            // Build templating data
            let base_data =
                BaseDataBuilder::new().with_metadata(self.blog.config().meta.clone().into());
            let index_page_data = IndexPageData { posts };

            let renderer = Renderer::new(self.env, base_layout);
            renderer.render_index_page(&index_layout.source, base_data, &index_page_data)?
        };

        // Push build output
        self.build_files.push(BuildFile::new(
            Path::new("index.html").into(),
            output.into(),
        ));

        Ok(())
    }

    fn build_posts(&mut self) -> Result<()> {
        let theme = self.blog.theme_bundle();
        let base_layout = theme.get_layout(LayoutKind::Base)?;
        let post_layout = theme.get_layout(LayoutKind::Post)?;
        let renderer = Renderer::new(self.env, base_layout);

        for post in self.blog.iter_posts() {
            log::debug!("Building post: {}", post.safe_name());
            let virtual_path = {
                let virtual_path = format!("posts/{}.html", post.safe_name());
                Path::new(&virtual_path).to_path_buf()
            };

            // Build data for handlebars rendering
            let base_data =
                BaseDataBuilder::new().with_metadata(self.blog.config().meta.clone().into());
            let post_data = PostData::try_from(post)?;
            let post_page_data = PostPageData {
                post: Some(post_data),
            };

            // Render post page
            let output =
                renderer.render_post_page(&post_layout.source, base_data, &post_page_data)?;

            // Push build output
            let build_file = BuildFile::new(virtual_path.into(), output.into());
            self.build_files.push(build_file);
        }

        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<()> {
        for build_file in &self.build_files {
            build_file.write_to_disk()?;
        }
        Ok(())
    }
}
