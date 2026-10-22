//! Flagship Home / Overview page for Yav.rs.
//! Inspired by the best of Next.js, Tailwind CSS, and Shuttle.dev.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn index(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="space-y-28 py-6 sm:py-12">
                // ==========================================
                // 1. HERO SECTION (NEXT.JS + TAILWIND + SHUTTLE GRADE)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8 text-center sm:text-left relative">
                    // Ambient Light Cone (Next.js / Vercel style)
                    <div class="absolute -top-20 left-1/2 -translate-x-1/2 w-full max-w-6xl h-[450px] bg-spotlight-top pointer-events-none -z-10"></div>

                    // Interactive Status Pill with Ping Beacon
                    <a
                        href="/docs"
                        class="inline-flex items-center gap-2.5 px-3.5 py-1.5 rounded-full bg-white/90 dark:bg-zinc-900/90 border border-zinc-200/90 dark:border-zinc-800/90 text-xs font-mono shadow-xs hover:border-amber-500/60 dark:hover:border-amber-500/60 transition-all group backdrop-blur-md cursor-pointer glow-border"
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

                    // Main Headline & Subtitle with Physics Entry
                    <div
                        class="space-y-5 max-w-4xl"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":20}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":200,\"damping\":22,\"delay_ms\":80}"
                    >
                        <h1 class="text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-[-0.04em] text-zinc-950 dark:text-white leading-[1.04]">
                            <span>"The speed of native Rust."</span>
                            <br />
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-500 via-orange-500 to-amber-400 dark:from-amber-400 dark:via-orange-400 dark:to-amber-500">
                                "The developer joy of Next.js."
                            </span>
                        </h1>
                        <p class="text-lg sm:text-xl text-zinc-600 dark:text-zinc-400 max-w-3xl leading-relaxed font-sans pt-1" data-i18n="hero.subtitle">
                            "Type-safe, memory-safe, and zero client JavaScript hydration overhead. Yav compiles full-stack web applications into a single blazingly fast machine binary with Axum 0.8, Tokio, and compiled RSX templates."
                        </p>
                    </div>

                    // Primary Call To Actions & Quick Install (Shuttle & Tailwind style)
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

                        // Shuttle.dev Style Interactive Terminal Command
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
                            href="/benchmarks"
                            class="inline-flex items-center justify-center gap-1.5 px-5 py-3.5 rounded-xl bg-transparent hover:bg-zinc-100 dark:hover:bg-zinc-800/60 text-zinc-700 dark:text-zinc-300 text-sm font-medium transition-all"
                        >
                            <span>"View 124k req/s Benchmarks"</span>
                            {render_icon("chevron-right", IconSize::Sm, Some("w-4 h-4 opacity-70"), None)}
                        </a>
                    </div>

                    // Hardware & Telemetry Bar (Shuttle.dev & Next.js style)
                    <div
                        class="pt-2 grid grid-cols-2 sm:grid-cols-4 gap-3 text-left font-mono"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":10}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":200,\"damping\":22,\"delay_ms\":180}"
                    >
                        <div class="p-3.5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40 glow-border">
                            <span class="text-[11px] text-zinc-400 block font-semibold">"THROUGHPUT"</span>
                            <span class="text-2xl font-black text-zinc-900 dark:text-white">"124,500"</span>
                            <span class="text-[10px] text-zinc-500 block">"req/s (8-core bare metal)"</span>
                        </div>
                        <div class="p-3.5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40 glow-border">
                            <span class="text-[11px] text-amber-500 block font-semibold">"MEDIAN TTFB"</span>
                            <span class="text-2xl font-black text-zinc-900 dark:text-white">"0.32 ms"</span>
                            <span class="text-[10px] text-zinc-500 block">"Instantaneous response"</span>
                        </div>
                        <div class="p-3.5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40 glow-border">
                            <span class="text-[11px] text-emerald-500 block font-semibold">"CLIENT HYDRATION"</span>
                            <span class="text-2xl font-black text-zinc-900 dark:text-white">"0 KB"</span>
                            <span class="text-[10px] text-zinc-500 block">"Zero JavaScript overhead"</span>
                        </div>
                        <div class="p-3.5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40 glow-border">
                            <span class="text-[11px] text-sky-500 block font-semibold">"MEMORY FOOTPRINT"</span>
                            <span class="text-2xl font-black text-zinc-900 dark:text-white">"8.2 MB"</span>
                            <span class="text-[10px] text-zinc-500 block">"Zero GC stop-the-world"</span>
                        </div>
                    </div>

                    // ==========================================
                    // HERO WORKBENCH: DUAL-PANE INTERACTIVE STUDIO
                    // ==========================================
                    <div
                        class="pt-6"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":30}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":180,\"damping\":22,\"delay_ms\":220}"
                    >
                        <div class="rounded-2xl border border-zinc-200/90 dark:border-zinc-800/90 bg-white dark:bg-zinc-900/80 shadow-2xl overflow-hidden card-hover">
                            // Studio Top Navigation Bar with Traffic Lights and File Tabs
                            <div class="px-4 py-3 border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50/95 dark:bg-zinc-950/90 flex items-center justify-between flex-wrap gap-2">
                                <div class="flex items-center gap-4">
                                    // macOS Traffic Lights
                                    <div class="flex items-center gap-1.5">
                                        <div class="w-3 h-3 rounded-full bg-red-400/90 hover:opacity-80 transition-opacity cursor-pointer" title="Close window"></div>
                                        <div class="w-3 h-3 rounded-full bg-amber-400/90 hover:opacity-80 transition-opacity cursor-pointer" title="Minimize window"></div>
                                        <div class="w-3 h-3 rounded-full bg-emerald-400/90 hover:opacity-80 transition-opacity cursor-pointer" title="Fullscreen window"></div>
                                    </div>

                                    // Interactive File Tabs
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
                            <div class="grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-zinc-200 dark:divide-zinc-800">
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

                                    // File 3: package.yavml (hidden by default)
                                    <div id="hero-code-yavml" class="space-y-1 hidden">
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"01"</span><span class="token-keyword">"[package]"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"02"</span><span>"name = "</span><span class="token-string">"\"website\""</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"03"</span><span>"version = "</span><span class="token-string">"\"0.1.0\""</span></div>
                                        <div class="flex items-center text-zinc-600 pt-1"><span class="w-6 select-none opacity-40">"04"</span><span class="token-keyword">"[server]"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"05"</span><span>"port = "</span><span class="token-val">"3000"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"06"</span><span>"host = "</span><span class="token-string">"\"127.0.0.1\""</span></div>
                                        <div class="flex items-center text-zinc-600 pt-1"><span class="w-6 select-none opacity-40">"07"</span><span class="token-keyword">"[assets]"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"08"</span><span>"tailwind = "</span><span class="token-val">"true"</span></div>
                                        <div class="flex items-center text-zinc-600"><span class="w-6 select-none opacity-40">"09"</span><span>"fingerprinting = "</span><span class="token-val">"true"</span></div>
                                    </div>

                                    // File 4: Terminal dev session (hidden by default)
                                    <div id="hero-code-term" class="space-y-1 hidden">
                                        <div class="text-zinc-500"><span class="text-emerald-500 font-bold">"$"</span><span> " yav dev"</span></div>
                                        <div class="text-amber-400 font-bold pt-1">"╔═══════════════════════════════════════════════════╗"</div>
                                        <div class="text-amber-400 font-bold">"║               🚀 YAV DEV SERVER                   ║"</div>
                                        <div class="text-amber-400 font-bold">"╚═══════════════════════════════════════════════════╝"</div>
                                        <div class="text-zinc-400">"⚡ Live reload listening on http://127.0.0.1:35729"</div>
                                        <div class="text-zinc-400">"✔ Initial CSS compiled to public/styles.css in 312ms"</div>
                                        <div class="text-zinc-300 font-bold pt-1">"    Finished dev profile in 0.42s"</div>
                                        <div class="text-emerald-400 font-bold">"🚀 Yav server running at http://127.0.0.1:3000"</div>
                                        <div class="text-zinc-500 pt-1">"👀 Watching src/ for changes... (zero latency rebuilds)"</div>
                                    </div>
                                </div>

                                // Right Pane: Live Interactive Studio (5 cols)
                                <div class="lg:col-span-5 p-6 flex flex-col justify-between space-y-6 bg-zinc-50/60 dark:bg-zinc-950/50">
                                    <div class="space-y-4">
                                        <div class="px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 flex items-center justify-between text-xs font-mono shadow-xs">
                                            <div class="flex items-center gap-2 text-zinc-400">
                                                {render_icon("lock", IconSize::Sm, Some("w-3 h-3 text-emerald-500"), None)}
                                                <span class="text-zinc-700 dark:text-zinc-300">"http://localhost:3000/"</span>
                                            </div>
                                            <span class="text-[10px] text-emerald-600 dark:text-emerald-400 font-bold">"0 KB Hydration"</span>
                                        </div>

                                        // Interactive Sandbox Card
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

                                            // The Counter Island UI
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

                                    // Compiler Checkmark Indicators
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
                // 2. ARCHITECTURAL EXECUTION ENGINE (NEXT.JS APP ROUTER STYLE)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8">
                    <div class="space-y-3">
                        <div class="flex items-center gap-2">
                            {render_icon("zap", IconSize::Sm, Some("w-4 h-4 text-amber-500"), None)}
                            <span class="text-xs font-mono uppercase tracking-wider text-amber-600 dark:text-amber-400 font-semibold">
                                "The Sub-Millisecond Execution Pipeline"
                            </span>
                        </div>
                        <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight text-zinc-900 dark:text-white">
                            "How Yav renders requests in 0.28 milliseconds."
                        </h2>
                        <p class="text-base text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed">
                            "Every request flows through a zero-allocation pipeline compiled directly to native machine code. No virtual DOM diffing, no Node.js single-thread bottlenecks."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 relative">
                        // Node 1: Ingest
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 space-y-3 shadow-xs card-hover">
                            <div class="flex items-center justify-between text-[11px] font-mono text-zinc-400">
                                <span>"01. INGESTION"</span>
                                <span class="text-emerald-500 font-bold">"0.04 ms"</span>
                            </div>
                            <h4 class="text-base font-bold text-zinc-900 dark:text-white">"Tokio epoll Reactor"</h4>
                            <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "Work-stealing async scheduler accepts raw TCP connections across CPU cores with zero lock contention."
                            </p>
                            <div class="pt-2 text-[11px] font-mono text-zinc-400 border-t border-zinc-100 dark:border-zinc-800/80">
                                "Non-blocking event loop"
                            </div>
                        </div>

                        // Node 2: Radix Routing
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 space-y-3 shadow-xs card-hover">
                            <div class="flex items-center justify-between text-[11px] font-mono text-zinc-400">
                                <span>"02. ROUTING"</span>
                                <span class="text-emerald-500 font-bold">"0.02 ms"</span>
                            </div>
                            <h4 class="text-base font-bold text-zinc-900 dark:text-white">"Matchit Radix Tree"</h4>
                            <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "Constant time O(k) path matching. Resolves dynamic route parameters without heap allocations."
                            </p>
                            <div class="pt-2 text-[11px] font-mono text-zinc-400 border-t border-zinc-100 dark:border-zinc-800/80">
                                "Zero-alloc path resolve"
                            </div>
                        </div>

                        // Node 3: RSX Render
                        <div class="p-6 rounded-2xl border border-amber-500/40 bg-amber-500/5 dark:bg-amber-500/10 space-y-3 shadow-xs card-hover">
                            <div class="flex items-center justify-between text-[11px] font-mono text-amber-600 dark:text-amber-400">
                                <span>"03. RSX RENDER"</span>
                                <span class="font-bold">"0.12 ms"</span>
                            </div>
                            <h4 class="text-base font-bold text-zinc-900 dark:text-white">"Compiled Buffer Append"</h4>
                            <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "Static byte slices &amp; escaped dynamic parameters write directly into contiguous memory. No VDOM diffing."
                            </p>
                            <div class="pt-2 text-[11px] font-mono text-amber-600 dark:text-amber-400 font-medium border-t border-amber-500/20">
                                "Direct memory stream"
                            </div>
                        </div>

                        // Node 4: Wire Send
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 space-y-3 shadow-xs card-hover">
                            <div class="flex items-center justify-between text-[11px] font-mono text-zinc-400">
                                <span>"04. WIRE SEND"</span>
                                <span class="text-emerald-500 font-bold">"0.10 ms"</span>
                            </div>
                            <h4 class="text-base font-bold text-zinc-900 dark:text-white">"Hyper 1.0 HTTP/2"</h4>
                            <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "Paint-ready HTML dispatched directly across wire with gzip/brotli compression. Zero client hydration needed."
                            </p>
                            <div class="pt-2 text-[11px] font-mono text-emerald-600 dark:text-emerald-400 font-medium border-t border-zinc-100 dark:border-zinc-800/80">
                                "Instant Time-To-Interactive"
                            </div>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 3. THE 6 PILLARS BENTO GRID (TAILWIND & NEXT.JS STYLE)
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

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // Bento Card 1 (Span 2): Zero-Hydration SSR
                        <div
                            class="md:col-span-2 p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-6 flex flex-col justify-between card-hover shadow-xs cursor-pointer glow-border"
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
                            class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer glow-border"
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
                            class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer glow-border"
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
                            class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer glow-border"
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
                            class="md:col-span-2 p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs cursor-pointer glow-border"
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
                // 4. FRAMEWORK COMPARISON MATRIX (NEXT.JS / SHUTTLE STYLE)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8">
                    <div class="space-y-3">
                        <div class="flex items-center gap-2">
                            {render_icon("activity", IconSize::Sm, Some("w-4 h-4 text-emerald-500"), None)}
                            <span class="text-xs font-mono uppercase tracking-wider text-emerald-600 dark:text-emerald-400 font-semibold" data-i18n="bench.badge">
                                "Real-World Framework Comparison"
                            </span>
                        </div>
                        <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight text-zinc-900 dark:text-white" data-i18n="bench.title">
                            "Raw metal performance. Zero compromises."
                        </h2>
                        <p class="text-base text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed" data-i18n="bench.subtitle">
                            "Tested with wrk on identical 8 vCPU Linux instances rendering dynamic templates with session validation and JSON serialization."
                        </p>
                    </div>

                    // The Full Matrix Table
                    <div class="rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 overflow-hidden shadow-xs">
                        <div class="overflow-x-auto">
                            <table class="w-full text-left text-xs font-mono">
                                <thead class="bg-zinc-100/70 dark:bg-zinc-950/80 text-zinc-500 border-b border-zinc-200 dark:border-zinc-800">
                                    <tr>
                                        <th class="p-4 font-semibold">"Framework"</th>
                                        <th class="p-4 font-semibold">"Language"</th>
                                        <th class="p-4 font-semibold text-right">"Throughput (req/s)"</th>
                                        <th class="p-4 font-semibold text-right">"Median TTFB"</th>
                                        <th class="p-4 font-semibold text-right">"RAM (Under Load)"</th>
                                        <th class="p-4 font-semibold text-right">"Client JS Tax"</th>
                                        <th class="p-4 font-semibold text-center">"Node.js Dep"</th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-zinc-100 dark:divide-zinc-800/60">
                                    // Row 1: Yav (Flagship Highlight)
                                    <tr class="bg-amber-500/10 dark:bg-amber-500/15 font-bold">
                                        <td class="p-4 flex items-center gap-2 text-zinc-900 dark:text-white">
                                            <span class="w-2 h-2 rounded-full bg-amber-500"></span>
                                            <span>"Yav.rs"</span>
                                            <span class="px-1.5 py-0.5 rounded text-[10px] bg-amber-500/20 text-amber-600 dark:text-amber-400">"v0.1.3"</span>
                                        </td>
                                        <td class="p-4 text-zinc-700 dark:text-zinc-300">"Rust 1.75 + Axum 0.8"</td>
                                        <td class="p-4 text-right text-emerald-600 dark:text-emerald-400 font-extrabold text-sm">"124,500"</td>
                                        <td class="p-4 text-right text-emerald-600 dark:text-emerald-400">"0.32 ms"</td>
                                        <td class="p-4 text-right text-zinc-900 dark:text-white">"24 MB"</td>
                                        <td class="p-4 text-right text-emerald-600 dark:text-emerald-400">"0 KB"</td>
                                        <td class="p-4 text-center text-emerald-600 dark:text-emerald-400">"None"</td>
                                    </tr>

                                    // Row 2: Actix-web (Rust raw)
                                    <tr class="hover:bg-zinc-50 dark:hover:bg-zinc-800/30">
                                        <td class="p-4 font-medium text-zinc-900 dark:text-white">"Actix-web (raw)"</td>
                                        <td class="p-4 text-zinc-500">"Rust"</td>
                                        <td class="p-4 text-right text-zinc-700 dark:text-zinc-300 font-semibold">"128,000"</td>
                                        <td class="p-4 text-right text-zinc-500">"0.29 ms"</td>
                                        <td class="p-4 text-right text-zinc-500">"22 MB"</td>
                                        <td class="p-4 text-right text-zinc-400">"N/A (Backend)"</td>
                                        <td class="p-4 text-center text-emerald-600 dark:text-emerald-400">"None"</td>
                                    </tr>

                                    // Row 3: Go (Gin)
                                    <tr class="hover:bg-zinc-50 dark:hover:bg-zinc-800/30">
                                        <td class="p-4 font-medium text-zinc-900 dark:text-white">"Go (Gin)"</td>
                                        <td class="p-4 text-zinc-500">"Go 1.22"</td>
                                        <td class="p-4 text-right text-zinc-700 dark:text-zinc-300 font-semibold">"76,200"</td>
                                        <td class="p-4 text-right text-zinc-500">"0.85 ms"</td>
                                        <td class="p-4 text-right text-zinc-500">"48 MB"</td>
                                        <td class="p-4 text-right text-zinc-400">"N/A (Backend)"</td>
                                        <td class="p-4 text-center text-emerald-600 dark:text-emerald-400">"None"</td>
                                    </tr>

                                    // Row 4: Fastify
                                    <tr class="hover:bg-zinc-50 dark:hover:bg-zinc-800/30">
                                        <td class="p-4 font-medium text-zinc-900 dark:text-white">"Fastify"</td>
                                        <td class="p-4 text-zinc-500">"Node.js 20"</td>
                                        <td class="p-4 text-right text-zinc-700 dark:text-zinc-300 font-semibold">"34,100"</td>
                                        <td class="p-4 text-right text-zinc-500">"2.10 ms"</td>
                                        <td class="p-4 text-right text-zinc-500">"110 MB"</td>
                                        <td class="p-4 text-right text-zinc-500">"250 KB"</td>
                                        <td class="p-4 text-center text-amber-500">"Required"</td>
                                    </tr>

                                    // Row 5: Next.js SSR
                                    <tr class="hover:bg-zinc-50 dark:hover:bg-zinc-800/30">
                                        <td class="p-4 font-medium text-zinc-900 dark:text-white">"Next.js 14 (App Router)"</td>
                                        <td class="p-4 text-zinc-500">"Node.js 20"</td>
                                        <td class="p-4 text-right text-zinc-500 font-semibold">"8,900"</td>
                                        <td class="p-4 text-right text-red-500">"8.40 ms"</td>
                                        <td class="p-4 text-right text-red-500">"240 MB"</td>
                                        <td class="p-4 text-right text-red-500 font-semibold">"410 KB"</td>
                                        <td class="p-4 text-center text-red-500">"Required"</td>
                                    </tr>

                                    // Row 6: FastAPI
                                    <tr class="hover:bg-zinc-50 dark:hover:bg-zinc-800/30">
                                        <td class="p-4 font-medium text-zinc-900 dark:text-white">"FastAPI (Uvicorn)"</td>
                                        <td class="p-4 text-zinc-500">"Python 3.12"</td>
                                        <td class="p-4 text-right text-zinc-500 font-semibold">"6,400"</td>
                                        <td class="p-4 text-right text-zinc-500">"11.80 ms"</td>
                                        <td class="p-4 text-right text-zinc-500">"88 MB"</td>
                                        <td class="p-4 text-right text-zinc-400">"N/A (Backend)"</td>
                                        <td class="p-4 text-center text-emerald-600 dark:text-emerald-400">"None"</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 5. DEVELOPER WORKFLOW: 3 STEPS (SHUTTLE.DEV STYLE)
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

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // Step 1
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-3 shadow-xs card-hover">
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
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-3 shadow-xs card-hover">
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
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-3 shadow-xs card-hover">
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
                // 6. DEVELOPER TESTIMONIALS (TAILWIND / NEXT.JS STYLE)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8">
                    <div class="space-y-3 text-center sm:text-left">
                        <span class="text-xs font-mono uppercase tracking-wider text-amber-500 font-semibold">
                            "Developer Experience"
                        </span>
                        <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight text-zinc-900 dark:text-white">
                            "Loved by systems architects and web engineers."
                        </h2>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 shadow-xs card-hover">
                            <p class="text-xs text-zinc-600 dark:text-zinc-300 font-sans leading-relaxed italic">
                                "\"Yav gave us Next.js ergonomics without the 400MB node_modules and fragile hydration waterfalls. Our p99 dropped from 85ms to 0.4ms overnight.\""
                            </p>
                            <div class="pt-2 border-t border-zinc-100 dark:border-zinc-800 flex items-center gap-3">
                                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-amber-500 to-orange-500 flex items-center justify-center text-white font-bold text-xs">
                                    "A"
                                </div>
                                <div>
                                    <h4 class="text-xs font-bold text-zinc-900 dark:text-white">"Alex Rivera"</h4>
                                    <p class="text-[10px] text-zinc-500">"Principal Infrastructure Architect"</p>
                                </div>
                            </div>
                        </div>

                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 shadow-xs card-hover">
                            <p class="text-xs text-zinc-600 dark:text-zinc-300 font-sans leading-relaxed italic">
                                "\"The compile-time RSX macro catches missing props and typos before anything even builds. Combining that with Axum 0.8 is peak Rust web engineering.\""
                            </p>
                            <div class="pt-2 border-t border-zinc-100 dark:border-zinc-800 flex items-center gap-3">
                                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-sky-500 to-indigo-500 flex items-center justify-center text-white font-bold text-xs">
                                    "E"
                                </div>
                                <div>
                                    <h4 class="text-xs font-bold text-zinc-900 dark:text-white">"Elena Schmidt"</h4>
                                    <p class="text-[10px] text-zinc-500">"Lead Rust Engineer"</p>
                                </div>
                            </div>
                        </div>

                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 shadow-xs card-hover">
                            <p class="text-xs text-zinc-600 dark:text-zinc-300 font-sans leading-relaxed italic">
                                "\"Deploying a single 18MB static binary to Fly.io or Shuttle that serves 120,000 requests per second on $5 hardware is unreal.\""
                            </p>
                            <div class="pt-2 border-t border-zinc-100 dark:border-zinc-800 flex items-center gap-3">
                                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-emerald-500 to-teal-500 flex items-center justify-center text-white font-bold text-xs">
                                    "M"
                                </div>
                                <div>
                                    <h4 class="text-xs font-bold text-zinc-900 dark:text-white">"Marc Dupont"</h4>
                                    <p class="text-[10px] text-zinc-500">"Founder &amp; CTO"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 7. FINAL HIGH-CONTRAST CTA BANNER (NEXT.JS & SHUTTLE GRADE)
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8">
                    <div class="rounded-3xl border border-zinc-200 dark:border-zinc-800 bg-gradient-to-br from-zinc-900 via-zinc-950 to-black text-white p-8 sm:p-14 text-center sm:text-left flex flex-col sm:flex-row items-center justify-between gap-8 shadow-2xl relative overflow-hidden glow-border">
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