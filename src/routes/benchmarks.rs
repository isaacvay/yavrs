//! Benchmarks — Editorial Blueprint & Complete Hardware-Verified Performance Reference.

use yav::prelude::*;
use axum::extract::Request;
use crate::layouts::*;
use crate::components::*;

pub async fn benchmarks(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="pb-24">
                <BenchHero />
                <BenchCharts />
                <BenchSimulator />
                <BenchMatrix />
                <BenchReproduce />
            </div>
        </AppLayout>
    };

    website_page(
        "Benchmarks & Hardware Performance Metrics — Yav.rs",
        "Hardware-verified throughput, latency distributions, SSR render times, and memory allocations comparing Yav, Actix, Axum, and Next.js.",
        "/benchmarks",
        page,
    )
}
