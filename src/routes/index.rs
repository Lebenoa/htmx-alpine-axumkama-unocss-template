use askama::Template;
use axum::response::IntoResponse;
use htmx_framework::prelude::*;

use crate::routes::DefaultShell;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
    username: String,
    stats_count: usize,
}

impl Page for IndexPage {
    fn metadata(&self) -> PageMeta {
        PageMeta::new("HTMX + Axum Index!")
            .with_description("HTML + Alpine.js + Axum + etc..")
            .with_meta("og:type", "website")
    }
}

impl IndexPage {
    pub fn new<S: Into<String>>(username: S, stats_count: usize) -> Self {
        Self {
            username: username.into(),
            stats_count,
        }
    }
}

pub async fn index(mode: RenderMode) -> impl IntoResponse {
    let shell = DefaultShell::new()
        .with_title("Index Page")
        .with_description("My Index Page is here!");

    let page = IndexPage::new("Lebenoa", 20);

    PageResponse::new(shell, page, mode)
}
