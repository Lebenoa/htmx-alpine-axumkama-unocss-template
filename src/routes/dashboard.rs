use askama::Template;
use axum::response::IntoResponse;
use htmx_framework::prelude::*;

use crate::routes::DefaultShell;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardPage {
    username: String,
    stats_count: usize,
}

impl Page for DashboardPage {
    fn metadata(&self) -> PageMeta {
        PageMeta::new("Dashboard")
            .with_description("This is another page")
            .with_meta("og:title", "Dashboard")
            .with_meta("og:description", "Dashboard description")
    }
}

impl DashboardPage {
    pub fn new<S: Into<String>>(username: S, stats_count: usize) -> Self {
        Self {
            username: username.into(),
            stats_count,
        }
    }
}

pub async fn dashboard(mode: RenderMode) -> impl IntoResponse {
    let shell = DefaultShell::new()
        .with_title("Index Page")
        .with_description("My Index Page is here!");

    let page = DashboardPage::new("Lebenoa", 20);

    PageResponse::new(shell, page, mode)
}
