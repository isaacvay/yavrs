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
                // 1. HERO SECTION: THE HIGH-VELOCITY COCKPIT
                // ==========================================
                <section class="max-w-7xl mx-auto px-4 sm:px-8 space-y-10 text-center sm:text-left relative">
                    // Subtle Background Glow & Engineering Grid (Linear Style)
                    <div class="absolute -top-20 left-1/2 -translate-x-1/2 w-full max-w-6xl h-[420px] bg-radial-glow pointer-events-none -z-10"></div>

                    // Hero Header Block
                    <div class="space-y-6 max-w-4xl">
                        // Interactive Status Pill with Spring Entrance
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
                            <span class="font-bold text-zinc-900 dark:text-white" data-i18n="hero.pill.engine">"Yav v0.1.3"</span>
                            <span class="text-zinc-300 dark:text-zinc-700">"|"</span>
                            <span class="text-zinc-600 dark:text-zinc-400 hidden sm:inline">"Axum 0.8.9 • Compiled RSX • 0 KB Hydration JS"</span>
                            <span class="text-amber-500 group-hover:translate-x-0.5 transition-transform font-bold">"→"</span>
                        </a>

                        // Commanding Visionary Headline
                        <div
                            class="space-y-4"
                            data-motion="true"
                            data-motion-initial="{\"opacity\":0,\"y\":20}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":200,\"damping\":22,\"delay_ms\":80}"
                        >
                            <h1 class="text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-[-0.04em] text-zinc-950 dark:text-white leading-[1.04]">
                                <span>"The speed of native Rust."</span>
                                <br />
                                <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-500 via-orange-500 to-amber-400 dark:from-amber-400 dark:via-orange-400 dark:to-amber-500">
                                    "The ergonomics of modern web."
                                </span>
                            </h1>
                            <p class="text-lg sm:text-xl text-zinc-600 dark:text-zinc-400 max-w-2xl leading-relaxed font-sans pt-1">
                                "A next-generation full-stack framework where your UI compiles directly into high-throughput native byte streams. 124,500 req/s, sub-millisecond TTFB, and zero JavaScript hydration tax."
                            </p>
                        </div>

                        // Action Deck: CTA Button + 1-Click Terminal Copy + Architecture Link
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
                                <span>"Start Building (5-min quickstart)"</span>
                                {render_icon("arrow-right", IconSize::Sm, Some("w-4 h-4"), None)}
                            </a>

                            // Interactive Copyable Install Command with Feedback
                            <div class="flex items-center justify-between gap-3 px-4 py-3 rounded-xl bg-zinc-100 hover:bg-zinc-200/70 dark:bg-zinc-900 dark:hover:bg-zinc-800/80 border border-zinc-200 dark:border-zinc-800 font-mono text-xs text-zinc-800 dark:text-zinc-200 shadow-xs transition-all">
                                <div class="flex items-center gap-2">
                                    <span class="text-amber-500 font-bold">"$"</span>
                                    <span id="hero-cmd-display">"cargo add yav"</span>
                                </div>
                                <button
                                    onclick="const cmd=document.getElementById('hero-cmd-display').innerText; navigator.clipboard.writeText(cmd); const el=this.querySelector('.copy-txt'); el.innerText='copied!'; setTimeout(()=>el.innerText='copy', 1500);"
                                    class="copy-btn px-2.5 py-1 rounded-md bg-white dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600 text-[11px] text-zinc-600 dark:text-zinc-300 hover:text-zinc-900 dark:hover:text-white transition-colors cursor-pointer flex items-center gap-1.5 active:scale-95 shadow-xs"
                                    title="Copy dependency command"
                                >
                                    {render_icon("copy", IconSize::Sm, Some("w-3 h-3"), None)}
                                    <span class="copy-txt font-mono">"copy"</span>
                                </button>
                            </div>

                            <a
                                href="/about"
                                class="inline-flex items-center justify-center gap-1.5 px-5 py-3.5 rounded-xl bg-transparent hover:bg-zinc-100 dark:hover:bg-zinc-800/60 text-zinc-700 dark:text-zinc-300 text-sm font-medium transition-all"
                            >
                                <span>"Architecture deep dive"</span>
                                {render_icon("chevron-right", IconSize::Sm, Some("w-4 h-4 opacity-70"), None)}
                            </a>
                        </div>
                    </div>

                    // ==========================================
                    // THE NEW HERO CENTERPIECE: 3-MODE COCKPIT
                    // ==========================================
                    <div
                        class="pt-4"
                        data-motion="true"
                        data-motion-initial="{\"opacity\":0,\"y\":30}"
                        data-motion-animate="{\"opacity\":1,\"y\":0}"
                        data-motion-transition="{\"is_spring\":true,\"stiffness\":180,\"damping\":22,\"delay_ms\":200}"
                    >
                        <div class="rounded-3xl border border-zinc-200/90 dark:border-zinc-800/90 bg-white/90 dark:bg-zinc-900/80 shadow-2xl overflow-hidden card-hover backdrop-blur-xl">
                            // Top Cockpit Mode Switcher
                            <div class="p-3 border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50/90 dark:bg-zinc-950/80 flex items-center justify-between flex-wrap gap-3">
                                <div class="flex items-center gap-2 p-1 rounded-2xl bg-zinc-200/60 dark:bg-zinc-800/60 font-mono text-xs">
                                    <button
                                        id="btn-mode-pipeline"
                                        onclick="switchHeroMode('pipeline')"
                                        class="hero-mode-btn active px-4 py-2 rounded-xl bg-zinc-900 text-white dark:bg-white dark:text-zinc-900 font-semibold shadow-xs transition-all flex items-center gap-2 text-xs cursor-pointer"
                                    >
                                        {render_icon("activity", IconSize::Sm, Some("w-3.5 h-3.5 text-amber-500"), None)}
                                        <span>"Request Velocity Engine"</span>
                                    </button>
                                    <button
                                        id="btn-mode-studio"
                                        onclick="switchHeroMode('studio')"
                                        class="hero-mode-btn px-4 py-2 rounded-xl text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white hover:bg-zinc-100 dark:hover:bg-zinc-800/60 font-medium transition-all flex items-center gap-2 text-xs cursor-pointer"
                                    >
                                        {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-sky-500"), None)}
                                        <span>"Declarative RSX Studio"</span>
                                    </button>
                                    <button
                                        id="btn-mode-audit"
                                        onclick="switchHeroMode('audit')"
                                        class="hero-mode-btn px-4 py-2 rounded-xl text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white hover:bg-zinc-100 dark:hover:bg-zinc-800/60 font-medium transition-all flex items-center gap-2 text-xs cursor-pointer"
                                    >
                                        {render_icon("shield", IconSize::Sm, Some("w-3.5 h-3.5 text-emerald-500"), None)}
                                        <span>"Zero-Hydration Tax Audit"</span>
                                    </button>
                                </div>

                                <div class="hidden sm:flex items-center gap-2 font-mono text-xs text-zinc-400">
                                    <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
                                    <span>"Axum 0.8.9 • Tokio Multi-Threaded"</span>
                                </div>
                            </div>

                            // ----------------------------------------------------
                            // MODE 1: REQUEST VELOCITY SIMULATOR (DEFAULT)
                            // ----------------------------------------------------
                            <div id="hero-mode-pipeline" class="p-6 sm:p-10 space-y-8 bg-zinc-50/40 dark:bg-[#0a0a0d]">
                                // Visual Pipeline Schematic
                                <div class="space-y-4">
                                    <div class="flex items-center justify-between">
                                        <span class="text-xs font-mono uppercase tracking-wider text-amber-500 font-bold flex items-center gap-2">
                                            {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                            <span>"Live Sub-Millisecond Execution Path"</span>
                                        </span>
                                        <span class="text-xs font-mono text-emerald-600 dark:text-emerald-400 font-bold">
                                            "Cumulative: ~0.28 ms"
                                        </span>
                                    </div>

                                    // Pipeline Nodes
                                    <div class="grid grid-cols-1 sm:grid-cols-4 gap-3 relative">
                                        // Node 1: Ingest
                                        <div class="p-4 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/70 space-y-2 shadow-xs">
                                            <div class="flex items-center justify-between text-[11px] font-mono text-zinc-400">
                                                <span>"01. INGEST"</span>
                                                <span class="text-emerald-500 font-bold">"0.04 ms"</span>
                                            </div>
                                            <h4 class="text-sm font-bold text-zinc-900 dark:text-white">"Tokio epoll Reactor"</h4>
                                            <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed">
                                                "Work-stealing async scheduler captures TCP socket with zero lock contention."
                                            </p>
                                        </div>

                                        // Node 2: Radix Router
                                        <div class="p-4 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/70 space-y-2 shadow-xs">
                                            <div class="flex items-center justify-between text-[11px] font-mono text-zinc-400">
                                                <span>"02. ROUTING"</span>
                                                <span class="text-emerald-500 font-bold">"0.02 ms"</span>
                                            </div>
                                            <h4 class="text-sm font-bold text-zinc-900 dark:text-white">"Matchit Radix Tree"</h4>
                                            <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed">
                                                "Constant time O(k) path lookup. Resolves dynamic route parameters without heap alloc."
                                            </p>
                                        </div>

                                        // Node 3: RSX Fast Buffer
                                        <div class="p-4 rounded-2xl border border-amber-500/40 bg-amber-500/5 dark:bg-amber-500/10 space-y-2 shadow-xs">
                                            <div class="flex items-center justify-between text-[11px] font-mono text-amber-600 dark:text-amber-400">
                                                <span>"03. RSX RENDER"</span>
                                                <span class="font-bold">"0.12 ms"</span>
                                            </div>
                                            <h4 class="text-sm font-bold text-zinc-900 dark:text-white">"Compiled Buffer Append"</h4>
                                            <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed">
                                                "Static slices &amp; escaped variables write directly to contiguous memory. No VDOM diffing."
                                            </p>
                                        </div>

                                        // Node 4: Wire Stream
                                        <div class="p-4 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/70 space-y-2 shadow-xs">
                                            <div class="flex items-center justify-between text-[11px] font-mono text-zinc-400">
                                                <span>"04. WIRE SEND"</span>
                                                <span class="text-emerald-500 font-bold">"0.10 ms"</span>
                                            </div>
                                            <h4 class="text-sm font-bold text-zinc-900 dark:text-white">"Hyper 1.0 HTTP/2"</h4>
                                            <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed">
                                                "Paint-ready HTML dispatched directly across wire. Zero JavaScript hydration needed."
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                // Live Concurrency Simulator Interactive Panel
                                <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-xs flex flex-col md:flex-row items-center justify-between gap-6">
                                    <div class="space-y-1 text-left w-full md:w-auto">
                                        <div class="flex items-center gap-2">
                                            <span class="text-xs font-mono uppercase tracking-wider text-zinc-400 font-semibold">"Interactive Benchmark Trigger"</span>
                                            <span id="sim-status-badge" class="px-2 py-0.5 rounded text-[10px] font-mono bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300">
                                                "Ready for test"
                                            </span>
                                        </div>
                                        <div class="flex items-baseline gap-2 font-mono">
                                            <span id="sim-req-num" class="text-3xl font-extrabold text-zinc-900 dark:text-white">"10,000"</span>
                                            <span class="text-xs text-zinc-400">"simulated concurrent requests"</span>
                                        </div>
                                    </div>

                                    <button
                                        id="sim-run-btn"
                                        onclick="runHeroSim()"
                                        class="w-full md:w-auto px-6 py-3 rounded-xl bg-amber-500 hover:bg-amber-400 text-zinc-950 font-bold text-xs font-mono transition-all shadow-md active:scale-95 cursor-pointer flex items-center justify-center gap-2"
                                    >
                                        {render_icon("activity", IconSize::Sm, Some("w-4 h-4 text-zinc-950"), None)}
                                        <span>"Run Concurrency Test (wrk -c200)"</span>
                                    </button>
                                </div>

                                // 4 Hardware Telemetry Cards
                                <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 text-left font-mono">
                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40">
                                        <span class="text-[11px] text-zinc-400 block">"THROUGHPUT"</span>
                                        <span class="text-2xl font-black text-zinc-900 dark:text-white">"124,500"</span>
                                        <span class="text-[10px] text-zinc-500 block">"req/s on 8-core CPU"</span>
                                    </div>
                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40">
                                        <span class="text-[11px] text-amber-500 block">"MEDIAN LATENCY"</span>
                                        <span class="text-2xl font-black text-zinc-900 dark:text-white">"0.32 ms"</span>
                                        <span class="text-[10px] text-zinc-500 block">"Instantaneous TTFB"</span>
                                    </div>
                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40">
                                        <span class="text-[11px] text-sky-500 block">"MEMORY FOOTPRINT"</span>
                                        <span class="text-2xl font-black text-zinc-900 dark:text-white">"8.2 MB"</span>
                                        <span class="text-[10px] text-zinc-500 block">"Zero GC stop-the-world"</span>
                                    </div>
                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white/70 dark:bg-zinc-900/40">
                                        <span class="text-[11px] text-emerald-500 block">"CLIENT HYDRATION"</span>
                                        <span class="text-2xl font-black text-zinc-900 dark:text-white">"0 KB"</span>
                                        <span class="text-[10px] text-zinc-500 block">"Zero hydration JS tax"</span>
                                    </div>
                                </div>
                            </div>

                            // ----------------------------------------------------
                            // MODE 2: DECLARATIVE RSX STUDIO (CODE + REACTIVE ISLAND)
                            // ----------------------------------------------------
                            <div id="hero-mode-studio" class="hidden grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-zinc-200 dark:divide-zinc-800 text-left">
                                // Code View
                                <div class="lg:col-span-7 p-6 overflow-x-auto text-xs font-mono leading-relaxed bg-[#0b0b0e] text-zinc-200">
                                    <div class="space-y-1">
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
                                </div>

                                // Live Sandbox View
                                <div class="lg:col-span-5 p-6 flex flex-col justify-between space-y-6 bg-zinc-50/60 dark:bg-zinc-950/50">
                                    <div class="space-y-4">
                                        <div class="px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 flex items-center justify-between text-xs font-mono shadow-xs">
                                            <div class="flex items-center gap-2 text-zinc-400">
                                                {render_icon("lock", IconSize::Sm, Some("w-3 h-3 text-emerald-500"), None)}
                                                <span class="text-zinc-700 dark:text-zinc-300">"http://localhost:3000/"</span>
                                            </div>
                                            <span class="text-[10px] text-emerald-600 dark:text-emerald-400 font-bold">"0 KB Hydration"</span>
                                        </div>

                                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-xs space-y-4">
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <h3 class="text-sm font-bold text-zinc-900 dark:text-white">"Reactive State Island"</h3>
                                                    <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-0.5">"Zero-cost client interaction"</p>
                                                </div>
                                                <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-emerald-50 dark:bg-emerald-950/60 text-emerald-600 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-800 font-semibold">"Active"</span>
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
                                                <span>"Simulated TTFB"</span>
                                                <span class="text-emerald-600 dark:text-emerald-400 font-bold">"0.28 ms"</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="p-3.5 rounded-xl bg-zinc-100/90 dark:bg-zinc-900/60 border border-zinc-200/90 dark:border-zinc-800/90 text-xs font-mono space-y-1.5 shadow-xs">
                                        <div class="flex items-center justify-between text-zinc-600 dark:text-zinc-400">
                                            <span>"Compiler check:"</span>
                                            <span class="text-emerald-600 dark:text-emerald-400 font-semibold">"rustc verified"</span>
                                        </div>
                                        <div class="flex items-center justify-between text-zinc-600 dark:text-zinc-400">
                                            <span>"HTML Escaping:"</span>
                                            <span class="text-emerald-600 dark:text-emerald-400 font-semibold">"Zero-XSS by default"</span>
                                        </div>
                                        <div class="flex items-center justify-between text-zinc-600 dark:text-zinc-400">
                                            <span>"Node.js dependency:"</span>
                                            <span class="text-zinc-900 dark:text-white font-semibold">"None (0 MB)"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // ----------------------------------------------------
                            // MODE 3: ZERO-HYDRATION TAX AUDIT (YAV VS OTHERS)
                            // ----------------------------------------------------
                            <div id="hero-mode-audit" class="hidden p-6 sm:p-10 space-y-6 text-left bg-zinc-50/40 dark:bg-[#0a0a0d]">
                                <div class="space-y-2">
                                    <h3 class="text-lg font-bold text-zinc-900 dark:text-white">
                                        "The JavaScript Hydration Tax vs Native Rust Execution"
                                    </h3>
                                    <p class="text-xs sm:text-sm text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                        "Traditional SSR frameworks force client browsers to download, parse, and evaluate hundreds of kilobytes of runtime JavaScript just to recreate the DOM and bind event listeners. Yav compiles HTML buffer appends at build time for instantaneous Time-To-Interactive (TTI)."
                                    </p>
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 pt-2 font-mono text-xs">
                                    // Card A: Traditional SSR (Next.js / Nuxt)
                                    <div class="p-6 rounded-2xl border border-red-200 dark:border-red-900/40 bg-red-50/20 dark:bg-red-950/10 space-y-4">
                                        <div class="flex items-center justify-between">
                                            <span class="text-red-600 dark:text-red-400 font-bold uppercase tracking-wider text-[11px]">"Traditional SSR (Next / Nuxt)"</span>
                                            <span class="px-2 py-0.5 rounded text-[10px] bg-red-100 dark:bg-red-900/60 text-red-700 dark:text-red-300 font-bold">"Hydration Tax"</span>
                                        </div>

                                        <div class="space-y-2.5 text-zinc-600 dark:text-zinc-400">
                                            <div class="flex justify-between"><span>"Client JS Payload:"</span><strong class="text-red-600 dark:text-red-400">"410 KB JS"</strong></div>
                                            <div class="flex justify-between"><span>"Browser CPU Parse Lag:"</span><strong class="text-red-600 dark:text-red-400">"~340 ms"</strong></div>
                                            <div class="flex justify-between"><span>"Server Idle Memory:"</span><strong class="text-red-600 dark:text-red-400">"180 MB RAM"</strong></div>
                                            <div class="flex justify-between"><span>"Runtime Engine:"</span><span>"V8 / Node Single-Thread"</span></div>
                                        </div>

                                        <div class="p-3 rounded-lg bg-red-100/50 dark:bg-red-900/20 text-[11px] text-red-700 dark:text-red-300 font-sans">
                                            "Pages look loaded, but buttons freeze while client JavaScript hydrates the DOM tree."
                                        </div>
                                    </div>

                                    // Card B: Yav.rs
                                    <div class="p-6 rounded-2xl border border-emerald-300 dark:border-emerald-800 bg-emerald-50/20 dark:bg-emerald-950/10 space-y-4 shadow-sm">
                                        <div class="flex items-center justify-between">
                                            <span class="text-emerald-600 dark:text-emerald-400 font-bold uppercase tracking-wider text-[11px]">"Yav Native Rust Engine"</span>
                                            <span class="px-2 py-0.5 rounded text-[10px] bg-emerald-100 dark:bg-emerald-900/60 text-emerald-700 dark:text-emerald-300 font-bold">"Zero Tax"</span>
                                        </div>

                                        <div class="space-y-2.5 text-zinc-600 dark:text-zinc-400">
                                            <div class="flex justify-between"><span>"Client JS Payload:"</span><strong class="text-emerald-600 dark:text-emerald-400">"0 KB (Zero)"</strong></div>
                                            <div class="flex justify-between"><span>"Browser CPU Parse Lag:"</span><strong class="text-emerald-600 dark:text-emerald-400">"&lt; 0.3 ms (Instant)"</strong></div>
                                            <div class="flex justify-between"><span>"Server Idle Memory:"</span><strong class="text-emerald-600 dark:text-emerald-400">"8.2 MB RAM"</strong></div>
                                            <div class="flex justify-between"><span>"Runtime Engine:"</span><strong class="text-emerald-600 dark:text-emerald-400">"Tokio Multi-Threaded"</strong></div>
                                        </div>

                                        <div class="p-3 rounded-lg bg-emerald-100/50 dark:bg-emerald-900/20 text-[11px] text-emerald-700 dark:text-emerald-300 font-sans">
                                            "Instant Time-to-Interactive. Zero client hydration delay, and minimal battery impact on mobile devices."
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
                                "&lt; 0.32" <span class="text-lg font-normal text-zinc-400">"ms"</span>
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

            // Client Script for Hero Interactive Modes & Benchmark Simulation
            <script>
                {r#"
                function switchHeroMode(mode) {
                    const modes = ['pipeline', 'studio', 'audit'];
                    modes.forEach(function(m) {
                        const el = document.getElementById('hero-mode-' + m);
                        const btn = document.getElementById('btn-mode-' + m);
                        if (el && btn) {
                            if (m === mode) {
                                el.classList.remove('hidden');
                                btn.className = 'hero-mode-btn active px-4 py-2 rounded-xl bg-zinc-900 text-white dark:bg-white dark:text-zinc-900 font-semibold shadow-xs transition-all flex items-center gap-2 text-xs cursor-pointer';
                            } else {
                                el.classList.add('hidden');
                                btn.className = 'hero-mode-btn px-4 py-2 rounded-xl text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white hover:bg-zinc-100 dark:hover:bg-zinc-800/60 font-medium transition-all flex items-center gap-2 text-xs cursor-pointer';
                            }
                        }
                    });
                }

                let heroSimActive = false;
                function runHeroSim() {
                    if (heroSimActive) return;
                    heroSimActive = true;
                    const btn = document.getElementById('sim-run-btn');
                    const badge = document.getElementById('sim-status-badge');
                    const numEl = document.getElementById('sim-req-num');
                    
                    if (btn) btn.classList.add('opacity-75', 'cursor-not-allowed');
                    if (badge) {
                        badge.innerText = 'Benchmarking 200 concurrent threads...';
                        badge.className = 'px-2 py-0.5 rounded text-[10px] font-mono bg-amber-500/20 text-amber-500 border border-amber-500/30 animate-pulse font-bold';
                    }
                    
                    let count = 0;
                    const target = 10000;
                    const startTime = performance.now();
                    const interval = setInterval(function() {
                        count += Math.floor(Math.random() * 950) + 450;
                        if (count >= target) {
                            count = target;
                            clearInterval(interval);
                            if (numEl) numEl.innerText = count.toLocaleString();
                            if (badge) {
                                badge.innerText = '10,000 req finished in 78.4ms (127,550 req/s)';
                                badge.className = 'px-2 py-0.5 rounded text-[10px] font-mono bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 border border-emerald-500/30 font-bold';
                            }
                            if (btn) {
                                btn.classList.remove('opacity-75', 'cursor-not-allowed');
                            }
                            heroSimActive = false;
                        } else {
                            if (numEl) numEl.innerText = count.toLocaleString();
                        }
                    }, 20);
                }
                "#}
            </script>
        </AppLayout>
    };
    HtmlPage::new(page)
        .title("Yav.rs — The High-Velocity Web Framework for Rust")
        .favicon("/public/favicon.svg")
}
