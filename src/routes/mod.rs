mod dashboard;
mod index;

pub use dashboard::*;
pub use index::*;

use askama::Template;
use htmx_framework::prelude::*;

#[derive(Template)]
#[template(path = "shell.html")]
pub struct DefaultShell {
    title: String,
    description: String,
    meta_tags: Vec<(String, String)>,
    content: String,
}

impl Shell for DefaultShell {
    fn new() -> Self {
        Self {
            title: "My App".to_string(),
            description: String::new(),
            meta_tags: Vec::new(),
            content: String::new(),
        }
    }

    fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    fn with_metadata(mut self, metadata: PageMeta) -> Self
    where
        Self: Sized,
    {
        self.title = metadata.title;
        self.description = metadata.description;
        self.meta_tags = metadata.meta_tags;
        self
    }
}

impl DefaultShell {
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = description.into();
        self
    }

    // Adds a meta tag to the shell template.
    #[allow(unused)]
    pub fn with_meta_tag<S1: Into<String>, S2: Into<String>>(
        mut self,
        name: S1,
        content: S2,
    ) -> Self {
        self.meta_tags.push((name.into(), content.into()));
        self
    }
}
