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
                    // Release v0.1.5
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-ok">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.5"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-ok border border-rule" data-i18n="releases.tag.current">
                                    "CURRENT STABLE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim" data-i18n="releases.v015.date">"November 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed" data-i18n="releases.v015.summary">
                            "Milestone enterprise release introducing Model Context Protocol (MCP) server, native AI SDK with streaming UI & Hybrid RAG, WebAuthn Passkeys biometric authentication, 60+ enterprise UI components (Charts, DataGrid, Kanban with O(1) LexoRank, FlowCanvas), proactive resilience (CircuitBreaker, Bulkhead, Multi-Region DB Replicas), modern protocols (GraphQL, TurboStream, Tus v1.0.0 resumable uploads), real-time CRDT collaboration, O(L) Radix RouteTrie with parallel/intercepting slots, security audit remediations, and the complete 20-command yav CLI suite."
                        </p>
                        <div class="space-y-3 font-sans text-xs">
                            <div class="text-paper font-semibold font-mono uppercase tracking-wider text-[11px]" data-i18n="releases.highlights_lbl">
                                "Highlights & Architectural Additions:"
                            </div>
                            <ul class="space-y-2 text-mute pl-4 list-disc">
                                <li><strong class="text-paper" data-i18n="releases.v015.h1_t">"Model Context Protocol (MCP) & Native AI SDK:"</strong><span data-i18n="releases.v015.h1_d">" JSON-RPC 2.0 McpServer exposing live tools, resources, and prompt templates to AI agents (Claude Desktop, Cursor, Copilot), multi-turn Conversation, JSON-Schema ToolRegistry, stream_ui_fragment SSE streaming, cosine VectorIndex, and Reciprocal Rank Fusion (reciprocal_rank_fusion)."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h2_t">"WebAuthn / Passkeys Biometric Auth:"</strong><span data-i18n="releases.v015.h2_d">" Native passwordless FIDO2 authentication (Touch ID, Face ID, Windows Hello, YubiKey) with challenge verification and client scripts via PasskeyManager."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h3_t">"60+ Enterprise UI Components & Charts:"</strong><span data-i18n="releases.v015.h3_d">" Server-rendered SVG charts (LineChart, BarChart, DonutChart, Sparkline, Heatmap, Sankey, Radar), enterprise DataGrid, KanbanBoard with O(1) LexoRank, FlowCanvas with live Bézier curve rerouting, in-app DevTools HUD, Calendar/Scheduler, reactive Spreadsheet formula engine, and pure-Rust 3D ModelViewer."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h4_t">"Proactive Resilience & High Availability:"</strong><span data-i18n="releases.v015.h4_d">" CircuitBreaker with half-open recovery probes, Bulkhead isolation, AdaptiveConcurrencyLimiter, TransactionalOutbox pattern with event bus, and multi-region DbReplicaPool with Read-Your-Own-Writes session stickiness."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h5_t">"Modern Protocols in Pure Rust:"</strong><span data-i18n="releases.v015.h5_d">" Zero-dependency GraphQlSchema with embedded GraphiQL 3.x explorer, Hotwire-compatible TurboStream multi-zone DOM mutations (7 actions), Tus Protocol v1.0.0 resumable chunked uploads with disk spooling, and RFC 8291/8292 Web Push with VAPID."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h6_t">"Real-Time Collaboration & CRDTs:"</strong><span data-i18n="releases.v015.h6_d">" Conflict-free LwwRegister/LwwMap, multiplayer PresenceRoom cursor/selection tracking, LexoRank O(1) fractional ordering, StateMachine statecharts, and SagaWorkflow compensating transactions."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h7_t">"Multi-Tier Distributed Cache & XFetch:"</strong><span data-i18n="releases.v015.h7_d">" L1 in-memory + L2 distributed cache with probabilistic early expiration (XFetch) to completely eliminate cache stampedes, paired with cluster-wide DistributedLock and leader election."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h8_t">"O(L) Radix Route Trie & App Router Slots:"</strong><span data-i18n="releases.v015.h8_d">" Constant O(L) route resolution matching file routing, Parallel Routes (@slot), Intercepting Routes ((.) / (..) / (...)), Optional Catch-All ([[...slug]]), and auto-wiring #[server] Server Actions."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v015.h9_t">"Security Remediations, i18n Lint & 20 CLI Commands:"</strong><span data-i18n="releases.v015.h9_d">" Full yav db suite with SQLite and PostgreSQL runners, Up/Down migrations, SHA-256 tamper-evident journal checks, static site export (yav export), fuzzy search indexing, and security audit hardenings."</span></li>
                            </ul>
                        </div>
                        <div class="pt-4 border-t border-rule flex flex-wrap items-center justify-between text-xs font-mono text-dim gap-2">
                            <span>"crates.io: yav 0.1.5 · yav-cli 0.1.5"</span>
                            <a href="https://github.com/isaacvay/yav.rs/releases" target="_blank" rel="noreferrer" class="text-oxide hover:underline" data-i18n="releases.view_github">
                                "View GitHub Release →"
                            </a>
                        </div>
                    </section>

                    // Release v0.1.4
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-oxide">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.4"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-dim border border-rule" data-i18n="releases.tag.stable">
                                    "STABLE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim" data-i18n="releases.v014.date">"October 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed" data-i18n="releases.v014.summary">
                            "Design & motion runtime update delivering Zero-JS spring physics, GPU cursor effects, an 8-theme OKLCH token engine with anti-FOUC injection, declarative yav:* micro-actions, and complete developer documentation."
                        </p>
                        <div class="space-y-3 font-sans text-xs">
                            <div class="text-paper font-semibold font-mono uppercase tracking-wider text-[11px]" data-i18n="releases.highlights_lbl">
                                "Highlights & Architectural Additions:"
                            </div>
                            <ul class="space-y-2 text-mute pl-4 list-disc">
                                <li><strong class="text-paper" data-i18n="releases.v014.h1_t">"Zero-JS Spring Motion Engine (yav::ui::motion):"</strong><span data-i18n="releases.v014.h1_d">" CSS linear() easing curves and GPU-accelerated ScrollReveal, StaggerList, TextReveal, Parallax, and KineticMarquee components."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v014.h2_t">"Cursor Engine & Tactile Micro-Actions:"</strong><span data-i18n="releases.v014.h2_d">" CursorAura, MagneticButton, SpotlightGrid, and zero-boilerplate yav:toggle, yav:copy, yav:hotkey, and yav:tab directives."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v014.h3_t">"8-Theme System & Anti-FOUC Injection:"</strong><span data-i18n="releases.v014.h3_d">" Pre-calibrated Carbon, Nord, Rose Pine, Brutalist, Tokyo Night, Latte, Catppuccin, and Dracula themes with instant dark-first bootstrap."</span></li>
                            </ul>
                        </div>
                        <div class="pt-4 border-t border-rule flex flex-wrap items-center justify-between text-xs font-mono text-dim gap-2">
                            <span>"crates.io: yav 0.1.4 · yav-cli 0.1.4"</span>
                            <a href="https://github.com/isaacvay/yav.rs/releases" target="_blank" rel="noreferrer" class="text-oxide hover:underline" data-i18n="releases.view_github">
                                "View GitHub Release →"
                            </a>
                        </div>
                    </section>

                    // Release v0.1.3
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-oxide">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.3"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-dim border border-rule" data-i18n="releases.tag.stable">
                                    "STABLE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim" data-i18n="releases.v013.date">"October 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed" data-i18n="releases.v013.summary">
                            "Major server engine modernization introducing Axum 0.8.1 foundation, hyper 1.0 high-concurrency primitives, and the embedded 13,000+ vector icon system with zero network requests."
                        </p>
                        <div class="space-y-3 font-sans text-xs">
                            <div class="text-paper font-semibold font-mono uppercase tracking-wider text-[11px]" data-i18n="releases.highlights_lbl">
                                "Highlights & Architectural Additions:"
                            </div>
                            <ul class="space-y-2 text-mute pl-4 list-disc">
                                <li><strong class="text-paper" data-i18n="releases.v013.h1_t">"Axum 0.8 & hyper 1.0 Foundation:"</strong><span data-i18n="releases.v013.h1_d">" Complete refactor of internal router pipeline for zero-copy buffer streaming and lock-free concurrency."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v013.h2_t">"Embedded 13,000+ Icon Engine (yav-icons):"</strong><span data-i18n="releases.v013.h2_d">" Lucide, MynaUI, and Phosphor sets bundled directly into the binary with size enums."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v013.h3_t">"Sliding-Window Rate Limiting:"</strong><span data-i18n="releases.v013.h3_d">" Cryptographic IP and token-bucket protection against DDoS and automated credential stuffing."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v013.h4_t">"Zero-Tax Pre-Compression:"</strong><span data-i18n="releases.v013.h4_d">" Built-in Brotli and Gzip archive generation during release builds with Blake3 ETags."</span></li>
                            </ul>
                        </div>
                        <div class="pt-4 border-t border-rule flex flex-wrap items-center justify-between text-xs font-mono text-dim gap-2">
                            <span>"crates.io: yav 0.1.3 · yav-cli 0.1.3"</span>
                            <a href="https://github.com/isaacvay/yav.rs/releases" target="_blank" rel="noreferrer" class="text-oxide hover:underline" data-i18n="releases.view_github">
                                "View GitHub Release →"
                            </a>
                        </div>
                    </section>

                    // Release v0.1.2
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-oxide">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.2"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-dim border border-rule" data-i18n="releases.tag.stable">
                                    "STABLE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim" data-i18n="releases.v012.date">"September 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed" data-i18n="releases.v012.summary">
                            "Security hardening update bringing stateless cryptographic sessions, constant-time CSRF token verification, and signed Flash notification cookies."
                        </p>
                        <div class="space-y-3 font-sans text-xs">
                            <div class="text-paper font-semibold font-mono uppercase tracking-wider text-[11px]" data-i18n="releases.highlights_lbl">
                                "Highlights & Architectural Additions:"
                            </div>
                            <ul class="space-y-2 text-mute pl-4 list-disc">
                                <li><strong class="text-paper" data-i18n="releases.v012.h1_t">"HMAC-SHA256 Stateless Sessions:"</strong><span data-i18n="releases.v012.h1_d">" Tamper-proof cookie encoding with constant-time equality checks."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v012.h2_t">"CSRF Guard Form Binding:"</strong><span data-i18n="releases.v012.h2_d">" Automatic hidden token generation and header injection preventing cross-site forgery."</span></li>
                                <li><strong class="text-paper" data-i18n="releases.v012.h3_t">"Standalone Tailwind JIT:"</strong><span data-i18n="releases.v012.h3_d">" Automated binary caching eliminating Node.js, npm, and vite completely."</span></li>
                            </ul>
                        </div>
                        <div class="pt-4 border-t border-rule flex flex-wrap items-center justify-between text-xs font-mono text-dim gap-2">
                            <span>"crates.io: yav 0.1.2 · yav-cli 0.1.2"</span>
                            <a href="https://github.com/isaacvay/yav.rs/releases" target="_blank" rel="noreferrer" class="text-oxide hover:underline" data-i18n="releases.view_github">
                                "View GitHub Release →"
                            </a>
                        </div>
                    </section>

                    // Release v0.1.0 & v0.1.1
                    <section class="plate p-6 sm:p-8 bg-ink-2 space-y-6 relative border-l-2 border-l-dim">
                        <div class="flex flex-wrap items-center justify-between gap-4">
                            <div class="flex items-center gap-3">
                                <span class="text-2xl font-bold font-mono text-paper">"v0.1.0 — Genesis"</span>
                                <span class="px-2.5 py-0.5 text-[11px] font-mono font-semibold bg-sunk text-dim border border-rule" data-i18n="releases.tag.initial">
                                    "INITIAL RELEASE"
                                </span>
                            </div>
                            <span class="text-xs font-mono text-dim" data-i18n="releases.v010.date">"July 2026"</span>
                        </div>
                        <p class="text-mute text-[14.5px] leading-relaxed" data-i18n="releases.v010.summary">
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
