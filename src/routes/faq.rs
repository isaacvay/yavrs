//! FAQ & Architectural Comparison — Editorial Blueprint.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn faq(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="wrap py-12 sm:py-16 space-y-16 max-w-4xl">
                // Header
                <div class="space-y-4">
                    <div class="inline-flex items-center gap-2 px-3 py-1 bg-ink-2 border border-rule font-mono text-[11px] text-oxide">
                        {render_icon("help-circle", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        <span data-i18n="faq.badge">"RATIONALE · ARCHITECTURE TRADE-OFFS · FAQS"</span>
                    </div>
                    <h1 class="text-[36px] sm:text-[48px] font-bold text-paper tracking-[-0.03em] leading-tight" data-i18n="faq.title">
                        "Why Yav? Architecture & Comparisons"
                    </h1>
                    <p class="text-[16px] text-mute leading-relaxed max-w-2xl" data-i18n="faq.subtitle">
                        "Clear architectural motivations, honest technical trade-offs, and an unvarnished comparison between Yav, client-side WASM, and JavaScript full-stack frameworks."
                    </p>
                </div>

                // Matrix Comparison Table
                <div class="space-y-4">
                    <div class="text-paper font-semibold text-lg flex items-center justify-between gap-2">
                        <span data-i18n="faq.mat.title">"Architectural Comparison Matrix"</span>
                        <span class="text-xs font-mono text-dim md:hidden" data-i18n="faq.mat.scroll">"Scroll horizontally →"</span>
                    </div>
                    <div class="table-scroll pb-2">
                        <table class="dtable min-w-[640px]">
                            <thead>
                                <tr>
                                    <th data-i18n="faq.mat.col1">"Dimension"</th>
                                    <th>"Yav.rs"</th>
                                    <th>"Leptos (WASM)"</th>
                                    <th>"Next.js (Node)"</th>
                                    <th data-i18n="faq.mat.col5">"Bare Axum"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr class="us">
                                    <td class="text-paper font-semibold" data-i18n="faq.mat.r1.c1">"Runtime Tax"</td>
                                    <td class="text-ok font-bold" data-i18n="faq.mat.r1.c2">"0 kB JS / No WASM"</td>
                                    <td data-i18n="faq.mat.r1.c3">"2 - 8 MB WASM bundle"</td>
                                    <td data-i18n="faq.mat.r1.c4">"120 - 450 kB React JS"</td>
                                    <td data-i18n="faq.mat.r1.c5">"0 kB (Manual HTML)"</td>
                                </tr>
                                <tr>
                                    <td class="text-paper font-semibold" data-i18n="faq.mat.r2.c1">"Hydration Cost"</td>
                                    <td class="text-ok font-bold" data-i18n="faq.mat.r2.c2">"Zero Hydration"</td>
                                    <td data-i18n="faq.mat.r2.c3">"Heavy WASM startup"</td>
                                    <td data-i18n="faq.mat.r2.c4">"Full V8 tree walk"</td>
                                    <td data-i18n="faq.mat.r2.c5">"None"</td>
                                </tr>
                                <tr class="us">
                                    <td class="text-paper font-semibold" data-i18n="faq.mat.r3.c1">"Peak Req/Sec"</td>
                                    <td class="text-oxide font-bold">"124,500 req/s"</td>
                                    <td>"~45,000 req/s"</td>
                                    <td>"~8,500 req/s"</td>
                                    <td>"132,000 req/s"</td>
                                </tr>
                                <tr>
                                    <td class="text-paper font-semibold" data-i18n="faq.mat.r4.c1">"File-System Routing"</td>
                                    <td class="text-ok font-bold" data-i18n="faq.mat.r4.c2">"Built-in Radix Tree"</td>
                                    <td data-i18n="faq.mat.r4.c3">"Router crate config"</td>
                                    <td data-i18n="faq.mat.r4.c4">"App Router (heavy)"</td>
                                    <td data-i18n="faq.mat.r4.c5">"Manual route matching"</td>
                                </tr>
                                <tr class="us">
                                    <td class="text-paper font-semibold" data-i18n="faq.mat.r5.c1">"Tailwind Compiler"</td>
                                    <td class="text-ok font-bold" data-i18n="faq.mat.r5.c2">"Native Standalone JIT"</td>
                                    <td data-i18n="faq.mat.r5.c3">"npm / trunk setup"</td>
                                    <td data-i18n="faq.mat.r5.c4">"Node.js toolchain"</td>
                                    <td data-i18n="faq.mat.r5.c5">"None"</td>
                                </tr>
                                <tr>
                                    <td class="text-paper font-semibold" data-i18n="faq.mat.r6.c1">"Deployment Artifact"</td>
                                    <td class="text-ok font-bold">"< 30 MB Distroless"</td>
                                    <td data-i18n="faq.mat.r6.c3">"Server + Static WASM"</td>
                                    <td data-i18n="faq.mat.r6.c4">"150 MB+ Node image"</td>
                                    <td data-i18n="faq.mat.r6.c5">"Static binary"</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>

                // FAQ Accordion Cards
                <div class="space-y-6">
                    <div class="text-paper font-semibold text-lg" data-i18n="faq.list.title">
                        "Frequently Asked Questions"
                    </div>

                    // Question 1
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q1."</span>
                            <span data-i18n="faq.q1.q">"Why doesn't Yav compile down to WebAssembly (WASM) for the client?"</span>
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed" data-i18n="faq.q1.a">
                            "Compiling entire UI component trees to client-side WASM produces massive 2 MB to 10 MB initial bundle sizes, requires complex JavaScript interop bridges for DOM access, and drains battery life on mobile devices during startup. Yav renders pure, clean HTML directly on the server with hyper 1.0 throughput, pairing server power with htmx and declarative CSS/WAAPI for tactile UI."
                        </p>
                    </div>

                    // Question 2
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q2."</span>
                            <span data-i18n="faq.q2.q">"How do I handle dynamic client interactivity without React or Vue?"</span>
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            <span data-i18n="faq.q2.a.1">"Yav treats "</span>
                            <code class="inline">"htmx"</code>
                            <span data-i18n="faq.q2.a.2">" attributes ("</span>
                            <code class="inline">"hx-post"</code>
                            ", "
                            <code class="inline">"hx-target"</code>
                            ", "
                            <code class="inline">"hx-swap"</code>
                            <span data-i18n="faq.q2.a.3">") as first-class citizens in RSX markup. When a user filters a search list or submits a modal form, the server responds with a sub-millisecond "</span>
                            <code class="inline">"HtmlFragment"</code>
                            <span data-i18n="faq.q2.a.4">" partial that swaps directly into the DOM with zero client-side framework runtime."</span>
                        </p>
                    </div>

                    // Question 3
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q3."</span>
                            <span data-i18n="faq.q3.q">"Is Node.js really not required for development or Tailwind CSS?"</span>
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            <span data-i18n="faq.q3.a.1">"Absolutely not. The Yav CLI automatically downloads and caches the standalone official Tailwind standalone binary directly from GitHub releases matching your OS architecture. Compilation happens asynchronously on a background thread in milliseconds, without creating a "</span>
                            <code class="inline">"node_modules"</code>
                            <span data-i18n="faq.q3.a.2">" directory."</span>
                        </p>
                    </div>

                    // Question 4
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q4."</span>
                            <span data-i18n="faq.q4.q">"Can I still use JavaScript libraries or Web Components if needed?"</span>
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            <span data-i18n="faq.q4.a.1">"Yes. Yav emits clean, standard HTML. You can embed any web component, CDN script (e.g. Mapbox, Chart.js, Monaco), or custom TypeScript script using standard "</span>
                            <code class="inline">"<script>"</code>
                            <span data-i18n="faq.q4.a.2">" tags or vanilla ESM imports."</span>
                        </p>
                    </div>

                    // Question 5
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q5."</span>
                            <span data-i18n="faq.q5.q">"How is Yav deployed to production?"</span>
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            <span data-i18n="faq.q5.a.1">"Run "</span>
                            <code class="inline">"cargo build --release"</code>
                            <span data-i18n="faq.q5.a.2">" to produce a single self-contained native binary, or "</span>
                            <code class="inline">"yav docker"</code>
                            <span data-i18n="faq.q5.a.3">" to scaffold a multi-stage distroless Alpine container image under 30 MB. It can run on AWS, fly.io, Railway, bare-metal Linux servers, or Kubernetes pods with zero dependencies."</span>
                        </p>
                    </div>

                    // Question 6: AI & MCP
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q6."</span>
                            <span data-i18n="faq.q6.q">"How does Yav integrate AI, LLM streaming, and Model Context Protocol (MCP)?"</span>
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            <span data-i18n="faq.q6.a.1">"Yav provides a first-class native AI SDK in "</span>
                            <code class="inline">"yav::server::ai"</code>
                            <span data-i18n="faq.q6.a.2">" with streaming Server-Sent Events tokens, Hybrid RAG with cosine vector similarity, and an Anthropic-compliant Model Context Protocol server ("</span>
                            <code class="inline">"yav::server::mcp::McpServer"</code>
                            <span data-i18n="faq.q6.a.3">") that exposes your Rust functions directly to LLMs as typed tools."</span>
                        </p>
                    </div>

                    // Question 7: Passkeys & ReBAC
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q7."</span>
                            <span data-i18n="faq.q7.q">"Does Yav support enterprise authentication and authorization?"</span>
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            <span data-i18n="faq.q7.a.1">"Yes. Yav includes built-in WebAuthn FIDO2 / Passkey authentication in "</span>
                            <code class="inline">"yav::server::passkeys"</code>
                            <span data-i18n="faq.q7.a.2">" with hardware keys, Touch ID, and Face ID support, coupled with a Google Zanzibar-inspired Relationship-Based Access Control ("</span>
                            <code class="inline">"yav::server::rebac::RebacStore"</code>
                            <span data-i18n="faq.q7.a.3">") for sub-millisecond graph authorization checks."</span>
                        </p>
                    </div>
                </div>
            </div>
        </AppLayout>
    };

    website_page(
        "Why Yav? Architecture & Comparisons — Yav.rs",
        "Technical motivations, comparisons with Leptos, Next.js, and Axum, and answers to architectural trade-offs.",
        "/faq",
        page,
    )
}
