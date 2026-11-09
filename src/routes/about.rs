//! Architecture — Editorial Blueprint & System Specifications.

use yav::prelude::*;
use axum::extract::Request;
use crate::components::*;
use crate::layouts::*;

pub async fn about(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <ArchHero />
            <ArchPipeline />
            <ArchMemory />
            <ArchCrates />
            <ArchSecurity />
            <ArchConcurrency />
        </AppLayout>
    };
    website_page(
        "Architecture — Yav.rs",
        "Deep architectural breakdown of Yav: zero-copy virtual DOM, Axum 0.8 SSR engine, memory safety, and crate taxonomy.",
        "/about",
        page,
    )
}
