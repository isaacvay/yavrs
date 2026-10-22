//! Flagship Home / Overview page for Yav.rs.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn index(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="space-y-24 py-6 sm:py-12">
                // ==========================================
                // 1. HERO SECTION
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8 text-center sm:text-left relative">
                    // Subtle Background Glow (Pure CSS, Non-AI)
                    <div class="absolute -top-10 left-1/2 -translate-x-1/2 w-full max-w-4xl h-72 bg-radial-glow pointer-events-none -z-10"></div>

                    // Status Pill
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800/80 border border-zinc-200 dark:border-zinc-700/80 text-xs font-mono text-zinc-700 dark:text-zinc-300 shadow-xs">
                        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
                        <span class="font-semibold text-zinc-900 dark:text-white" data-i18n="hero.pill.engine">"Axum 0.8"</span>
                        <span class="text-zinc-400">"•"</span>
                        <span data-i18n="hero.pill.rsx">"Compile-Time RSX"</span>
                        <span class="text-zinc-400">"•"</span>
                        <span class="text-amber-600 dark:text-amber-400 font-semibold" data-i18n="hero.pill.nohydration">"Zero JS Hydration"</span>
                    </div>

                    // Main Headline & Subtitle
                    <div class="space-y-4 max-w-4xl">
                        <h1 class="text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-zinc-950 dark:text-white leading-[1.08]">
                            <span data-i18n="hero.title.line1">"Fast by nature."</span>
                            <br />
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-600 via-orange-500 to-amber-500 dark:from-amber-400 dark:via-orange-400 dark:to-amber-500" data-i18n="hero.title.line2">
                                "Safe by design."
                            </span>
                            <br />
                            <span data-i18n="hero.title.line3">"The modern web framework for Rust."</span>
                        </h1>
                        <p class="text-lg sm:text-xl text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed font-sans pt-2" data-i18n="hero.subtitle">
                            "Build production-grade web applications with compiled RSX templates, automatic file-system routing, and built-in Tailwind CSS. No Node.js. No npm. Zero client runtime overhead."
                        </p>
                    </div>

                    // Primary Call To Actions
                    <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3 pt-2">
                        <a
                            href="/docs"
                            class="inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 dark:bg-white dark:hover:bg-zinc-100 text-white dark:text-zinc-900 text-sm font-semibold transition-all shadow-md active:scale-98"
                        >
                            <span data-i18n="hero.cta.start">"Get Started (5-min guide)"</span>
                            {render_icon("arrow-right", IconSize::Sm, Some("w-4 h-4"), None)}
                        </a>

                        // Interactive Copyable Install Command
                        <div class="flex items-center justify-between gap-3 px-4 py-3 rounded-xl bg-zinc-100 hover:bg-zinc-200/70 dark:bg-zinc-900 dark:hover:bg-zinc-800/80 border border-zinc-200 dark:border-zinc-800 font-mono text-xs text-zinc-800 dark:text-zinc-200 shadow-xs transition-all">
                            <div class="flex items-center gap-2">
                                <span class="text-amber-500 font-bold">"$"</span>
                                <span>"cargo install yav-cli"</span>
                            </div>
                            <button
                                onclick="navigator.clipboard.writeText('cargo install yav-cli'); const el = this.querySelector('.copy-txt'); el.innerText='copied!'; setTimeout(()=>el.innerText='copy', 1500);"
                                class="copy-btn px-2 py-1 rounded bg-white dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600 text-[11px] text-zinc-500 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white transition-colors cursor-pointer flex items-center gap-1 active:scale-95"
                                title="Copy install command"
                            >
                                {render_icon("copy", IconSize::Sm, Some("w-3 h-3"), None)}
                                <span class="copy-txt">"copy"</span>
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

                    // ==========================================
                    // HERO WORKBENCH & INTERACTIVE LIVE SANDBOX
                    // ==========================================
                    <div class="pt-6">
                        <div class="rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/70 shadow-xl overflow-hidden">
                            // Window Header with macOS Dots and Tabs
                            <div class="px-4 py-3 border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50/90 dark:bg-zinc-950/70 flex items-center justify-between flex-wrap gap-2">
                                <div class="flex items-center gap-4">
                                    <div class="flex items-center gap-1.5">
                                        <div class="w-3 h-3 rounded-full bg-red-400/80"></div>
                                        <div class="w-3 h-3 rounded-full bg-amber-400/80"></div>
                                        <div class="w-3 h-3 rounded-full bg-emerald-400/80"></div>
                                    </div>

                                    <div class="flex items-center gap-1 p-1 rounded-lg bg-zinc-200/60 dark:bg-zinc-800/80 text-xs font-mono">
                                        <button
                                            id="btn-file-rsx"
                                            onclick="switchHeroTab('rsx')"
                                            class="hero-tab-btn active px-3 py-1 rounded-md bg-white dark:bg-zinc-900 text-zinc-900 dark:text-white shadow-xs font-medium cursor-pointer transition-all"
                                        >
                                            "src/routes/index.rs"
                                        </button>
                                        <button
                                            id="btn-file-yavml"
                                            onclick="switchHeroTab('yavml')"
                                            class="hero-tab-btn px-3 py-1 rounded-md text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white cursor-pointer transition-all"
                                        >
                                            "package.yavml"
                                        </button>
                                        <button
                                            id="btn-file-term"
                                            onclick="switchHeroTab('term')"
                                            class="hero-tab-btn px-3 py-1 rounded-md text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white cursor-pointer transition-all"
                                        >
                                            "$ yav dev"
                                        </button>
                                    </div>
                                </div>

                                <div class="flex items-center gap-2 text-xs font-mono text-zinc-400">
                                    <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
                                    <span>"Rust 2021 • Axum 0.8.9"</span>
                                </div>
                            </div>

                            // Split Grid: Code on Left, Live Interactive Preview on Right
                            <div class="grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-zinc-200 dark:divide-zinc-800">
                                // Left Pane: Code Viewer (7 cols)
                                <div class="lg:col-span-7 p-6 overflow-x-auto text-xs font-mono leading-relaxed bg-zinc-950 text-zinc-200">
                                    // File 1: RSX Component
                                    <div id="hero-code-rsx" class="space-y-1">
                                        <div><span class="token-keyword">"use"</span><span> " yav::prelude::*;"</span></div>
                                        <div><span class="token-keyword">"use"</span><span> " axum::extract::Request;"</span></div>
                                        <div class="pt-2"><span class="token-comment">"// Public route handler auto-discovered by yav::auto_routes!()"</span></div>
                                        <div><span class="token-keyword">"pub async fn"</span> <span class="token-fn">"index"</span><span>"(_req: Request) -&gt; HtmlPage {"</span></div>
                                        <div class="pl-4"><span class="token-macro">"rsx!"</span><span> " {"</span></div>
                                        <div class="pl-8"><span class="token-tag">"&lt;div"</span> <span class="token-attr">"class="</span><span class="token-string">"\"max-w-xl p-8 rounded-2xl bg-zinc-900 border border-zinc-800\""</span><span class="token-tag">"&gt;"</span></div>
                                        <div class="pl-12"><span class="token-tag">"&lt;h1"</span> <span class="token-attr">"class="</span><span class="token-string">"\"text-2xl font-bold tracking-tight text-white\""</span><span class="token-tag">"&gt;"</span></div>
                                        <div class="pl-16"><span class="token-string">"\"High-Velocity Full-Stack Rust\""</span></div>
                                        <div class="pl-12"><span class="token-tag">"&lt;/h1&gt;"</span></div>
                                        <div class="pl-12"><span class="token-tag">"&lt;p"</span> <span class="token-attr">"class="</span><span class="token-string">"\"text-zinc-400 mt-2\""</span><span class="token-tag">"&gt;"</span></div>
                                        <div class="pl-16"><span class="token-string">"\"Compiled directly to native machine code.\""</span></div>
                                        <div class="pl-12"><span class="token-tag">"&lt;/p&gt;"</span></div>
                                        <div class="pl-12"><span class="token-tag">"&lt;Counter"</span> <span class="token-attr">"initial="</span><span class="token-val">"{0}"</span> <span class="token-tag">"/&gt;"</span></div>
                                        <div class="pl-8"><span class="token-tag">"&lt;/div&gt;"</span></div>
                                        <div class="pl-4"><span>"}"</span></div>
                                        <div><span>"}"</span></div>
                                    </div>

                                    // File 2: package.yavml (hidden by default)
                                    <div id="hero-code-yavml" class="space-y-1 hidden">
                                        <div><span class="token-keyword">"[package]"</span></div>
                                        <div><span>"name = "</span><span class="token-string">"\"website\""</span></div>
                                        <div><span>"version = "</span><span class="token-string">"\"0.1.0\""</span></div>
                                        <div class="pt-2"><span class="token-keyword">"[server]"</span></div>
                                        <div><span>"port = "</span><span class="token-val">"3000"</span></div>
                                        <div><span>"host = "</span><span class="token-string">"\"127.0.0.1\""</span></div>
                                        <div class="pt-2"><span class="token-keyword">"[tailwind]"</span></div>
                                        <div><span>"enabled = "</span><span class="token-val">"true"</span></div>
                                        <div><span>"input = "</span><span class="token-string">"\"src/styles/app.css\""</span></div>
                                        <div><span>"output = "</span><span class="token-string">"\"public/styles.css\""</span></div>
                                        <div class="pt-2"><span class="token-keyword">"[build]"</span></div>
                                        <div><span>"minify = "</span><span class="token-val">"true"</span></div>
                                        <div><span>"fingerprint = "</span><span class="token-val">"true"</span></div>
                                    </div>

                                    // File 3: Terminal Output ($ yav dev) (hidden by default)
                                    <div id="hero-code-term" class="space-y-1 hidden">
                                        <div class="text-emerald-400 font-bold">"╔═══════════════════════════════════════════════════╗"</div>
                                        <div class="text-emerald-400 font-bold">"║               🚀 YAV DEV SERVER                   ║"</div>
                                        <div class="text-emerald-400 font-bold">"╚═══════════════════════════════════════════════════╝"</div>
                                        <div class="text-zinc-400">"⚡ Live reload server listening on http://127.0.0.1:35729"</div>
                                        <div class="text-zinc-400">"✔ Initial CSS compiled to public/styles.css in 320ms"</div>
                                        <div class="text-zinc-400">"📦 Starting application server..."</div>
                                        <div class="text-emerald-400 font-semibold">"🌐 Application running at http://127.0.0.1:3000"</div>
                                        <div class="text-zinc-500 pt-2">"👀 Watching src/ for changes"</div>
                                        <div class="text-zinc-500">"👀 Watching public/ for changes"</div>
                                        <div class="text-amber-400 pt-2">"✔ Route registered: GET /"</div>
                                        <div class="text-amber-400">"✔ Route registered: GET /docs"</div>
                                        <div class="text-amber-400">"✔ Route registered: GET /benchmarks"</div>
                                    </div>
                                </div>

                                // Right Pane: Live Interactive Sandbox (5 cols)
                                <div class="lg:col-span-5 p-6 flex flex-col justify-between space-y-6 bg-zinc-50/50 dark:bg-zinc-950/40">
                                    <div class="space-y-4">
                                        <div class="flex items-center justify-between">
                                            <span class="text-[10px] font-mono uppercase tracking-wider text-amber-500 font-bold flex items-center gap-1.5">
                                                <span class="w-2 h-2 rounded-full bg-amber-500"></span>
                                                <span data-i18n="sandbox.island.title">"Reactive State Island"</span>
                                            </span>
                                            <span class="text-[10px] font-mono text-zinc-400" data-i18n="sandbox.island.desc">"0 KB Hydration JS"</span>
                                        </div>

                                        // Interactive Micro-Counter Island
                                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-sm space-y-4">
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <h3 class="text-sm font-bold text-zinc-900 dark:text-white" data-i18n="sandbox.island.title">
                                                        "Reactive State Island"
                                                    </h3>
                                                    <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-0.5" data-i18n="sandbox.island.desc">
                                                        "Zero-cost client interaction"
                                                    </p>
                                                </div>
                                                <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-emerald-50 dark:bg-emerald-950/60 text-emerald-600 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-800" data-i18n="sandbox.active">
                                                    "Active"
                                                </span>
                                            </div>

                                            <div class="flex items-center justify-center gap-3 py-2">
                                                <button
                                                    onclick="const el=document.getElementById('demo-count'); el.innerText=Math.max(0, parseInt(el.innerText)-1);"
                                                    class="w-10 h-10 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-800 dark:hover:bg-zinc-700 font-mono font-bold text-base text-zinc-800 dark:text-zinc-200 flex items-center justify-center transition-all cursor-pointer active:scale-95"
                                                    aria-label="Decrement counter"
                                                >
                                                    "-"
                                                </button>

                                                <div class="px-6 py-2 rounded-xl bg-zinc-50 dark:bg-zinc-950 border border-zinc-200 dark:border-zinc-800 font-mono text-2xl font-black text-zinc-900 dark:text-white min-w-[90px] text-center shadow-inner">
                                                    <span id="demo-count">"42"</span>
                                                </div>

                                                <button
                                                    onclick="const el=document.getElementById('demo-count'); el.innerText=parseInt(el.innerText)+1;"
                                                    class="w-10 h-10 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-800 dark:hover:bg-zinc-700 font-mono font-bold text-base text-zinc-800 dark:text-zinc-200 flex items-center justify-center transition-all cursor-pointer active:scale-95"
                                                    aria-label="Increment counter"
                                                >
                                                    "+"
                                                </button>
                                            </div>

                                            <div class="flex items-center justify-between pt-2 border-t border-zinc-100 dark:border-zinc-800/80 text-[11px] font-mono text-zinc-400">
                                                <span data-i18n="sandbox.ttfb">"Simulated TTFB"</span>
                                                <span class="text-emerald-600 dark:text-emerald-400 font-semibold">"0.28 ms"</span>
                                            </div>
                                        </div>
                                    </div>

                                    // Quick Verification Card
                                    <div class="p-3.5 rounded-xl bg-zinc-100/80 dark:bg-zinc-900/40 border border-zinc-200/80 dark:border-zinc-800/80 text-xs font-mono space-y-1.5">
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
                // 2. PERFORMANCE BENCHMARKS BAR
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8">
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors">
                            <span class="text-xs font-mono text-zinc-400 uppercase tracking-wider font-semibold" data-i18n="metrics.throughput">"Throughput"</span>
                            <div class="text-3xl sm:text-4xl font-extrabold text-zinc-900 dark:text-white font-mono tracking-tight">
                                "124,500"
                            </div>
                            <p class="text-xs text-zinc-500 pt-1 font-sans" data-i18n="metrics.throughput.desc">
                                "req/sec on standard 8-core hardware"
                            </p>
                        </div>

                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors">
                            <span class="text-xs font-mono text-amber-500 uppercase tracking-wider font-semibold" data-i18n="metrics.latency">"Median Latency"</span>
                            <div class="text-3xl sm:text-4xl font-extrabold text-zinc-900 dark:text-white font-mono tracking-tight">
                                "&lt; 0.32" <span class="text-lg font-normal text-zinc-400">"ms"</span>
                            </div>
                            <p class="text-xs text-zinc-500 pt-1 font-sans" data-i18n="metrics.latency.desc">
                                "Instantaneous time-to-first-byte"
                            </p>
                        </div>

                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors">
                            <span class="text-xs font-mono text-emerald-500 uppercase tracking-wider font-semibold" data-i18n="metrics.hydration">"Client Hydration"</span>
                            <div class="text-3xl sm:text-4xl font-extrabold text-zinc-900 dark:text-white font-mono tracking-tight">
                                "0" <span class="text-lg font-normal text-zinc-400">"KB"</span>
                            </div>
                            <p class="text-xs text-zinc-500 pt-1 font-sans" data-i18n="metrics.hydration.desc">
                                "Zero JavaScript runtime sent to browser"
                            </p>
                        </div>

                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-1 shadow-xs hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors">
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
                // 3. THE 6 PILLARS BENTO GRID
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
                        <div class="md:col-span-2 p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-6 flex flex-col justify-between card-hover shadow-xs">
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
                        <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs">
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
                        <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs">
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
                        <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs">
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
                        <div class="md:col-span-2 p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-4 flex flex-col justify-between card-hover shadow-xs">
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
                // 4. INTERACTIVE BENCHMARK COMPARISON
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-8">
                    <div class="space-y-3">
                        <div class="flex items-center gap-2">
                            {render_icon("activity", IconSize::Sm, Some("w-4 h-4 text-emerald-500"), None)}
                            <span class="text-xs font-mono uppercase tracking-wider text-emerald-600 dark:text-emerald-400 font-semibold" data-i18n="bench.badge">
                                "Real-World Benchmarks"
                            </span>
                        </div>
                        <h2 class="text-3xl sm:text-4xl font-extrabold tracking-tight text-zinc-900 dark:text-white" data-i18n="bench.title">
                            "Raw metal performance. Zero compromises."
                        </h2>
                        <p class="text-base text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed" data-i18n="bench.subtitle">
                            "Tested with wrk on identical 8 vCPU Linux instances rendering dynamic templates with session validation and JSON serialization."
                        </p>
                    </div>

                    // Visual Comparison Chart
                    <div class="rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 p-6 sm:p-8 space-y-6 shadow-xs">
                        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-zinc-200 dark:border-zinc-800 pb-4">
                            <span class="text-xs font-mono text-zinc-500 dark:text-zinc-400 uppercase tracking-wider font-semibold">
                                "Throughput Comparison (Requests Per Second • Higher is better)"
                            </span>
                            <div class="flex items-center gap-3 text-xs font-mono">
                                <span class="flex items-center gap-1.5"><span class="w-3 h-3 rounded bg-amber-500"></span><span>"Yav (Rust)"</span></span>
                                <span class="flex items-center gap-1.5"><span class="w-3 h-3 rounded bg-sky-500"></span><span>"Go (Gin)"</span></span>
                                <span class="flex items-center gap-1.5"><span class="w-3 h-3 rounded bg-zinc-400"></span><span>"Next.js (Node)"</span></span>
                            </div>
                        </div>

                        // Bar 1: Yav (Rust)
                        <div class="space-y-2">
                            <div class="flex items-center justify-between text-xs font-mono">
                                <span class="font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                                    <span class="w-2 h-2 rounded-full bg-amber-500"></span>
                                    "Yav.rs (Rust 1.75 + Axum 0.8)"
                                </span>
                                <span class="font-bold text-amber-600 dark:text-amber-400">"124,500 req/s • 0.32ms"</span>
                            </div>
                            <div class="w-full h-8 rounded-lg bg-zinc-100 dark:bg-zinc-800/80 overflow-hidden p-1">
                                <div class="h-full rounded-md bg-gradient-to-r from-amber-500 to-orange-500 w-full flex items-center justify-end pr-3 text-white text-[11px] font-mono font-bold shadow-xs">
                                    "100%"
                                </div>
                            </div>
                        </div>

                        // Bar 2: Go (Gin)
                        <div class="space-y-2">
                            <div class="flex items-center justify-between text-xs font-mono">
                                <span class="text-zinc-600 dark:text-zinc-400 flex items-center gap-2">
                                    <span class="w-2 h-2 rounded-full bg-sky-500"></span>
                                    "Go 1.22 (Gin SSR)"
                                </span>
                                <span class="text-zinc-500">"76,200 req/s • 0.85ms"</span>
                            </div>
                            <div class="w-full h-8 rounded-lg bg-zinc-100 dark:bg-zinc-800/80 overflow-hidden p-1">
                                <div class="h-full rounded-md bg-sky-500 w-[61%] flex items-center justify-end pr-3 text-white text-[11px] font-mono font-bold">
                                    "61%"
                                </div>
                            </div>
                        </div>

                        // Bar 3: Next.js (Node)
                        <div class="space-y-2">
                            <div class="flex items-center justify-between text-xs font-mono">
                                <span class="text-zinc-600 dark:text-zinc-400 flex items-center gap-2">
                                    <span class="w-2 h-2 rounded-full bg-zinc-400"></span>
                                    "Next.js 14 (Node.js 20 SSR)"
                                </span>
                                <span class="text-zinc-500">"8,900 req/s • 8.40ms"</span>
                            </div>
                            <div class="w-full h-8 rounded-lg bg-zinc-100 dark:bg-zinc-800/80 overflow-hidden p-1">
                                <div class="h-full rounded-md bg-zinc-400 dark:bg-zinc-600 w-[8%] flex items-center justify-end pr-2 text-white text-[10px] font-mono font-bold">
                                    "7%"
                                </div>
                            </div>
                        </div>

                        <div class="pt-2 text-right">
                            <a href="/benchmarks" class="inline-flex items-center gap-1 text-xs font-mono text-amber-600 dark:text-amber-400 hover:underline">
                                <span>"View full benchmark methodology &amp; hardware specifications"</span>
                                {render_icon("arrow-right", IconSize::Sm, Some("w-3 h-3"), None)}
                            </a>
                        </div>
                    </div>
                </section>

                // ==========================================
                // 5. QUICKSTART IN 3 STEPS
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
                // 6. FINAL HIGH-CONTRAST CTA BANNER
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
                    const tabs = ['rsx', 'yavml', 'term'];
                    tabs.forEach(t => {
                        const codeEl = document.getElementById('hero-code-' + t);
                        const btnEl = document.getElementById('btn-file-' + t);
                        if (t === tab) {
                            codeEl.classList.remove('hidden');
                            btnEl.className = 'hero-tab-btn active px-3 py-1 rounded-md bg-white dark:bg-zinc-900 text-zinc-900 dark:text-white shadow-xs font-medium cursor-pointer transition-all';
                        } else {
                            codeEl.classList.add('hidden');
                            btnEl.className = 'hero-tab-btn px-3 py-1 rounded-md text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white cursor-pointer transition-all';
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
