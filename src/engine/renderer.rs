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
    pub fn new(env: BuildEnvironment, base_layout: &'a ThemeLayout) -> Self {
        Self { env, base_layout }
    }

    pub fn render_index_page(
        &self,
        source: impl AsRef<str>,
        base_data: BaseDataBuilder,
        index_page_data: &IndexPageData,
    ) -> Result<String> {
        let handlebars = Handlebars::new();

        // Render index page template
        let rendered_html = handlebars.render_template(source.as_ref(), &index_page_data)?;

        // Render base template
        let base_data = base_data.with_content(rendered_html);
        let rendered_html =
            handlebars.render_template(self.base_layout.source.as_ref(), &base_data)?;

        // Minify if production build
        Ok(match self.env {
            BuildEnvironment::Production => {
                let cfg = minify_html::Cfg::spec_compliant();
                let minified_bytes = minify_html::minify(rendered_html.as_bytes(), &cfg);
                String::from_utf8(minified_bytes)?
            }
            _ => rendered_html,
        })
    }

    pub fn render_post_page(
        &self,
        source: impl AsRef<str>,
        base_data: BaseDataBuilder,
        post_page_data: &PostPageData,
    ) -> Result<String> {
        let handlebars = Handlebars::new();

        // Render post page template
        let rendered_html = handlebars.render_template(source.as_ref(), &post_page_data)?;

        // Render base template
        let base_data = base_data.with_content(rendered_html);
        let rendered_html =
            handlebars.render_template(self.base_layout.source.as_ref(), &base_data)?;

        // Minify if production build
        Ok(match self.env {
            BuildEnvironment::Production => {
                let cfg = minify_html::Cfg::spec_compliant();
                let minified_bytes = minify_html::minify(rendered_html.as_bytes(), &cfg);
                String::from_utf8(minified_bytes)?
            }
            _ => rendered_html,
        })
    }
}
