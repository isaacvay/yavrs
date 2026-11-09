//! Releases & Changelog Timeline — Editorial Blueprint.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn releases(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="wrap py-12 sm:py-16 space-y-16 max-w-4xl">
                // Header
                <div class="space-y-4">
                    <div class="inline-flex items-center gap-2 px-3 py-1 bg-ink-2 border border-rule font-mono text-[11px] text-oxide">
                        {render_icon("git-branch", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        <span data-i18n="releases.badge">"SEMVER TIMELINE · PRODUCTION CHANGELOG"</span>
                    </div>
                    <h1 class="text-[36px] sm:text-[48px] font-bold text-paper tracking-[-0.03em] leading-tight" data-i18n="releases.title">
                        "Releases & Engineering Changelog"
                    </h1>
                    <p class="text-[16px] text-mute leading-relaxed max-w-2xl" data-i18n="releases.subtitle">
                        "Transparent semver progression, backward-compatibility invariants, and runtime performance benchmarks across every published edition of the Yav toolchain."
                    </p>
                </div>

                // Timeline
                <div class="space-y-12">
                    // Release v0.1.4
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-ok">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.4"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-ok border border-rule">
                                    "CURRENT STABLE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim">"October 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed">
                            "Milestone enterprise release introducing Model Context Protocol (MCP) server, native AI SDK with streaming UI & Hybrid RAG, WebAuthn Passkeys biometric authentication, 60+ enterprise UI components (Charts, DataGrid, Kanban with O(1) LexoRank, FlowCanvas), proactive resilience (CircuitBreaker, Bulkhead, Multi-Region DB Replicas), modern protocols (GraphQL, TurboStream, Tus v1.0.0 resumable uploads), real-time CRDT collaboration, O(L) Radix RouteTrie with parallel/intercepting slots, Zero-JS motion physics, and the complete 20-command yav CLI suite."
                        </p>
                        <div class="space-y-3 font-sans text-xs">
                            <div class="text-paper font-semibold font-mono uppercase tracking-wider text-[11px]">
                                "Highlights & Architectural Additions:"
                            </div>
                            <ul class="space-y-2 text-mute pl-4 list-disc">
                                <li><strong class="text-paper">"Model Context Protocol (MCP) & Native AI SDK:"</strong>" JSON-RPC 2.0 McpServer exposing live tools, resources, and prompt templates to AI agents (Claude Desktop, Cursor, Copilot), multi-turn Conversation, JSON-Schema ToolRegistry, stream_ui_fragment SSE streaming, cosine VectorIndex, and Reciprocal Rank Fusion (reciprocal_rank_fusion)."</li>
                                <li><strong class="text-paper">"WebAuthn / Passkeys Biometric Auth:"</strong>" Native passwordless FIDO2 authentication (Touch ID, Face ID, Windows Hello, YubiKey) with challenge verification and client scripts via PasskeyManager."</li>
                                <li><strong class="text-paper">"60+ Enterprise UI Components & Charts:"</strong>" Server-rendered SVG charts (LineChart, BarChart, DonutChart, Sparkline, Heatmap, Sankey, Radar), enterprise DataGrid, KanbanBoard with O(1) LexoRank, FlowCanvas with live Bézier curve rerouting, in-app DevTools HUD, Calendar/Scheduler, reactive Spreadsheet formula engine, and pure-Rust 3D ModelViewer."</li>
                                <li><strong class="text-paper">"Proactive Resilience & High Availability:"</strong>" CircuitBreaker with half-open recovery probes, Bulkhead isolation, AdaptiveConcurrencyLimiter, TransactionalOutbox pattern with event bus, and multi-region DbReplicaPool with Read-Your-Own-Writes session stickiness."</li>
                                <li><strong class="text-paper">"Modern Protocols in Pure Rust:"</strong>" Zero-dependency GraphQlSchema with embedded GraphiQL 3.x explorer, Hotwire-compatible TurboStream multi-zone DOM mutations (7 actions), Tus Protocol v1.0.0 resumable chunked uploads with disk spooling, and RFC 8291/8292 Web Push with VAPID."</li>
                                <li><strong class="text-paper">"Real-Time Collaboration & CRDTs:"</strong>" Conflict-free LwwRegister/LwwMap, multiplayer PresenceRoom cursor/selection tracking, LexoRank O(1) fractional ordering, StateMachine statecharts, and SagaWorkflow compensating transactions."</li>
                                <li><strong class="text-paper">"Multi-Tier Distributed Cache & XFetch:"</strong>" L1 in-memory + L2 distributed cache with probabilistic early expiration (XFetch) to completely eliminate cache stampedes, paired with cluster-wide DistributedLock and leader election."</li>
                                <li><strong class="text-paper">"O(L) Radix Route Trie & App Router Slots:"</strong>" Constant O(L) route resolution matching file routing, Parallel Routes (@slot), Intercepting Routes ((.) / (..) / (...)), Optional Catch-All ([[...slug]]), and auto-wiring #[server] Server Actions."</li>
                                <li><strong class="text-paper">"Database Tooling & 20 CLI Commands:"</strong>" Full yav db suite with SQLite and PostgreSQL runners, Up/Down migrations, and SHA-256 tamper-evident journal checks in _yav_migrations."</li>
                            </ul>
                        </div>
                        <div class="pt-4 border-t border-rule flex flex-wrap items-center justify-between text-xs font-mono text-dim gap-2">
                            <span>"crates.io: yav 0.1.4 · yav-cli 0.1.4"</span>
                            <a href="https://github.com/isaacvay/yav.rs/releases" target="_blank" rel="noreferrer" class="text-oxide hover:underline">
                                "View GitHub Release →"
                            </a>
                        </div>
                    </section>

                    // Release v0.1.3
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-oxide">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.3"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-dim border border-rule">
                                    "STABLE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim">"October 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed">
                            "Major server engine modernization introducing Axum 0.8.1 foundation, hyper 1.0 high-concurrency primitives, and the embedded 13,000+ vector icon system with zero network requests."
                        </p>
                        <div class="space-y-3 font-sans text-xs">
                            <div class="text-paper font-semibold font-mono uppercase tracking-wider text-[11px]">
                                "Highlights & Architectural Additions:"
                            </div>
                            <ul class="space-y-2 text-mute pl-4 list-disc">
                                <li><strong class="text-paper">"Axum 0.8 & hyper 1.0 Foundation:"</strong>" Complete refactor of internal router pipeline for zero-copy buffer streaming and lock-free concurrency."</li>
                                <li><strong class="text-paper">"Embedded 13,000+ Icon Engine (yav-icons):"</strong>" Lucide, MynaUI, and Phosphor sets bundled directly into the binary with size enums."</li>
                                <li><strong class="text-paper">"Sliding-Window Rate Limiting:"</strong>" Cryptographic IP and token-bucket protection against DDoS and automated credential stuffing."</li>
                                <li><strong class="text-paper">"Zero-Tax Pre-Compression:"</strong>" Built-in Brotli and Gzip archive generation during release builds with Blake3 ETags."</li>
                            </ul>
                        </div>
                        <div class="pt-4 border-t border-rule flex flex-wrap items-center justify-between text-xs font-mono text-dim gap-2">
                            <span>"crates.io: yav 0.1.3 · yav-cli 0.1.3"</span>
                            <a href="https://github.com/isaacvay/yav.rs/releases" target="_blank" rel="noreferrer" class="text-oxide hover:underline">
                                "View GitHub Release →"
                            </a>
                        </div>
                    </section>

                    // Release v0.1.2
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-oxide">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.2"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-dim border border-rule">
                                    "STABLE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim">"September 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed">
                            "Security hardening update bringing stateless cryptographic sessions, constant-time CSRF token verification, and signed Flash notification cookies."
                        </p>
                        <div class="space-y-3 font-sans text-xs">
                            <div class="text-paper font-semibold font-mono uppercase tracking-wider text-[11px]">
                                "Highlights & Architectural Additions:"
                            </div>
                            <ul class="space-y-2 text-mute pl-4 list-disc">
                                <li><strong class="text-paper">"HMAC-SHA256 Stateless Sessions:"</strong>" Tamper-proof cookie encoding with constant-time equality checks."</li>
                                <li><strong class="text-paper">"CSRF Guard Form Binding:"</strong>" Automatic hidden token generation and header injection preventing cross-site forgery."</li>
                                <li><strong class="text-paper">"Standalone Tailwind JIT:"</strong>" Automated binary caching eliminating Node.js, npm, and vite completely."</li>
                            </ul>
                        </div>
                        <div class="pt-4 border-t border-rule flex flex-wrap items-center justify-between text-xs font-mono text-dim gap-2">
                            <span>"crates.io: yav 0.1.2 · yav-cli 0.1.2"</span>
                            <a href="https://github.com/isaacvay/yav.rs/releases" target="_blank" rel="noreferrer" class="text-oxide hover:underline">
                                "View GitHub Release →"
                            </a>
                        </div>
                    </section>

                    // Release v0.1.0 & v0.1.1
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-dim">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.0 — Genesis"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-dim border border-rule">
                                    "INITIAL RELEASE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim">"July 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed">
                            "Initial open-source release of the Yav engine: procedural RSX declarative macros, Radix tree file-system router, and multi-stage distroless Docker scaffolding."
                        </p>
                    </section>
                </div>
            </div>
        </AppLayout>
    };

    website_page(
        "Releases & Engineering Changelog — Yav.rs",
        "Semantic versioning timeline, release notes, bug fixes, and feature additions for the Yav framework ecosystem.",
        "/releases",
        page,
    )
}
