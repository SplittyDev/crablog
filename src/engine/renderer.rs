use anyhow::Result;
use handlebars::Handlebars;

use super::{full_data::FullData, BuildEnvironment};

#[derive(Debug)]
pub struct Renderer;

impl Renderer {
    pub fn render(
        env: BuildEnvironment,
        source: impl AsRef<str>,
        data: &FullData,
    ) -> Result<String> {
        // Render template
        let handlebars = Handlebars::new();
        let rendered_html = handlebars.render_template(source.as_ref(), &data)?;

        // Minify if production build
        Ok(match env {
            BuildEnvironment::Production => {
                let cfg = minify_html::Cfg::spec_compliant();
                let minified_bytes = minify_html::minify(rendered_html.as_bytes(), &cfg);
                String::from_utf8(minified_bytes)?
            }
            _ => rendered_html,
        })
    }
}
