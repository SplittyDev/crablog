use anyhow::Result;
use comrak::ComrakOptions;

use super::PostMetadata;

#[derive(Debug)]
pub struct Post {
    metadata: PostMetadata,
    source: String,
}

impl Post {
    pub fn from_markdown(source: impl AsRef<str>) -> Result<Self> {
        let source = source.as_ref().trim().to_string();
        let metadata = PostMetadata::from_markdown(&source);
        Ok(Self { metadata, source })
    }

    pub fn to_html(&self) -> Result<String> {
        // TODO: Parse handlebars templates from theme
        let html = comrak::markdown_to_html(&self.source, &ComrakOptions::default());
        Ok(html.trim().to_string())
    }

    pub fn to_html_minified(&self) -> Result<String> {
        Ok(self.to_html()?.replace('\n', ""))
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;

    use super::Post;

    #[test]
    fn post_from_markdown() {
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
        let post = Post::from_markdown(source).unwrap();
        assert_eq!(post.metadata.title, "Hello world");
        assert_eq!(post.metadata.created_at, NaiveDateTime::default());
        assert_eq!(post.metadata.published, true);
        assert_eq!(post.source, source_expected);
    }

    #[test]
    fn post_to_html() {
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
        let post = Post::from_markdown(source).unwrap();
        let html = post.to_html().unwrap();
        assert_eq!(html, source_html);
    }
}
