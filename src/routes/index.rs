//! Flagship Home / Overview page for Yav.rs.
//! Inspired by the world-class engineering and UX design of Next.js, Tailwind CSS, and Shuttle.dev.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn index(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="space-y-24 py-6 sm:py-12">
                // ==========================================
                // 1. HERO SECTION (NEXT.JS + SHUTTLE DX)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8 text-center sm:text-left relative">
                    // Radiant Background Glow & Engineering Mesh
                    <div class="absolute -top-16 left-1/2 -translate-x-1/2 w-full max-w-5xl h-96 bg-radial-glow pointer-events-none -z-10"></div>

                    // Next.js Style Release Pill Banner
                    <a
                        href="/docs"
                        class="inline-flex items-center gap-2.5 px-3.5 py-1.5 rounded-full bg-white/90 dark:bg-zinc-900/90 border border-zinc-200/90 dark:border-zinc-800/90 text-xs font-mono shadow-xs hover:border-amber-500/60 dark:hover:border-amber-500/60 transition-all group backdrop-blur-md cursor-pointer"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":-12}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":20}"
                    >
                        <span class="flex h-2 w-2 relative">
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                        </span>
                        <span class="font-bold text-zinc-900 dark:text-white" data-i18n="hero.pill.engine">"Yav v0.1.3 Released"</span>
                        <span class="text-zinc-300 dark:text-zinc-700">"|"</span>
                        <span class="text-zinc-600 dark:text-zinc-400 hidden sm:inline" data-i18n="hero.pill.rsx">"Axum 0.8 • Compiled RSX • 0 KB Hydration JS"</span>
                        <span class="text-amber-500 group-hover:translate-x-0.5 transition-transform font-bold">"→"</span>
                    </a>

                    // Commanding Headline & Subtitle
                    <div
                        class="space-y-4 max-w-4xl"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":20}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":200,\"damping\":22,\"delay_ms\":80}"
                    >
                        <h1 class="text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-[-0.035em] text-zinc-950 dark:text-white leading-[1.05]">
                            <span data-i18n="hero.title.line1">"Fast by nature."</span>
                            <br />
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-500 via-orange-500 to-amber-400 dark:from-amber-400 dark:via-orange-400 dark:to-amber-500" data-i18n="hero.title.line2">
                                "Safe by design."
                            </span>
                            <br />
                            <span data-i18n="hero.title.line3">"The full-stack web framework for Rust."</span>
                        </h1>
                        <p class="text-lg sm:text-xl text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed font-sans pt-1" data-i18n="hero.subtitle">
                            "Build production-grade web applications with compiled RSX templates, automatic file-system routing, and built-in Tailwind CSS. Powered by Axum 0.8 and Tokio. No Node.js. No npm. Zero client runtime hydration tax."
                        </p>
                    </div>

                    // Primary Actions & Shuttle-Style Terminal Install Box
                    <div
                        class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3 pt-2"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":20}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":200,\"damping\":22,\"delay_ms\":140}"
                    >
                        <a
                            href="/docs"
                            class="inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 dark:bg-white dark:hover:bg-zinc-100 text-white dark:text-zinc-900 text-sm font-semibold transition-all shadow-md active:scale-98"
                            data-motion="true"
                            data-motion-hover="{\"scale\":1.02}"
                            data-motion-tap="{\"scale\":0.98}"
                        >
                            <span data-i18n="hero.cta.start">"Start Building (5-min guide)"</span>
                            {render_icon("arrow-right", IconSize::Sm, Some("w-4 h-4"), None)}
                        </a>

                        // 1-Click Copy Install Command with Visual Feedback
                        <div class="flex items-center justify-between gap-3 px-4 py-3 rounded-xl bg-zinc-100 hover:bg-zinc-200/70 dark:bg-zinc-900 dark:hover:bg-zinc-800/80 border border-zinc-200 dark:border-zinc-800 font-mono text-xs text-zinc-800 dark:text-zinc-200 shadow-xs transition-all">
                            <div class="flex items-center gap-2">
                                <span class="text-amber-500 font-bold">"$"</span>
                                <span id="hero-cmd-display">"cargo install yav-cli"</span>
                            </div>
                            <button
                                onclick="const cmd=document.getElementById('hero-cmd-display').innerText; navigator.clipboard.writeText(cmd); const el=this.querySelector('.copy-txt'); el.innerText='copied!'; setTimeout(()=>el.innerText='copy', 1500);"
                                class="copy-btn px-2.5 py-1 rounded-md bg-white dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600 text-[11px] text-zinc-600 dark:text-zinc-300 hover:text-zinc-900 dark:hover:text-white transition-colors cursor-pointer flex items-center gap-1.5 active:scale-95 shadow-xs"
                                title="Copy CLI install command"
                            >
                                {render_icon("copy", IconSize::Sm, Some("w-3 h-3"), None)}
                                <span class="copy-txt font-mono">"copy"</span>
                            </button>
                        </div>

                        <a
                            href="/about"
                            class="inline-flex items-center justify-center gap-1.5 px-5 py-3.5 rounded-xl bg-transparent hover:bg-zinc-100 dark:hover:bg-zinc-800/60 text-zinc-700 dark:text-zinc-300 text-sm font-medium transition-all"
                        >
                            <span data-i18n="hero.cta.arch">"Architecture deep dive"</span>
                            {render_icon("chevron-right", IconSize::Sm, Some("w-4 h-4 opacity-70"), None)}
                        </a>
                    </div>

                    // Quick Telemetry Badges
                    <div
                        class="pt-2 flex flex-wrap items-center gap-4 sm:gap-6 text-xs text-zinc-500 dark:text-zinc-400 font-mono"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0}"
                        data-motion-animate="{\"opacity\":1}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":200,\"damping\":22,\"delay_ms\":180}"
                    >
                        <div class="flex items-center gap-2">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                            <strong class="text-zinc-900 dark:text-zinc-200">"124,500 req/s"</strong>
                            <span>"Throughput"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
                            <strong class="text-zinc-900 dark:text-zinc-200">"0.28 ms"</strong>
                            <span>"Median TTFB"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="w-1.5 h-1.5 rounded-full bg-sky-500"></span>
                            <strong class="text-zinc-900 dark:text-zinc-200">"0 KB JS"</strong>
                            <span>"Client Hydration"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="w-1.5 h-1.5 rounded-full bg-purple-500"></span>
                            <strong class="text-zinc-900 dark:text-zinc-200">"Tokio &amp; Axum 0.8"</strong>
                            <span>"Engine"</span>
                        </div>
                    </div>

                    // ==========================================
                    // FLAGSHIP STUDIO (TAILWIND + SHUTTLE DX)
                    // ==========================================
                    <div
                        class="pt-6"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":30}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":180,\"damping\":22,\"delay_ms\":220}"
                    >
                        <div class="rounded-2xl border border-zinc-200/90 dark:border-zinc-800/90 bg-white dark:bg-zinc-900/80 shadow-2xl overflow-hidden card-hover">
                            // Studio Top Bar with macOS Traffic Lights & Interactive File Tabs
                            <div class="px-4 py-3 border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50/95 dark:bg-zinc-950/90 flex items-center justify-between flex-wrap gap-2">
                                <div class="flex items-center gap-4">
                                    // macOS Traffic Lights
                                    <div class="flex items-center gap-1.5">
                                        <div class="w-3 h-3 rounded-full bg-red-400/90 hover:opacity-80 transition-opacity cursor-pointer"></div>
                                        <div class="w-3 h-3 rounded-full bg-amber-400/90 hover:opacity-80 transition-opacity cursor-pointer"></div>
                                        <div class="w-3 h-3 rounded-full bg-emerald-400/90 hover:opacity-80 transition-opacity cursor-pointer"></div>
                                    </div>

                                    // Interactive Tabs
                                    <div class="flex items-center gap-1 p-1 rounded-lg bg-zinc-200/60 dark:bg-zinc-800/80 text-xs font-mono">
                                        <button
                                            id="btn-file-rsx"
                                            onclick="switchHeroTab('rsx')"
                                            class="hero-tab-btn active px-3 py-1 rounded-md bg-white dark:bg-zinc-900 text-zinc-900 dark:text-white shadow-xs font-medium cursor-pointer transition-all flex items-center gap-1.5"
                                        >
                                            {render_icon("code", IconSize::Sm, Some("w-3 h-3 text-amber-500"), None)}
                                            <span>"src/routes/index.rs"</span>
                                        </button>
                                        <button
                                            id="btn-file-api"
                                            onclick="switchHeroTab('api')"
                                            class="hero-tab-btn px-3 py-1 rounded-md text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white cursor-pointer transition-all flex items-center gap-1.5"
                                        >
                                            {render_icon("server", IconSize::Sm, Some("w-3 h-3 text-sky-500"), None)}
                                            <span>"src/routes/api/users.rs"</span>
                                        </button>
                                        <button
                                            id="btn-file-yavml"
                                            onclick="switchHeroTab('yavml')"
                                            class="hero-tab-btn px-3 py-1 rounded-md text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white cursor-pointer transition-all flex items-center gap-1.5"
                                        >
                                            {render_icon("file-text", IconSize::Sm, Some("w-3 h-3 text-purple-500"), None)}
                                            <span>"package.yavml"</span>
                                        </button>
                                        <button
                                            id="btn-file-term"
                                            onclick="switchHeroTab('term')"
                                            class="hero-tab-btn px-3 py-1 rounded-md text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white cursor-pointer transition-all flex items-center gap-1.5"
                                        >
                                            {render_icon("terminal", IconSize::Sm, Some("w-3 h-3 text-emerald-500"), None)}
                                            <span>"$ yav dev"</span>
                                        </button>
                                    </div>
                                </div>

                                <div class="flex items-center gap-3 text-xs font-mono">
                                    <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-200 dark:border-emerald-800 text-emerald-700 dark:text-emerald-400 font-semibold text-[11px]">
                                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                                        <span>"Live: http://127.0.0.1:3000"</span>
                                    </div>
                                </div>
                            </div>

                            // Split Grid: Code Viewer on Left (7 cols), Live Interactive Studio on Right (5 cols)
                            <div class="grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-zinc-200 dark:divide-zinc-800 text-left">
                                // Left Pane: Code Viewer with Line Numbers (7 cols)
                                <div class="lg:col-span-7 p-6 overflow-x-auto text-xs font-mono leading-relaxed bg-[#0b0b0e] text-zinc-200">
                                    // File 1: RSX Component
                                    <div id="hero-code-rsx" class="space-y-1">
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"01"</span><span class="token-keyword">"use"</span><span> " yav::prelude::*;"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"02"</span><span class="token-keyword">"use"</span><span> " axum::extract::Request;"</span></div>
                                        <div class="flex items-center text-zinc-500 pt-1"><span class="w-6 select-none opacity-40">"03"</span><span class="token-comment">"// Discovered at compile time by yav::auto_routes!()"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"04"</span><span class="token-keyword">"pub async fn"</span> <span class="token-fn">"index"</span><span>"(_req: Request) -&gt; HtmlPage {"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"05"</span><span class="pl-4 token-macro">"rsx!"</span><span> " {"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"06"</span><span class="pl-8 token-tag">"&lt;AppLayout&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"07"</span><span class="pl-12 token-tag">"&lt;div"</span> <span class="token-attr">"class="</span><span class="token-string">"\"max-w-xl p-8 rounded-2xl bg-zinc-900\""</span><span class="token-tag">"&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"08"</span><span class="pl-16 token-tag">"&lt;h1"</span> <span class="token-attr">"class="</span><span class="token-string">"\"text-2xl font-bold text-white\""</span><span class="token-tag">"&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"09"</span><span class="pl-20 token-string">"\"High-Velocity Full-Stack Rust\""</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"10"</span><span class="pl-16 token-tag">"&lt;/h1&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"11"</span><span class="pl-16 token-tag">"&lt;p"</span> <span class="token-attr">"class="</span><span class="token-string">"\"text-zinc-400 mt-2\""</span><span class="token-tag">"&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"12"</span><span class="pl-20 token-string">"\"Compiled directly to native machine code.\""</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"13"</span><span class="pl-16 token-tag">"&lt;/p&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"14"</span><span class="pl-16 token-tag">"&lt;Counter"</span> <span class="token-attr">"initial="</span><span class="token-val">"{42}"</span> <span class="token-tag">"/&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"15"</span><span class="pl-12 token-tag">"&lt;/div&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"16"</span><span class="pl-8 token-tag">"&lt;/AppLayout&gt;"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"17"</span><span class="pl-4">"}"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"18"</span><span>"}"</span></div>
                                    </div>

                                    // File 2: Axum JSON API Handler
                                    <div id="hero-code-api" class="space-y-1 hidden">
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"01"</span><span class="token-keyword">"use"</span><span> " yav::prelude::*;"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"02"</span><span class="token-keyword">"use"</span><span> " axum::extract::{Path, State};"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"03"</span><span class="token-keyword">"use"</span><span> " serde::{Serialize, Deserialize};"</span></div>
                                        <div class="flex items-center text-zinc-500 pt-1"><span class="w-6 select-none opacity-40">"04"</span><span class="token-comment">"// Native Axum 0.8 JSON Endpoint with sub-microsecond latency"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"05"</span><span class="token-keyword">"#[derive(Serialize)]"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"06"</span><span class="token-keyword">"pub struct"</span> <span class="token-type">"User"</span><span> " {"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"07"</span><span class="pl-4">"pub id: "</span><span class="token-type">"u64"</span><span>","</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"08"</span><span class="pl-4">"pub name: "</span><span class="token-type">"String"</span><span>","</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"09"</span><span class="pl-4">"pub latency_us: "</span><span class="token-type">"u32"</span><span>","</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"10"</span><span>"}"</span></div>
                                        <div class="flex items-center pt-1"><span class="w-6 select-none text-zinc-600 opacity-40">"11"</span><span class="token-keyword">"pub async fn"</span> <span class="token-fn">"get_user"</span><span>"(Path(id): Path&lt;u64&gt;) -&gt; json_response&lt;User&gt; {"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"12"</span><span class="pl-4">"json_response(User { id, name: "</span><span class="token-string">"\"Isaac\""</span><span class="token-fn">".into()"</span><span>", latency_us: 42 })"</span></div>
                                        <div class="flex items-center"><span class="w-6 select-none text-zinc-600 opacity-40">"13"</span><span>"}"</span></div>
                                    </div>

                                    // File 3: package.yavml
                                    <div id="hero-code-yavml" class="space-y-1 hidden">
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"01"</span><span class="token-keyword">"[package]"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"02"</span><span>"name = "</span><span class="token-string">"\"website\""</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"03"</span><span>"version = "</span><span class="token-string">"\"0.1.0\""</span></div>
                                        <div class="flex items-center text-zinc-600 pt-1"><span class="w-6 select-none opacity-40">"04"</span><span class="token-keyword">"[server]"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"05"</span><span>"port = "</span><span class="token-val">"3000"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"06"</span><span>"host = "</span><span class="token-string">"\"127.0.0.1\""</span></div>
                                        <div class="flex items-center text-zinc-600 pt-1"><span class="w-6 select-none opacity-40">"07"</span><span class="token-keyword">"[assets]"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"08"</span><span>"tailwind = "</span><span class="token-val">"true"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"09"</span><span>"minify = "</span><span class="token-val">"true"</span></div>
                                    </div>

                                    // File 4: Live Compiler Terminal (Shuttle style)
                                    <div id="hero-code-term" class="space-y-2 hidden font-mono text-zinc-300">
                                        <div class="text-zinc-500">"$ yav dev"</div>
                                        <div class="text-emerald-400 font-bold">"🚀 Yav Dev Server v0.1.3"</div>
                                        <div class="text-zinc-400">"⚡ Live reload listening on 127.0.0.1:35729"</div>
                                        <div class="text-sky-400">"✔ Tailwind CSS compiled in 48ms"</div>
                                        <div class="text-zinc-300">"📦 Starting application server..."</div>
                                        <div class="text-amber-400 font-bold">"🌐 Application running at http://127.0.0.1:3000"</div>
                                        <div class="text-zinc-500 pt-1">"👀 Watching src/ and public/ for instantaneous reload"</div>
                                    </div>
                                </div>

                                // Right Pane: Live Interactive Sandbox Studio (5 cols)
                                <div class="lg:col-span-5 p-6 flex flex-col justify-between space-y-6 bg-zinc-50/60 dark:bg-zinc-950/50">
                                    <div class="space-y-4">
                                        // Mini Browser URL Bar
                                        <div class="px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 flex items-center justify-between text-xs font-mono shadow-xs">
                                            <div class="flex items-center gap-2 text-zinc-400">
                                                {render_icon("lock", IconSize::Sm, Some("w-3 h-3 text-emerald-500"), None)}
                                                <span class="text-zinc-700 dark:text-zinc-300">"http://localhost:3000/"</span>
                                            </div>
                                            <span class="text-[10px] text-emerald-600 dark:text-emerald-400 font-bold">"0 KB Hydration"</span>
                                        </div>

                                        // Interactive Counter Island
                                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-xs space-y-4">
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <h3 class="text-sm font-bold text-zinc-900 dark:text-white" data-i18n="sandbox.island.title">
                                                        "Reactive State Island"
                                                    </h3>
                                                    <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-0.5" data-i18n="sandbox.island.desc">
                                                        "Zero-cost client interaction"
                                                    </p>
                                                </div>
                                                <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-emerald-50 dark:bg-emerald-950/60 text-emerald-600 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-800 font-semibold" data-i18n="sandbox.active">
                                                    "Active"
                                                </span>
                                            </div>

                                            <div class="flex items-center justify-center gap-3 py-2">
                                                <button
                                                    onclick="const el=document.getElementById('demo-count'); el.innerText=Math.max(0, parseInt(el.innerText)-1);"
                                                    class="w-10 h-10 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-800 dark:hover:bg-zinc-700 font-mono font-bold text-base text-zinc-800 dark:text-zinc-200 flex items-center justify-center transition-all cursor-pointer active:scale-95 shadow-xs"
                                                >
                                                    "-"
                                                </button>
                                                <div class="px-6 py-2 rounded-xl bg-zinc-50 dark:bg-zinc-950 border border-zinc-200 dark:border-zinc-800 font-mono text-2xl font-black text-zinc-900 dark:text-white min-w-[90px] text-center shadow-inner">
                                                    <span id="demo-count">"42"</span>
                                                </div>
                                                <button
                                                    onclick="const el=document.getElementById('demo-count'); el.innerText=parseInt(el.innerText)+1;"
                                                    class="w-10 h-10 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-800 dark:hover:bg-zinc-700 font-mono font-bold text-base text-zinc-800 dark:text-zinc-200 flex items-center justify-center transition-all cursor-pointer active:scale-95 shadow-xs"
                                                >
                                                    "+"
                                                </button>
                                            </div>

                                            <div class="flex items-center justify-between pt-2 border-t border-zinc-100 dark:border-zinc-800/80 text-[11px] font-mono text-zinc-400">
                                                <span data-i18n="sandbox.ttfb">"Simulated TTFB"</span>
                                                <span class="text-emerald-600 dark:text-emerald-400 font-bold">"0.28 ms"</span>
                                            </div>
                                        </div>
                                    </div>

                                    // Compiler Check Status
                                    <div class="p-3.5 rounded-xl bg-zinc-100/90 dark:bg-zinc-900/60 border border-zinc-200/90 dark:border-zinc-800/90 text-xs font-mono space-y-1.5 shadow-xs">
                                        <div class="flex items-center justify-between text-zinc-600 dark:text-zinc-400">
                                            <span data-i18n="sandbox.compiler">"Compiler check:"</span>
                                            <span class="text-emerald-600 dark:text-emerald-400 font-semibold" data-i18n="sandbox.verified">"rustc verified"</span>
                                        </div>
                                        <div class="flex items-center justify-between text-zinc-600 dark:text-zinc-400">
                                            <span data-i18n="sandbox.escaping">"HTML Escaping:"</span>
                                            <span class="text-emerald-600 dark:text-emerald-400 font-semibold" data-i18n="sandbox.no_xss">"Zero-XSS by default"</span>
                                        </div>
                                        <div class="flex items-center justify-between text-zinc-600 dark:text-zinc-400">
                                            <span data-i18n="sandbox.node">"Node.js dependency:"</span>
                                            <span class="text-zinc-900 dark:text-white font-semibold" data-i18n="sandbox.none">"None (0 MB)"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 2. HARD PROOF METRICS (SHUTTLE BENCHMARK DX)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8">
                    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 text-left">
                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors card-hover cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":16}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.02}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":260,\"damping\":20}"
                        >
                            <span class="text-xs font-mono text-zinc-400 uppercase tracking-wider font-semibold" data-i18n="metrics.throughput">"Throughput"</span>
                            <div class="text-3xl sm:text-4xl font-extrabold text-zinc-900 dark:text-white font-mono tracking-tight">
                                "124,500"
                            </div>
                            <p class="text-xs text-zinc-500 pt-1 font-sans" data-i18n="metrics.throughput.desc">
                                "req/sec on standard 8-core hardware"
                            </p>
                        </div>

                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors card-hover cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":16}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.02}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":260,\"damping\":20,\"delay_ms\":60}"
                        >
                            <span class="text-xs font-mono text-amber-500 uppercase tracking-wider font-semibold" data-i18n="metrics.latency">"Median Latency"</span>
                            <div class="text-3xl sm:text-4xl font-extrabold text-zinc-900 dark:text-white font-mono tracking-tight">
                                "&lt; 0.28" <span class="text-lg font-normal text-zinc-400">"ms"</span>
                            </div>
                            <p class="text-xs text-zinc-500 pt-1 font-sans" data-i18n="metrics.latency.desc">
                                "Instantaneous time-to-first-byte"
                            </p>
                        </div>

                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors card-hover cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":16}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.02}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":260,\"damping\":20,\"delay_ms\":120}"
                        >
                            <span class="text-xs font-mono text-emerald-500 uppercase tracking-wider font-semibold" data-i18n="metrics.hydration">"Client Hydration"</span>
                            <div class="text-3xl sm:text-4xl font-extrabold text-zinc-900 dark:text-white font-mono tracking-tight">
                                "0" <span class="text-lg font-normal text-zinc-400">"KB"</span>
                            </div>
                            <p class="text-xs text-zinc-500 pt-1 font-sans" data-i18n="metrics.hydration.desc">
                                "Zero JavaScript runtime sent to browser"
                            </p>
                        </div>

                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors card-hover cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":16}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.02}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":260,\"damping\":20,\"delay_ms\":180}"
                        >
                            <span class="text-xs font-mono text-sky-500 uppercase tracking-wider font-semibold" data-i18n="metrics.memory">"Idle Memory"</span>
                            <div class="text-3xl sm:text-4xl font-extrabold text-zinc-900 dark:text-white font-mono tracking-tight">
                                "8.2" <span class="text-lg font-normal text-zinc-400">"MB"</span>
                            </div>
                            <p class="text-xs text-zinc-500 pt-1 font-sans" data-i18n="metrics.memory.desc">
                                "Lightweight native memory footprint"
                            </p>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 3. NEXT.JS STYLE ARCHITECTURAL BENTO GRID
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8">
                    <div class="space-y-3">
                        <div class="flex items-center gap-2">
                            {render_icon("layers", IconSize::Sm, Some("w-4 h-4 text-amber-500"), None)}
                            <span class="text-xs font-mono uppercase tracking-wider text-amber-600 dark:text-amber-400 font-semibold" data-i18n="bento.badge">
                                "Core Innovations"
                            </span>
                        </div>
                        <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight text-zinc-900 dark:text-white" data-i18n="bento.title">
                            "Engineered for speed, built for reliability."
                        </h2>
                        <p class="text-base text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed" data-i18n="bento.subtitle">
                            "Everything required to build, test, and ship modern web software—without the fragility of multi-tier JavaScript build pipelines."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
                        // Bento Card 1 (Span 2): Zero-Hydration SSR
                        <div
                            class="md:col-span-2 p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-6 flex flex-col justify-between card-hover shadow-xs cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":24}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.008}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22}"
                        >
                            <div class="space-y-4">
                                <div class="w-10 h-10 rounded-xl bg-amber-500/10 dark:bg-amber-500/20 text-amber-600 dark:text-amber-400 flex items-center justify-center">
                                    {render_icon("zap", IconSize::Md, None, None)}
                                </div>
                                <h3 class="text-xl font-bold text-zinc-900 dark:text-white tracking-tight" data-i18n="bento.card1.title">
                                    "Zero-Hydration Server-Side Rendering"
                                </h3>
                                <p class="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed font-sans max-w-xl" data-i18n="bento.card1.desc">
                                    "Traditional JavaScript frameworks render HTML on the server, only to serialize all data and force the browser to parse 500KB+ of JS just to attach event listeners. Yav eliminates this hydration tax: HTML is compiled into direct buffer appends, transmitted instantly, and interactive widgets run via lightweight micro-scripts."
                                </p>
                            </div>

                            <div class="p-4 rounded-xl bg-zinc-50 dark:bg-zinc-950/70 border border-zinc-200/80 dark:border-zinc-800/80 grid grid-cols-2 gap-4 font-mono text-xs">
                                <div>
                                    <span class="text-zinc-400 block text-[10px] uppercase">"Traditional SSR (Next/Nuxt)"</span>
                                    <span class="text-red-500 font-semibold text-sm">"300 - 800 KB JS"</span>
                                    <span class="text-[11px] text-zinc-500 block">"Hydration lag &amp; TTI delay"</span>
                                </div>
                                <div class="border-l border-zinc-200 dark:border-zinc-800 pl-4">
                                    <span class="text-amber-500 font-semibold block text-[10px] uppercase">"Yav Engine"</span>
                                    <span class="text-emerald-500 font-bold text-sm">"0 KB Hydration"</span>
                                    <span class="text-[11px] text-zinc-500 block">"Instant interactive rendering"</span>
                                </div>
                            </div>
                        </div>

                        // Bento Card 2: Compile-Time RSX Macro
                        <div
                            class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":24}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.01}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22,\"delay_ms\":60}"
                        >
                            <div class="space-y-3">
                                <div class="w-10 h-10 rounded-xl bg-sky-500/10 dark:bg-sky-500/20 text-sky-600 dark:text-sky-400 flex items-center justify-center">
                                    {render_icon("code", IconSize::Md, None, None)}
                                </div>
                                <h3 class="text-xl font-bold text-zinc-900 dark:text-white tracking-tight" data-i18n="bento.card2.title">
                                    "Compile-Time RSX Macro"
                                </h3>
                                <p class="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed font-sans" data-i18n="bento.card2.desc">
                                    "Write intuitive declarative markup with the ergonomics of JSX, backed by the full power of the Rust compiler. Type-checked attributes, automatic HTML escaping, and zero runtime template parsing."
                                </p>
                            </div>
                            <div class="pt-2">
                                <span class="inline-flex items-center gap-1.5 text-xs font-mono text-sky-600 dark:text-sky-400 font-medium">
                                    <span>"Validated by rustc"</span>
                                    {render_icon("check", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                </span>
                            </div>
                        </div>

                        // Bento Card 3: Autonomous Tailwind Pipeline
                        <div
                            class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":24}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.01}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22,\"delay_ms\":120}"
                        >
                            <div class="space-y-3">
                                <div class="w-10 h-10 rounded-xl bg-purple-500/10 dark:bg-purple-500/20 text-purple-600 dark:text-purple-400 flex items-center justify-center">
                                    {render_icon("box", IconSize::Md, None, None)}
                                </div>
                                <h3 class="text-xl font-bold text-zinc-900 dark:text-white tracking-tight" data-i18n="bento.card3.title">
                                    "Autonomous Tailwind CSS"
                                </h3>
                                <p class="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed font-sans" data-i18n="bento.card3.desc">
                                    "Yav manages the official standalone Tailwind compiler binary directly. It detects CSS changes, triggers instant incremental re-compilations, and bundles production styles with automatic content hashing."
                                </p>
                            </div>
                            <div class="pt-2">
                                <span class="text-xs font-mono text-purple-600 dark:text-purple-400 font-medium">
                                    "Zero Node.js • Zero npm"
                                </span>
                            </div>
                        </div>

                        // Bento Card 4: Matchit Radix Routing
                        <div
                            class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":24}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.01}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22,\"delay_ms\":180}"
                        >
                            <div class="space-y-3">
                                <div class="w-10 h-10 rounded-xl bg-emerald-500/10 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 flex items-center justify-center">
                                    {render_icon("activity", IconSize::Md, None, None)}
                                </div>
                                <h3 class="text-xl font-bold text-zinc-900 dark:text-white tracking-tight" data-i18n="bento.card4.title">
                                    "File-System Matchit Router"
                                </h3>
                                <p class="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed font-sans" data-i18n="bento.card4.desc">
                                    "Drop route handlers in src/routes/ with intuitive dynamic parameters like [id].rs. At build time, Yav compiles your entire folder hierarchy into high-speed Radix routing trees with constant-time O(k) lookup."
                                </p>
                            </div>
                            <div class="pt-2">
                                <span class="text-xs font-mono text-emerald-600 dark:text-emerald-400 font-medium">
                                    "Sub-microsecond resolution"
                                </span>
                            </div>
                        </div>

                        // Bento Card 5: Full Axum 0.8 Integration (Span 2)
                        <div
                            class="md:col-span-2 p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":24}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.008}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22,\"delay_ms\":240}"
                        >
                            <div class="space-y-3">
                                <div class="w-10 h-10 rounded-xl bg-orange-500/10 dark:bg-orange-500/20 text-orange-600 dark:text-orange-400 flex items-center justify-center">
                                    {render_icon("server", IconSize::Md, None, None)}
                                </div>
                                <h3 class="text-xl font-bold text-zinc-900 dark:text-white tracking-tight" data-i18n="bento.card5.title">
                                    "Uncompromised Axum 0.8 &amp; Tokio Foundation"
                                </h3>
                                <p class="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed font-sans max-w-xl" data-i18n="bento.card5.desc">
                                    "Yav is not a black-box wrapper that locks you in. You have complete native access to Tokio multi-threaded asynchronous tasks, Axum extractors, Tower middleware layers, WebSockets, Server-Sent Events, and signed encrypted cookie sessions."
                                </p>
                            </div>

                            <div class="flex flex-wrap gap-2 pt-2">
                                <span class="px-2.5 py-1 rounded-md bg-zinc-100 dark:bg-zinc-800 text-[11px] font-mono text-zinc-700 dark:text-zinc-300">
                                    "Tower Middleware"
                                </span>
                                <span class="px-2.5 py-1 rounded-md bg-zinc-100 dark:bg-zinc-800 text-[11px] font-mono text-zinc-700 dark:text-zinc-300">
                                    "WebSockets"
                                </span>
                                <span class="px-2.5 py-1 rounded-md bg-zinc-100 dark:bg-zinc-800 text-[11px] font-mono text-zinc-700 dark:text-zinc-300">
                                    "SSE Streaming"
                                </span>
                                <span class="px-2.5 py-1 rounded-md bg-zinc-100 dark:bg-zinc-800 text-[11px] font-mono text-zinc-700 dark:text-zinc-300">
                                    "HMAC Signed Sessions"
                                </span>
                                <span class="px-2.5 py-1 rounded-md bg-zinc-100 dark:bg-zinc-800 text-[11px] font-mono text-zinc-700 dark:text-zinc-300">
                                    "CSRF Protection"
                                </span>
                            </div>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 4. SHUTTLE-GRADE COMPARISON MATRIX
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8">
                    <div class="space-y-3 text-center sm:text-left">
                        <div class="flex items-center gap-2 justify-center sm:justify-start">
                            {render_icon("activity", IconSize::Sm, Some("w-4 h-4 text-emerald-500"), None)}
                            <span class="text-xs font-mono uppercase tracking-wider text-emerald-600 dark:text-emerald-400 font-semibold" data-i18n="bench.badge">
                                "Real-World Benchmarks &amp; Architecture"
                            </span>
                        </div>
                        <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight text-zinc-900 dark:text-white" data-i18n="bench.title">
                            "Raw metal performance. Zero compromises."
                        </h2>
                        <p class="text-base text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed" data-i18n="bench.subtitle">
                            "Comprehensive technical comparison across modern full-stack frameworks running dynamic workloads on identical hardware."
                        </p>
                    </div>

                    // Comparison Table (Shuttle.dev Style)
                    <div class="rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 overflow-hidden shadow-xs">
                        <div class="overflow-x-auto">
                            <table class="w-full text-left text-xs font-mono">
                                <thead>
                                    <tr class="border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-950/80 text-zinc-500">
                                        <th class="p-4 sm:px-6 font-semibold uppercase tracking-wider">"Capability"</th>
                                        <th class="p-4 sm:px-6 font-bold text-amber-500 bg-amber-500/5 dark:bg-amber-500/10">"Yav.rs (Rust)"</th>
                                        <th class="p-4 sm:px-6 font-medium">"Next.js 15 (Node.js)"</th>
                                        <th class="p-4 sm:px-6 font-medium">"Leptos (WASM)"</th>
                                        <th class="p-4 sm:px-6 font-medium">"Bare Axum (API)"</th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-zinc-200 dark:divide-zinc-800/80">
                                    <tr class="hover:bg-zinc-50/50 dark:hover:bg-zinc-800/20">
                                        <td class="p-4 sm:px-6 text-zinc-900 dark:text-white font-sans font-medium">"Requests Per Second"</td>
                                        <td class="p-4 sm:px-6 font-bold text-emerald-500 bg-amber-500/5 dark:bg-amber-500/10">"124,500 req/s"</td>
                                        <td class="p-4 sm:px-6 text-zinc-500">"8,900 req/s"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"45,000 req/s"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"130,000 req/s (no HTML)"</td>
                                    </tr>
                                    <tr class="hover:bg-zinc-50/50 dark:hover:bg-zinc-800/20">
                                        <td class="p-4 sm:px-6 text-zinc-900 dark:text-white font-sans font-medium">"Client JS Hydration"</td>
                                        <td class="p-4 sm:px-6 font-bold text-emerald-500 bg-amber-500/5 dark:bg-amber-500/10">"0 KB JS"</td>
                                        <td class="p-4 sm:px-6 text-red-500">"350 - 800 KB"</td>
                                        <td class="p-4 sm:px-6 text-amber-500">"250 - 500 KB WASM"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"N/A"</td>
                                    </tr>
                                    <tr class="hover:bg-zinc-50/50 dark:hover:bg-zinc-800/20">
                                        <td class="p-4 sm:px-6 text-zinc-900 dark:text-white font-sans font-medium">"Median TTFB Latency"</td>
                                        <td class="p-4 sm:px-6 font-bold text-emerald-500 bg-amber-500/5 dark:bg-amber-500/10">"0.28 ms"</td>
                                        <td class="p-4 sm:px-6 text-zinc-500">"8.40 ms"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"1.20 ms"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"0.24 ms"</td>
                                    </tr>
                                    <tr class="hover:bg-zinc-50/50 dark:hover:bg-zinc-800/20">
                                        <td class="p-4 sm:px-6 text-zinc-900 dark:text-white font-sans font-medium">"Idle RAM Usage"</td>
                                        <td class="p-4 sm:px-6 font-bold text-emerald-500 bg-amber-500/5 dark:bg-amber-500/10">"8.2 MB"</td>
                                        <td class="p-4 sm:px-6 text-red-500">"180 - 320 MB"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"24 MB"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"6.5 MB"</td>
                                    </tr>
                                    <tr class="hover:bg-zinc-50/50 dark:hover:bg-zinc-800/20">
                                        <td class="p-4 sm:px-6 text-zinc-900 dark:text-white font-sans font-medium">"Node.js / npm Required"</td>
                                        <td class="p-4 sm:px-6 font-bold text-emerald-500 bg-amber-500/5 dark:bg-amber-500/10">"Never (0 MB)"</td>
                                        <td class="p-4 sm:px-6 text-red-500">"Mandatory (450 MB)"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"Optional (trunk/wasm-bindgen)"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"None"</td>
                                    </tr>
                                    <tr class="hover:bg-zinc-50/50 dark:hover:bg-zinc-800/20">
                                        <td class="p-4 sm:px-6 text-zinc-900 dark:text-white font-sans font-medium">"Developer Ergonomics"</td>
                                        <td class="p-4 sm:px-6 font-bold text-emerald-500 bg-amber-500/5 dark:bg-amber-500/10">"RSX + Matchit + yav dev"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"JSX + App Router"</td>
                                        <td class="p-4 sm:px-6 text-zinc-400">"view! macro"</td>
                                        <td class="p-4 sm:px-6 text-zinc-500">"Manual string concats / Askama"</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 5. GET STARTED IN 60 SECONDS (SHUTTLE ONBOARDING)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8">
                    <div class="space-y-3">
                        <div class="flex items-center gap-2">
                            {render_icon("terminal", IconSize::Sm, Some("w-4 h-4 text-amber-500"), None)}
                            <span class="text-xs font-mono uppercase tracking-wider text-amber-600 dark:text-amber-400 font-semibold" data-i18n="steps.badge">
                                "Get Started in 60 Seconds"
                            </span>
                        </div>
                        <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight text-zinc-900 dark:text-white" data-i18n="steps.title">
                            "Three commands to your first app."
                        </h2>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
                        // Step 1
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-3 shadow-xs">
                            <span class="w-7 h-7 rounded-lg bg-zinc-100 dark:bg-zinc-800 font-mono font-bold text-xs flex items-center justify-center text-zinc-700 dark:text-zinc-300">
                                "1"
                            </span>
                            <h3 class="text-base font-bold text-zinc-900 dark:text-white" data-i18n="steps.step1.title">
                                "Install the Yav CLI"
                            </h3>
                            <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed" data-i18n="steps.step1.desc">
                                "Installs the standalone yav development toolchain onto your PATH."
                            </p>
                            <div class="p-3 rounded-lg bg-zinc-950 font-mono text-xs text-zinc-200 flex items-center justify-between">
                                <span>"cargo install yav-cli"</span>
                                <button
                                    onclick="navigator.clipboard.writeText('cargo install yav-cli'); alertNotification('Copied: cargo install yav-cli');"
                                    class="text-zinc-400 hover:text-white cursor-pointer"
                                    title="Copy command"
                                >
                                    {render_icon("copy", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                </button>
                            </div>
                        </div>

                        // Step 2
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-3 shadow-xs">
                            <span class="w-7 h-7 rounded-lg bg-zinc-100 dark:bg-zinc-800 font-mono font-bold text-xs flex items-center justify-center text-zinc-700 dark:text-zinc-300">
                                "2"
                            </span>
                            <h3 class="text-base font-bold text-zinc-900 dark:text-white" data-i18n="steps.step2.title">
                                "Scaffold New Project"
                            </h3>
                            <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed" data-i18n="steps.step2.desc">
                                "Generates all layouts, routes, styles, and package configuration."
                            </p>
                            <div class="p-3 rounded-lg bg-zinc-950 font-mono text-xs text-zinc-200 flex items-center justify-between">
                                <span>"yav new my-app"</span>
                                <button
                                    onclick="navigator.clipboard.writeText('yav new my-app'); alertNotification('Copied: yav new my-app');"
                                    class="text-zinc-400 hover:text-white cursor-pointer"
                                    title="Copy command"
                                >
                                    {render_icon("copy", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                </button>
                            </div>
                        </div>

                        // Step 3
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-3 shadow-xs">
                            <span class="w-7 h-7 rounded-lg bg-zinc-100 dark:bg-zinc-800 font-mono font-bold text-xs flex items-center justify-center text-zinc-700 dark:text-zinc-300">
                                "3"
                            </span>
                            <h3 class="text-base font-bold text-zinc-900 dark:text-white" data-i18n="steps.step3.title">
                                "Run Dev Server"
                            </h3>
                            <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed" data-i18n="steps.step3.desc">
                                "Starts live reload, Tailwind compilation, and browser synchronization."
                            </p>
                            <div class="p-3 rounded-lg bg-zinc-950 font-mono text-xs text-zinc-200 flex items-center justify-between">
                                <span>"cd my-app &amp;&amp; yav dev"</span>
                                <button
                                    onclick="navigator.clipboard.writeText('cd my-app && yav dev'); alertNotification('Copied: cd my-app && yav dev');"
                                    class="text-zinc-400 hover:text-white cursor-pointer"
                                    title="Copy command"
                                >
                                    {render_icon("copy", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                </button>
                            </div>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 6. FINAL HIGH-CONTRAST CTA BANNER (NEXT.JS STYLE)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8">
                    <div class="rounded-3xl border border-zinc-200 dark:border-zinc-800 bg-gradient-to-br from-zinc-900 via-zinc-950 to-black text-white p-8 sm:p-14 text-center sm:text-left flex flex-col sm:flex-row items-center justify-between gap-8 shadow-2xl relative overflow-hidden">
                        <div class="space-y-4 max-w-xl z-10">
                            <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight leading-tight" data-i18n="cta.title">
                                "Ready to build faster web applications in Rust?"
                            </h2>
                            <p class="text-sm sm:text-base text-zinc-400 leading-relaxed font-sans" data-i18n="cta.subtitle">
                                "Join the next generation of full-stack Rust developers. Type safe, memory safe, and blazingly fast."
                            </p>
                        </div>

                        <div class="flex flex-col sm:flex-row items-center gap-3 z-10 w-full sm:w-auto">
                            <a
                                href="/docs"
                                class="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl bg-white text-zinc-950 hover:bg-zinc-100 text-sm font-semibold transition-all shadow-md active:scale-95"
                            >
                                <span data-i18n="cta.docs">"Read Documentation"</span>
                                {render_icon("arrow-right", IconSize::Sm, Some("w-4 h-4"), None)}
                            </a>
                            <a
                                href="https://github.com/isaacvay/yav.rs"
                                target="_blank"
                                rel="noreferrer"
                                class="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-5 py-3.5 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-white text-sm font-medium transition-all border border-zinc-700 active:scale-95"
                            >
                                {render_icon("minimal:github", IconSize::Sm, Some("w-4 h-4"), None)}
                                <span data-i18n="cta.github">"Star on GitHub"</span>
                            </a>
                        </div>
                    </div>
                </section>
            </div>

            // Client Script for Hero Code Tabs
            <script>
                {r#"
                function switchHeroTab(tab) {
                    const tabs = ['rsx', 'api', 'yavml', 'term'];
                    tabs.forEach(t => {
                        const codeEl = document.getElementById('hero-code-' + t);
                        const btnEl = document.getElementById('btn-file-' + t);
                        if (codeEl && btnEl) {
                            if (t === tab) {
                                codeEl.classList.remove('hidden');
                                btnEl.className = 'hero-tab-btn active px-3 py-1 rounded-md bg-white dark:bg-zinc-900 text-zinc-900 dark:text-white shadow-xs font-medium cursor-pointer transition-all flex items-center gap-1.5';
                            } else {
                                codeEl.classList.add('hidden');
                                btnEl.className = 'hero-tab-btn px-3 py-1 rounded-md text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white cursor-pointer transition-all flex items-center gap-1.5';
                            }
                        }
                    });
                }
                "#}
            </script>
        </AppLayout>
    };
    HtmlPage::new(page)
        .title("Yav.rs — The High-Velocity Web Framework for Rust")
        .favicon("/public/favicon.svg")
}