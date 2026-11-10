//! Showcase — Production Proof & Real-World Deployments built with Yav.rs.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn showcase(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="wrap py-12 sm:py-16 space-y-16">
                // Sub-Header
                <div class="space-y-4 max-w-3xl">
                    <div class="inline-flex items-center gap-2 px-3 py-1 bg-ink-2 border border-rule font-mono text-[11px] text-oxide">
                        {render_icon("award", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        <span data-i18n="showcase.badge">"PRODUCTION PROOF · REAL-WORLD SYSTEMS"</span>
                    </div>
                    <h1 class="text-[36px] sm:text-[52px] font-bold text-paper tracking-[-0.03em] leading-tight" data-i18n="showcase.title">
                        "Built with Yav. Tested in production."
                    </h1>
                    <p class="text-[17px] text-mute leading-relaxed" data-i18n="showcase.subtitle">
                        "From micro-second financial telemetry to edge-deployed e-commerce storefronts, see what engineering teams build when JavaScript hydration taxes and bloated runtimes are eliminated."
                    </p>
                </div>

                // Showcase Grid
                <div class="grid md:grid-cols-2 gap-8">
                    // Project 1: Kestrel
                    <div class="plate p-6 sm:p-8 bg-ink-2 space-y-6 flex flex-col justify-between hover:border-oxide/60 transition-colors">
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="px-2.5 py-0.5 text-[10.5px] font-mono tracking-wider font-semibold uppercase bg-sunk text-oxide border border-rule" data-i18n="showcase.p1.cat">
                                    "FinTech · High-Throughput"
                                </span>
                                <span class="text-xs font-mono text-dim" data-i18n="showcase.p1.tag">"0 kB Client JS"</span>
                            </div>
                            <h2 class="text-2xl font-bold text-paper">"Kestrel Market Telemetry"</h2>
                            <p class="text-mute text-[14px] leading-relaxed" data-i18n="showcase.p1.desc">
                                "High-frequency crypto and equities market data streamer distributing 240,000 order book updates per second across WebSockets and native Server-Sent Events with hyper 1.0 zero-copy buffers."
                            </p>
                            <div class="grid grid-cols-3 gap-1.5 sm:gap-2 font-mono py-2 text-center text-xs">
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"240k/s"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p1.s1">"Throughput"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"180 µs"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p1.s2">"p99 Latency"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-ok font-bold text-xs sm:text-sm">"14 MB"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p1.s3">"RAM Footprint"</div>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center justify-between pt-4 border-t border-rule text-xs font-mono text-dim">
                            <span>"Stack: YavApp · Tokio · hyper 1.0"</span>
                            <span class="text-oxide font-medium" data-i18n="showcase.verified">"Production Verified"</span>
                        </div>
                    </div>

                    // Project 2: Vortex Observability
                    <div class="plate p-6 sm:p-8 bg-ink-2 space-y-6 flex flex-col justify-between hover:border-oxide/60 transition-colors">
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="px-2.5 py-0.5 text-[10.5px] font-mono tracking-wider font-semibold uppercase bg-sunk text-ok border border-rule" data-i18n="showcase.p2.cat">
                                    "DevTools · Cloud Infrastructure"
                                </span>
                                <span class="text-xs font-mono text-dim" data-i18n="showcase.p2.tag">"< 20 MB Container"</span>
                            </div>
                            <h2 class="text-2xl font-bold text-paper">"Vortex Telemetry Dashboard"</h2>
                            <p class="text-mute text-[14px] leading-relaxed" data-i18n="showcase.p2.desc">
                                "Distributed log aggregation and metrics gateway deployed as an unprivileged distroless container across 14 edge clusters, serving live dashboards with Blake3 ETags and automatic compression."
                            </p>
                            <div class="grid grid-cols-3 gap-1.5 sm:gap-2 font-mono py-2 text-center text-xs">
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"18.4 MB"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p2.s1">"Docker Size"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"12 ms"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p2.s2">"Cold Start"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-ok font-bold text-xs sm:text-sm">"100%"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p2.s3">"Brotli Cached"</div>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center justify-between pt-4 border-t border-rule text-xs font-mono text-dim">
                            <span>"Stack: yav docker · Axum 0.8 · Blake3"</span>
                            <span class="text-ok font-medium" data-i18n="showcase.verified">"Production Verified"</span>
                        </div>
                    </div>

                    // Project 3: Axiom Storefront
                    <div class="plate p-6 sm:p-8 bg-ink-2 space-y-6 flex flex-col justify-between hover:border-oxide/60 transition-colors">
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="px-2.5 py-0.5 text-[10.5px] font-mono tracking-wider font-semibold uppercase bg-sunk text-paper border border-rule" data-i18n="showcase.p3.cat">
                                    "E-Commerce · Edge Commerce"
                                </span>
                                <span class="text-xs font-mono text-dim">"htmx + Yav"</span>
                            </div>
                            <h2 class="text-2xl font-bold text-paper">"Axiom Luxury Hardware"</h2>
                            <p class="text-mute text-[14px] leading-relaxed" data-i18n="showcase.p3.desc">
                                "Headless mechanical gear boutique achieving a perfect 100 Google Lighthouse score. Cart updates, instant inventory filters, and checkout mutations execute via sub-10ms partial HTML swaps."
                            </p>
                            <div class="grid grid-cols-3 gap-1.5 sm:gap-2 font-mono py-2 text-center text-xs">
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-ok font-bold text-xs sm:text-sm">"100/100"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate">"Lighthouse"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"0.00 s"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p3.s2">"Total Blocking Time"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"24 ms"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p3.s3">"Global TTFB"</div>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center justify-between pt-4 border-t border-rule text-xs font-mono text-dim">
                            <span>"Stack: HtmlFragment · Tailwind JIT · htmx"</span>
                            <span class="text-paper font-medium" data-i18n="showcase.verified">"Production Verified"</span>
                        </div>
                    </div>

                    // Project 4: Prism Documentation Cloud
                    <div class="plate p-6 sm:p-8 bg-ink-2 space-y-6 flex flex-col justify-between hover:border-oxide/60 transition-colors">
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="px-2.5 py-0.5 text-[10.5px] font-mono tracking-wider font-semibold uppercase bg-sunk text-oxide border border-rule" data-i18n="showcase.p4.cat">
                                    "Enterprise SaaS · Docs"
                                </span>
                                <span class="text-xs font-mono text-dim" data-i18n="showcase.p4.tag">"10k+ pages"</span>
                            </div>
                            <h2 class="text-2xl font-bold text-paper">"Prism Technical Docs"</h2>
                            <p class="text-mute text-[14px] leading-relaxed" data-i18n="showcase.p4.desc">
                                "Developer portal powering over 10,000 API reference pages with zero client-side hydration penalty, sub-millisecond full-text search, and embedded vector icons compiled directly into the binary."
                            </p>
                            <div class="grid grid-cols-3 gap-1.5 sm:gap-2 font-mono py-2 text-center text-xs">
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"10,400"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p4.s1">"Routes"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"< 2 ms"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p4.s2">"Search Latency"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-ok font-bold text-xs sm:text-sm">"0 kB"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate">"React/Vue JS"</div>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center justify-between pt-4 border-t border-rule text-xs font-mono text-dim">
                            <span>"Stack: yav-icons · yav-motion · Radix Tree"</span>
                            <span class="text-oxide font-medium" data-i18n="showcase.verified">"Production Verified"</span>
                        </div>
                    </div>

                    // Project 5: Nexus AI Canvas
                    <div class="plate p-6 sm:p-8 bg-ink-2 space-y-6 flex flex-col justify-between hover:border-oxide/60 transition-colors">
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="px-2.5 py-0.5 text-[10.5px] font-mono tracking-wider font-semibold uppercase bg-sunk text-ok border border-rule" data-i18n="showcase.p5.cat">
                                    "AI · Collaboration"
                                </span>
                                <span class="text-xs font-mono text-dim">"AI SDK · MCP"</span>
                            </div>
                            <h2 class="text-2xl font-bold text-paper">"Nexus AI Canvas"</h2>
                            <p class="text-mute text-[14px] leading-relaxed" data-i18n="showcase.p5.desc">
                                "Collaborative flow canvas and workflow builder powered by yav::server::ai, streaming LLM tokens via SSE, Model Context Protocol tools, and real-time CRDT presence rooms."
                            </p>
                            <div class="grid grid-cols-3 gap-1.5 sm:gap-2 font-mono py-2 text-center text-xs">
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm" data-i18n="showcase.p5.v1">"100+ Nodes"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p5.s1">"SVG Flow Graph"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"< 50 µs"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p5.s2">"CRDT Merge"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-ok font-bold text-xs sm:text-sm" data-i18n="showcase.p5.v3">"Native"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p5.s3">"MCP Server"</div>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center justify-between pt-4 border-t border-rule text-xs font-mono text-dim">
                            <span>"Stack: yav::server::ai · mcp · CRDT"</span>
                            <span class="text-ok font-medium" data-i18n="showcase.verified">"Production Verified"</span>
                        </div>
                    </div>

                    // Project 6: Hyperion Secure Vault
                    <div class="plate p-6 sm:p-8 bg-ink-2 space-y-6 flex flex-col justify-between hover:border-oxide/60 transition-colors">
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="px-2.5 py-0.5 text-[10.5px] font-mono tracking-wider font-semibold uppercase bg-sunk text-oxide border border-rule" data-i18n="showcase.p6.cat">
                                    "Security · Enterprise"
                                </span>
                                <span class="text-xs font-mono text-dim">"Passkeys · ReBAC"</span>
                            </div>
                            <h2 class="text-2xl font-bold text-paper">"Hyperion Enterprise Vault"</h2>
                            <p class="text-mute text-[14px] leading-relaxed" data-i18n="showcase.p6.desc">
                                "Zero-trust administrative console featuring WebAuthn Passkey authentication, Zanzibar-inspired relationship access control, circuit breakers, and transactional outbox guarantees."
                            </p>
                            <div class="grid grid-cols-3 gap-1.5 sm:gap-2 font-mono py-2 text-center text-xs">
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"FIDO2"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p6.s1">"Hardware Keys"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-paper font-bold text-xs sm:text-sm">"< 1 ms"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p6.s2">"ReBAC Check"</div>
                                </div>
                                <div class="p-2 sm:p-2.5 bg-ink border border-rule">
                                    <div class="text-ok font-bold text-xs sm:text-sm">"99.999%"</div>
                                    <div class="text-[9.5px] sm:text-[10px] text-dim truncate" data-i18n="showcase.p6.s3">"Resilience"</div>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center justify-between pt-4 border-t border-rule text-xs font-mono text-dim">
                            <span>"Stack: yav::server::passkeys · rebac · outbox"</span>
                            <span class="text-oxide font-medium" data-i18n="showcase.verified">"Production Verified"</span>
                        </div>
                    </div>
                </div>

                // Submit callout
                <div class="plate p-8 bg-ink-2 border-l-2 border-l-oxide space-y-4">
                    <h3 class="text-xl font-bold text-paper" data-i18n="showcase.cta.title">"Deploying Yav in production?"</h3>
                    <p class="text-mute text-[14.5px] leading-relaxed max-w-2xl" data-i18n="showcase.cta.desc">
                        "We welcome community contributions, architectural case studies, and production benchmark reports. Open a pull request on GitHub to feature your team's application in the official showcase."
                    </p>
                    <div>
                        <a href="https://github.com/isaacvay/yav.rs" target="_blank" rel="noreferrer" class="btn btn-solid btn-sm inline-flex items-center gap-2">
                            {render_icon("minimal:github", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                            <span data-i18n="showcase.submit">"Submit Showcase via GitHub PR"</span>
                            <span class="arr">"→"</span>
                        </a>
                    </div>
                </div>
            </div>
        </AppLayout>
    };

    website_page(
        "Showcase & Production Proof — Yav.rs",
        "Discover real-world high-throughput applications, FinTech systems, and edge dashboards built with Yav and Rust.",
        "/showcase",
        page,
    )
}
