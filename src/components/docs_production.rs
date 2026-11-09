//! Documentation Section 5: Production & Tooling (Asset Pipeline & Brotli, SSG, Docker, Multi-Cloud Deploy, Codegen, PWA, Diagnostics, Doc Pagination)

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn DocsProduction() -> Node {
    rsx! {
        <div class="space-y-20">
            // =========================================================================
            // 5.1 ASSET PIPELINE & BROTLI
            // =========================================================================
            <section id="asset-pipeline" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"5.1"</span>
                    <span class="t">"Asset Pipeline & In-Memory Pre-Compression"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"brotli + gzip"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Blake3 ETags and zero on-the-fly CPU compression tax."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "During "
                    <code class="inline">"yav build"</code>
                    ", static assets are fingerprinted ("
                    <code class="inline">"styles.[hash].css"</code>
                    ") and pre-compressed into Brotli ("
                    <code class="inline">".br"</code>
                    ") and Gzip ("
                    <code class="inline">".gz"</code>
                    ") archives in a single in-memory pass. The server streams pre-compressed buffers directly from memory matching the client's "
                    <code class="inline">"Accept-Encoding"</code>
                    " header."
                </p>

                <div class="grid sm:grid-cols-2 gap-4 text-xs font-mono">
                    <div class="plate p-5 bg-ink-2 space-y-2">
                        <div class="text-oxide font-bold flex items-center gap-2">
                            {render_icon("zap", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                            "Blake3 ETags & 304 Not Modified"
                        </div>
                        <p class="text-mute text-[12px] leading-relaxed">
                            "All assets calculate ultra-fast Blake3 hashes. Repeat client visits receive 304 headers in microseconds without redundant byte transmissions."
                        </p>
                    </div>
                    <div class="plate p-5 bg-ink-2 space-y-2">
                        <div class="text-oxide font-bold flex items-center gap-2">
                            {render_icon("shield-check", IconSize::Sm, Some("w-4 h-4 text-ok"), None)}
                            "Strict Path Traversal Hardening"
                        </div>
                        <p class="text-mute text-[12px] leading-relaxed">
                            "Relative path escapes (../) and null bytes are rejected at the kernel interface; canonical paths are bounded strictly within public/."
                        </p>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 5.2 STATIC SITE GENERATION (SSG)
            // =========================================================================
            <section id="ssg-export" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"5.2"</span>
                    <span class="t">"Static Site Generation (SSG with yav export)"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav export"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Pre-render all routes to static HTML for zero-server hosting."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "If your project consists of documentation, marketing pages, or blogs, "
                    <code class="inline">"yav export"</code>
                    " crawls and renders all discovered static routes into standalone HTML files ready to deploy on Cloudflare Pages, GitHub Pages, Netlify, or AWS S3."
                </p>

                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <span class="text-[11px] text-paper font-mono font-medium">"SSG EXPORT"</span>
                            <span class="text-[10px] text-oxide font-mono">"yav export"</span>
                        </div>
                        <button data-copy="yav export --output dist --base-url https://yav.rs" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#070709]">
                        <div class="token-comment">"# Export all discovered static routes into dist/ directory"</div>
                        <div><span class="text-oxide">"$ "</span>"yav export"</div>
                        <div class="token-comment pt-2">"# Specify custom output path and production base URL for canonical tags"</div>
                        <div><span class="text-oxide">"$ "</span>"yav export --output dist --base-url https://mon-site.com"</div>
                    </div>
                    <div class="ide-status">
                        <span>"Generates directory-structured index.html files"</span>
                        <span class="text-ok">"Zero-Server Static Output"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 5.3 MULTI-STAGE DOCKER & MULTI-CLOUD DEPLOY
            // =========================================================================
            <section id="docker-deployment" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"5.3"</span>
                    <span class="t">"Distroless Docker & Multi-Cloud Deployment"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"sub-30MB"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Deploy anywhere. One statically-linked native binary."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Run "
                    <code class="inline">"yav docker"</code>
                    " to generate a hardened multi-stage Dockerfile that caches cargo dependencies and yields a production image under 30 MB running as an unprivileged user, or use "
                    <code class="inline">"yav deploy"</code>
                    " for platform-specific configurations."
                </p>

                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <span class="text-[11px] text-paper font-mono font-medium">"CLOUD DEPLOY SCAFFOLDING"</span>
                            <span class="text-[10px] text-oxide font-mono">"yav deploy"</span>
                        </div>
                        <button data-copy="yav deploy fly\nyav deploy shuttle\nyav deploy railway\nyav deploy systemd" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#070709]">
                        <div class="token-comment">"# Scaffold configuration for Fly.io (generates fly.toml and multi-stage Dockerfile)"</div>
                        <div><span class="text-oxide">"$ "</span>"yav deploy fly"</div>
                        <div class="token-comment pt-2">"# Scaffold Shuttle.rs serverless Rust configuration"</div>
                        <div><span class="text-oxide">"$ "</span>"yav deploy shuttle"</div>
                        <div class="token-comment pt-2">"# Scaffold Railway.app configuration (railway.json & Procfile)"</div>
                        <div><span class="text-oxide">"$ "</span>"yav deploy railway"</div>
                        <div class="token-comment pt-2">"# Scaffold hardened Linux Systemd service unit"</div>
                        <div><span class="text-oxide">"$ "</span>"yav deploy systemd"</div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 5.4 CODEGEN & PWA
            // =========================================================================
            <section id="codegen-pwa" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"5.4"</span>
                    <span class="t">"TypeScript / OpenAPI Codegen & PWA Offline"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"codegen"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Cross-boundary type-safety and instant offline caching."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Bridge backend models to frontend consumers effortlessly with automatic TypeScript type definitions or export full OpenAPI 3.1 specifications. One command scaffolds full PWA offline support."
                </p>

                <div class="grid md:grid-cols-2 gap-6">
                    <div class="term-window">
                        <div class="term-head">
                            <span class="text-[11px] text-paper font-mono font-medium">"CLIENT CODEGEN"</span>
                            <span class="text-[10px] text-oxide font-mono">"yav codegen"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] space-y-1 bg-[#070709]">
                            <div class="token-comment">"# Export TypeScript types"</div>
                            <div><span class="text-oxide">"$ "</span>"yav codegen --ts"</div>
                            <div class="pt-2 token-comment">"# Export OpenAPI 3.1 schema"</div>
                            <div><span class="text-oxide">"$ "</span>"yav codegen --openapi"</div>
                        </div>
                    </div>

                    <div class="term-window">
                        <div class="term-head">
                            <span class="text-[11px] text-paper font-mono font-medium">"PROGRESSIVE WEB APP"</span>
                            <span class="text-[10px] text-ok font-mono">"yav pwa"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] space-y-1 bg-[#070709]">
                            <div class="token-comment">"# Scaffold manifest & service worker"</div>
                            <div><span class="text-oxide">"$ "</span>"yav pwa"</div>
                            <div class="pt-2 text-mute text-[11px] leading-relaxed">
                                "Generates manifest.json, sw.js with cache-first static strategy, and offline fallback."
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 5.5 DEVELOPER DIAGNOSTICS & ROUTE AUDIT
            // =========================================================================
            <section id="developer-diagnostics" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"5.5"</span>
                    <span class="t">"Developer Diagnostics & Route Inspection"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav doctor"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Full system auditing and tabular route introspection."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Diagnose toolchain discrepancies with "
                    <code class="inline">"yav doctor"</code>
                    " or inspect your entire file-based route hierarchy, handlers, and active layout chains with "
                    <code class="inline">"yav routes"</code>
                    "."
                </p>

                <div class="grid md:grid-cols-2 gap-6">
                    <div class="term-window">
                        <div class="term-head">
                            <span class="text-[11px] text-paper font-mono font-medium">"SYSTEM AUDIT"</span>
                            <span class="text-[10px] text-oxide font-mono">"yav doctor"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] space-y-1 bg-[#070709]">
                            <div class="token-comment">"# Run health audit on toolchains, tailwind, git, and port"</div>
                            <div><span class="text-oxide">"$ "</span>"yav doctor"</div>
                            <div class="pt-2 text-ok text-[11px]">"✓ Rust toolchain: 1.75+ (stable)"</div>
                            <div class="text-ok text-[11px]">"✓ Manifest: package.yavml (valid)"</div>
                            <div class="text-ok text-[11px]">"✓ Standalone Tailwind binary cached"</div>
                            <div class="text-ok text-[11px]">"✓ Port 3000: available"</div>
                        </div>
                    </div>

                    <div class="term-window">
                        <div class="term-head">
                            <span class="text-[11px] text-paper font-mono font-medium">"ROUTE INSPECTION"</span>
                            <span class="text-[10px] text-ok font-mono">"yav routes"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] space-y-1 bg-[#070709]">
                            <div class="token-comment">"# Inspect file-based routes table in terminal"</div>
                            <div><span class="text-oxide">"$ "</span>"yav routes"</div>
                            <div class="pt-2 text-paper text-[11px]">"GET  /              -> src/routes/index.rs"</div>
                            <div class="text-paper text-[11px]">"GET  /about         -> src/routes/about.rs"</div>
                            <div class="text-paper text-[11px]">"GET  /users/:id     -> src/routes/users/_id_.rs"</div>
                            <div class="text-paper text-[11px]">"GET  /docs/*slug    -> src/routes/docs/__slug__.rs"</div>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 5.6 DATABASE MIGRATIONS & SCAFFOLDING (YAV DB)
            // =========================================================================
            <section id="database-migrations" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"5.6"</span>
                    <span class="t">"Database Migrations & SQL Scaffolding (yav db)"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav db"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Deterministic SQL migrations with SHA-256 cryptographic journaling."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "The "
                    <code class="inline">"yav db"</code>
                    " command provides an integrated migration runner for SQLite and PostgreSQL. It reads your database URL from "
                    <code class="inline">".env"</code>
                    ", applies forward migrations, enables one-command rollbacks, and verifies SHA-256 checksums in the "
                    <code class="inline">"_yav_migrations"</code>
                    " journal to prevent migration drift across environments."
                </p>

                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <span class="text-[11px] text-paper font-mono font-medium">"DATABASE WORKFLOW"</span>
                            <span class="text-[10px] text-oxide font-mono">"yav db"</span>
                        </div>
                        <button data-copy="yav db init\nyav db new create_users\nyav db migrate\nyav db status\nyav db rollback" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#070709]">
                        <div class="token-comment">"# 1. Initialize migrations directory and configure DATABASE_URL in .env"</div>
                        <div><span class="text-oxide">"$ "</span>"yav db init"</div>
                        <div class="token-comment pt-2">"# 2. Create a timestamped SQL migration with UP and DOWN sections"</div>
                        <div><span class="text-oxide">"$ "</span>"yav db new create_users_table"</div>
                        <div class="token-comment pt-2">"# 3. Execute all pending migrations and commit SHA-256 hashes to journal"</div>
                        <div><span class="text-oxide">"$ "</span>"yav db migrate"</div>
                        <div class="token-comment pt-2">"# 4. Check migration status, history, and checksum integrity"</div>
                        <div><span class="text-oxide">"$ "</span>"yav db status"</div>
                        <div class="token-comment pt-2">"# 5. Rollback the most recent migration step"</div>
                        <div><span class="text-oxide">"$ "</span>"yav db rollback"</div>
                    </div>
                    <div class="ide-status">
                        <span>"Native SQLite & PostgreSQL runners"</span>
                        <span class="text-ok">"SHA-256 Journaled"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // BOTTOM DOC PAGINATION CARDS
            // =========================================================================
            <div class="pt-12 border-t border-rule grid sm:grid-cols-2 gap-6">
                <a href="/about" class="plate p-6 bg-sunk hover:border-oxide/60 transition-colors group block space-y-2">
                    <div class="text-[11px] font-mono text-dim flex items-center gap-2 group-hover:text-oxide transition-colors">
                        {render_icon("arrow-left", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "Previous Architectural Guide"
                    </div>
                    <div class="text-paper font-medium text-lg">"Architecture & Performance"</div>
                    <p class="text-mute text-[13px] leading-relaxed">
                        "Explore the memory layout, Tokio thread allocation, and zero-lock runtime engine."
                    </p>
                </a>
                <a href="/components" class="plate p-6 bg-sunk hover:border-oxide/60 transition-colors group block space-y-2 text-right">
                    <div class="text-[11px] font-mono text-dim flex items-center justify-end gap-2 group-hover:text-oxide transition-colors">
                        "Next Interactive Guide"
                        {render_icon("arrow-right", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                    </div>
                    <div class="text-paper font-medium text-lg">"UI Components & Icons"</div>
                    <p class="text-mute text-[13px] leading-relaxed">
                        "Browse interactive widgets, form controls, copy-paste components, and icon search."
                    </p>
                </a>
            </div>
        </div>
    }
}
