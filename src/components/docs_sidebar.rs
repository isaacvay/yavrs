//! Documentation sidebar component with independent scrolling and live search filter.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn DocsSidebar() -> Node {
    rsx! {
        <aside class="hidden lg:block lg:col-span-3 sticky top-20 h-[calc(100vh-5.5rem)] select-none">
            <nav class="toc sidebar-scroll h-full overflow-y-auto overscroll-contain pr-3 space-y-6 text-xs" aria-label="Documentation navigation">
                
                // Live filter input for docs navigation
                <div class="relative pb-2">
                    <input 
                        id="doc-search-input" 
                        type="text" 
                        placeholder="Filter sections..." 
                        oninput="filterDocNav(this.value)"
                        class="w-full bg-sunk border border-rule rounded px-3 py-1.5 text-[12px] font-mono text-paper placeholder:text-dim outline-none focus:border-oxide transition-colors"
                    />
                    <span class="absolute right-2.5 top-2 text-[10px] text-faint font-mono">"ESC"</span>
                </div>

                <div class="doc-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "1. Getting Started"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#installation" class="doc-nav-item">"1.1 Prerequisites & Install"</a>
                        <a href="#quickstart" class="doc-nav-item">"1.2 Quickstart & Dev Server"</a>
                        <a href="#project-anatomy" class="doc-nav-item">"1.3 Project Anatomy"</a>
                        <a href="#manifest-reference" class="doc-nav-item">"1.4 package.yavml Manifest"</a>
                        <a href="#cli-toolbelt" class="doc-nav-item">"1.5 Yav CLI Toolbelt (20 cmds)"</a>
                    </div>
                </div>

                <div class="doc-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "2. Components & RSX"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#rsx-syntax" class="doc-nav-item">"2.1 RSX Macro & Syntax"</a>
                        <a href="#component-patterns" class="doc-nav-item">"2.2 #[component] & Props"</a>
                        <a href="#component-children" class="doc-nav-item">"2.3 Children & Nesting"</a>
                        <a href="#file-components" class="doc-nav-item">"2.4 .yavml File Components"</a>
                        <a href="#static-folding" class="doc-nav-item">"2.5 Static Subtree Folding"</a>
                        <a href="#native-control-flow" class="doc-nav-item">"2.6 Native if & for Loops"</a>
                    </div>
                </div>

                <div class="doc-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("server", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "3. Routing & Server"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#filesystem-routing" class="doc-nav-item">"3.1 Radix File Routing"</a>
                        <a href="#dynamic-routes" class="doc-nav-item">"3.2 Dynamic Params & Slugs"</a>
                        <a href="#server-architecture" class="doc-nav-item">"3.3 Axum 0.8 & Tokio Engine"</a>
                        <a href="#security-sessions" class="doc-nav-item">"3.4 HMAC Cryptographic Sessions"</a>
                        <a href="#security-hardening" class="doc-nav-item">"3.5 CSRF & Sliding Rate Limiter"</a>
                        <a href="#ssr-caching" class="doc-nav-item">"3.6 In-Memory SSR Caching"</a>
                        <a href="#view-transitions" class="doc-nav-item">"3.7 View Transitions & url_for!"</a>
                        <a href="#form-validation" class="doc-nav-item">"3.8 Form Validation & Hardening"</a>
                        <a href="#ai-mcp" class="doc-nav-item">"3.9 AI SDK & MCP Server"</a>
                        <a href="#resilience" class="doc-nav-item">"3.10 Resilience & XFetch"</a>
                        <a href="#passkeys-rebac" class="doc-nav-item">"3.11 Passkeys & ReBAC Graph"</a>
                        <a href="#modern-protocols" class="doc-nav-item">"3.12 GraphQL, Turbo & Tus"</a>
                    </div>
                </div>

                <div class="doc-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("layers", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "4. Interactivity & UI"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#zero-js-microactions" class="doc-nav-item">"4.1 Zero-JS Micro-Actions"</a>
                        <a href="#cursor-tracking" class="doc-nav-item">"4.2 Cursor Tracking & 3D Tilt"</a>
                        <a href="#motion-springs" class="doc-nav-item">"4.3 Spring Physics Motion"</a>
                        <a href="#micro-signals" class="doc-nav-item">"4.4 Declarative Signals"</a>
                        <a href="#theme-system" class="doc-nav-item">"4.5 Multi-Theme (8 Themes)"</a>
                        <a href="#tree-shaking" class="doc-nav-item">"4.6 SSR Tree-Shaking Runtimes"</a>
                        <a href="#htmx-interactivity" class="doc-nav-item">"4.7 HTMX & Partial Fragments"</a>
                        <a href="#icons-catalog" class="doc-nav-item">"4.8 13,000+ Icon System"</a>
                        <a href="#ui-scaffolding" class="doc-nav-item">"4.9 yav add (60+ Primitives)"</a>
                    </div>
                </div>

                <div class="doc-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("cpu", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "5. Production & Tooling"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#asset-pipeline" class="doc-nav-item">"5.1 Asset Pipeline & Brotli"</a>
                        <a href="#ssg-export" class="doc-nav-item">"5.2 Static Site Generation"</a>
                        <a href="#docker-deployment" class="doc-nav-item">"5.3 Docker & Cloud Deploy"</a>
                        <a href="#codegen-pwa" class="doc-nav-item">"5.4 Codegen & PWA Offline"</a>
                        <a href="#developer-diagnostics" class="doc-nav-item">"5.5 Diagnostics (yav doctor)"</a>
                        <a href="#database-migrations" class="doc-nav-item">"5.6 Database Migrations (yav db)"</a>
                    </div>
                </div>

                // Telemetry status badge
                <div class="plate bg-sunk p-3.5 text-[11.5px] font-mono text-dim space-y-1.5 border border-rule rounded">
                    <div class="text-paper flex items-center justify-between font-semibold">
                        <span class="flex items-center gap-2">
                            <span class="dot dot-ok"></span>"Zero Hydration Tax"
                        </span>
                        <span class="text-oxide">"0 kB JS"</span>
                    </div>
                    <p class="text-[11px] leading-relaxed text-mute">
                        "Native Rust compile-time templates running at 97% bare-metal HTTP throughput."
                    </p>
                </div>
            </nav>
        </aside>
    }
}
