//! Home — Editorial Blueprint & Flagship Showcase for Yav.rs.

use yav::prelude::*;
use axum::extract::Request;
use crate::components::*;
use crate::layouts::*;

pub async fn index(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <HomeHero />
            <HomeStudio />
            <HomePrinciples />
            <HomeEcosystem />
            <HomeBenchmarks />
            <HomeToolchain />
            <HomeCta />
        </AppLayout>
    };
    website_page(
        "Yav.rs — The web framework for Rust",
        "A web framework for people who read the assembly. High-performance SSR with Rust 2021, Axum 0.8, and zero unnecessary runtime.",
        "/",
        page,
    )
}
