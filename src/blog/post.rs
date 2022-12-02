use std::{
    borrow::Cow,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::Result;
use comrak::{ComrakOptions, ComrakRenderOptions};
use minify_html::minify;

use crate::traits::TryFromFile;

use super::PostMetadata;

#[derive(Debug)]
pub struct Post {
    metadata: PostMetadata,
    path: PathBuf,
    source: String,
}

impl Post {
    #[cfg(test)]
    pub fn from_markdown_file_without_path(source: impl AsRef<str>) -> Result<Self> {
        let path = Path::new("/tmp/fake.md");
        let source = source.as_ref().trim().to_string();
        let metadata = PostMetadata::from_markdown(&source);
        Ok(Self {
            metadata,
            path: path.into(),
            source,
        })
    }

    pub fn from_markdown_file(path: Cow<Path>) -> Result<Self> {
        let source = read_to_string(&path)?.trim().to_string();
        let metadata = PostMetadata::from_markdown(&source);
        Ok(Self {
            metadata,
            path: path.into(),
            source,
        })
    }

    /// Render the post to html
    pub fn to_html(&self) -> Result<String> {
        // Allow unsafe HTML code in posts
        let render = ComrakRenderOptions {
            unsafe_: true,
            ..Default::default()
        };
        let options = ComrakOptions {
            render,
            ..Default::default()
        };
        let html = comrak::markdown_to_html(&self.source, &options);
        Ok(html.trim().to_string())
    }

    /// Render the post to minified spec-compliant html
    pub fn to_html_minified(&self) -> Result<String> {
        let html = self.to_html()?;
        let minify_config = minify_html::Cfg::spec_compliant();
        let minified_html = String::from_utf8(minify(html.as_bytes(), &minify_config))?;
        Ok(minified_html)
    }

    pub fn safe_name(&self) -> String {
        self.metadata
            .title
            .trim()
            .to_lowercase()
            .replace(' ', "_")
            .replace('/', "")
    }
}

impl TryFromFile for Post {
    fn try_from_file(path: Cow<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        Self::from_markdown_file(path)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use chrono::NaiveDateTime;

    use super::Post;

    #[test]
    fn post_from_markdown() -> Result<()> {
        let source = r"
[//]: # (title: Hello world)
[//]: # (created_at: 1970-01-01 00:00:00)
[//]: # (published: true)

# Hello world
> This is a test!

- Foo
- Bar
        ";
        let source_expected = r"[//]: # (title: Hello world)
[//]: # (created_at: 1970-01-01 00:00:00)
[//]: # (published: true)

# Hello world
> This is a test!

- Foo
- Bar";
        let post = Post::from_markdown_file_without_path(source)?;
        assert_eq!(post.metadata.title, "Hello world");
        assert_eq!(post.metadata.created_at, NaiveDateTime::default());
        assert_eq!(post.metadata.published, true);
        assert_eq!(post.source, source_expected);
        Ok(())
    }

    #[test]
    fn post_to_html() -> Result<()> {
        let source = r"[//]: # (title: Hello world)
[//]: # (created_at: 1970-01-01 00:00:00)
[//]: # (published: true)

# Hello world
> This is a test!

- Foo
- Bar";
        let source_html = r"<h1>Hello world</h1>
<blockquote>
<p>This is a test!</p>
</blockquote>
<ul>
<li>Foo</li>
<li>Bar</li>
</ul>"
            .trim();
        let post = Post::from_markdown_file_without_path(source)?;
        let html = post.to_html()?;
        assert_eq!(html, source_html);
        Ok(())
    }

    #[test]
    fn post_to_html_minified() -> Result<()> {
        let source = r"[//]: # (title: Hello world)
[//]: # (created_at: 1970-01-01 00:00:00)
[//]: # (published: true)

# Hello world
> This is a test!

- Foo
- Bar";
        let source_html = r"<h1>Hello world</h1><blockquote><p>This is a test!</blockquote><ul><li>Foo<li>Bar</ul>";
        let post = Post::from_markdown_file_without_path(source)?;
        let html = post.to_html_minified()?;
        assert_eq!(html, source_html);
        Ok(())
    }

    #[test]
    fn post_safe_name() -> Result<()> {
        let source = r"[//]: # (title: Hello world)";
        let post = Post::from_markdown_file_without_path(source)?;
        assert_eq!(post.safe_name(), "hello_world");
        Ok(())
    }
}
