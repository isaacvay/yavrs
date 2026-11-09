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
                    <div class="text-paper font-semibold text-lg flex items-center gap-2">
                        <span>"Architectural Comparison Matrix"</span>
                    </div>
                    <table class="dtable">
                        <thead>
                            <tr>
                                <th>"Dimension"</th>
                                <th>"Yav.rs"</th>
                                <th>"Leptos (WASM)"</th>
                                <th>"Next.js (Node)"</th>
                                <th>"Bare Axum"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="us">
                                <td class="text-paper font-semibold">"Runtime Tax"</td>
                                <td class="text-ok font-bold">"0 kB JS / No WASM"</td>
                                <td>"2 - 8 MB WASM bundle"</td>
                                <td>"120 - 450 kB React JS"</td>
                                <td>"0 kB (Manual HTML)"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"Hydration Cost"</td>
                                <td class="text-ok font-bold">"Zero Hydration"</td>
                                <td>"Heavy WASM startup"</td>
                                <td>"Full V8 tree walk"</td>
                                <td>"None"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"Peak Req/Sec"</td>
                                <td class="text-oxide font-bold">"124,500 req/s"</td>
                                <td>"~45,000 req/s"</td>
                                <td>"~8,500 req/s"</td>
                                <td>"132,000 req/s"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"File-System Routing"</td>
                                <td class="text-ok font-bold">"Built-in Radix Tree"</td>
                                <td>"Router crate config"</td>
                                <td>"App Router (heavy)"</td>
                                <td>"Manual route matching"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"Tailwind Compiler"</td>
                                <td class="text-ok font-bold">"Native Standalone JIT"</td>
                                <td>"npm / trunk setup"</td>
                                <td>"Node.js toolchain"</td>
                                <td>"None"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"Deployment Artifact"</td>
                                <td class="text-ok font-bold">"< 30 MB Distroless"</td>
                                <td>"Server + Static WASM"</td>
                                <td>"150 MB+ Node image"</td>
                                <td>"Static binary"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                // FAQ Accordion Cards
                <div class="space-y-6">
                    <div class="text-paper font-semibold text-lg">
                        "Frequently Asked Questions"
                    </div>

                    // Question 1
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q1."</span>
                            "Why doesn't Yav compile down to WebAssembly (WASM) for the client?"
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            "Compiling entire UI component trees to client-side WASM produces massive 2 MB to 10 MB initial bundle sizes, requires complex JavaScript interop bridges for DOM access, and drains battery life on mobile devices during startup. Yav renders pure, clean HTML directly on the server with hyper 1.0 throughput, pairing server power with htmx and declarative CSS/WAAPI for tactile UI."
                        </p>
                    </div>

                    // Question 2
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q2."</span>
                            "How do I handle dynamic client interactivity without React or Vue?"
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            "Yav treats "
                            <code class="inline">"htmx"</code>
                            " attributes ("
                            <code class="inline">"hx-post"</code>
                            ", "
                            <code class="inline">"hx-target"</code>
                            ", "
                            <code class="inline">"hx-swap"</code>
                            ") as first-class citizens in RSX markup. When a user filters a search list or submits a modal form, the server responds with a sub-millisecond "
                            <code class="inline">"HtmlFragment"</code>
                            " partial that swaps directly into the DOM with zero client-side framework runtime."
                        </p>
                    </div>

                    // Question 3
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q3."</span>
                            "Is Node.js really not required for development or Tailwind CSS?"
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            "Absolutely not. The Yav CLI automatically downloads and caches the standalone official Tailwind standalone binary directly from GitHub releases matching your OS architecture. Compilation happens asynchronously on a background thread in milliseconds, without creating a "
                            <code class="inline">"node_modules"</code>
                            " directory."
                        </p>
                    </div>

                    // Question 4
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q4."</span>
                            "Can I still use JavaScript libraries or Web Components if needed?"
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            "Yes. Yav emits clean, standard HTML. You can embed any web component, CDN script (e.g. Mapbox, Chart.js, Monaco), or custom TypeScript script using standard "
                            <code class="inline">"<script>"</code>
                            " tags or vanilla ESM imports."
                        </p>
                    </div>

                    // Question 5
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q5."</span>
                            "How is Yav deployed to production?"
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            "Run "
                            <code class="inline">"cargo build --release"</code>
                            " to produce a single self-contained native binary, or "
                            <code class="inline">"yav docker"</code>
                            " to scaffold a multi-stage distroless Alpine container image under 30 MB. It can run on AWS, fly.io, Railway, bare-metal Linux servers, or Kubernetes pods with zero dependencies."
                        </p>
                    </div>

                    // Question 6: AI & MCP
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q6."</span>
                            "How does Yav integrate AI, LLM streaming, and Model Context Protocol (MCP)?"
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            "Yav provides a first-class native AI SDK in "
                            <code class="inline">"yav::server::ai"</code>
                            " with streaming Server-Sent Events tokens, Hybrid RAG with cosine vector similarity, and an Anthropic-compliant Model Context Protocol server ("
                            <code class="inline">"yav::server::mcp::McpServer"</code>
                            ") that exposes your Rust functions directly to LLMs as typed tools."
                        </p>
                    </div>

                    // Question 7: Passkeys & ReBAC
                    <div class="plate p-6 bg-ink-2 space-y-3">
                        <h3 class="text-paper font-bold text-base flex items-center gap-2">
                            <span class="text-oxide font-mono">"Q7."</span>
                            "Does Yav support enterprise authentication and authorization?"
                        </h3>
                        <p class="text-mute text-[14px] leading-relaxed">
                            "Yes. Yav includes built-in WebAuthn FIDO2 / Passkey authentication in "
                            <code class="inline">"yav::server::passkeys"</code>
                            " with hardware keys, Touch ID, and Face ID support, coupled with a Google Zanzibar-inspired Relationship-Based Access Control ("
                            <code class="inline">"yav::server::rebac::RebacStore"</code>
                            ") for sub-millisecond graph authorization checks."
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
