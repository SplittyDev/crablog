use anyhow::Result;
use handlebars::Handlebars;

use crate::theme::ThemeLayout;

use super::{
    data::{BaseDataBuilder, IndexPageData, PostPageData},
    BuildEnvironment,
};

#[derive(Debug)]
pub struct Renderer<'a> {
    env: BuildEnvironment,
    base_layout: &'a ThemeLayout,
}

impl<'a> Renderer<'a> {
    /// Construct a new `Renderer` for a given environment and base layout.
    pub fn new(env: BuildEnvironment, base_layout: &'a ThemeLayout) -> Self {
        Self { env, base_layout }
    }

    /// Render the index page.
    pub fn render_index_page(
        &self,
        source: impl AsRef<str>,
        base_data: BaseDataBuilder,
        index_page_data: IndexPageData,
    ) -> Result<String> {
        let handlebars = Handlebars::new();

        // Render index page template
        let data = base_data.build(index_page_data);
        let rendered_html = handlebars.render_template(source.as_ref(), &data)?;

        // Render base template
        let data = data.into_base_data().with_content(rendered_html);
        let rendered_html = handlebars.render_template(self.base_layout.source.as_ref(), &data)?;

        // Minify if production build
        self.postprocess_html(rendered_html)
    }

    /// Render a post page.
    pub fn render_post_page(
        &self,
        source: impl AsRef<str>,
        base_data: BaseDataBuilder,
        post_page_data: PostPageData,
    ) -> Result<String> {
        let handlebars = Handlebars::new();

        // Render post page template
        let data = base_data.build(post_page_data);
        let rendered_html = handlebars.render_template(source.as_ref(), &data)?;

        // Render base template
        let data = data.into_base_data().with_content(rendered_html);
        let rendered_html = handlebars.render_template(self.base_layout.source.as_ref(), &data)?;

        // Minify if production build
        self.postprocess_html(rendered_html)
    }

    /// Format output html file according to current environment.
    /// When building for production, the html source is minified.
    fn postprocess_html(&self, html: String) -> Result<String> {
        Ok(match self.env {
            // Minify HTML
            BuildEnvironment::Production => {
                let cfg = minify_html::Cfg::spec_compliant();
                let minified_bytes = minify_html::minify(html.as_bytes(), &cfg);
                String::from_utf8(minified_bytes)?
            }
            // Remove empty lines to get rid of artifacts from template rendering
            // and improve readability of the rendered html source.
            _ => html
                .lines()
                .filter(|&line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n"),
        })
    }
}
