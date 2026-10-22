//! Architecture and Internal Pipeline page for Yav.rs.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn about(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="max-w-7xl mx-auto px-4 sm:px-8 py-10 space-y-16">
                // Header
                <div class="space-y-4 max-w-3xl">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-sky-50 dark:bg-sky-950/40 border border-sky-200 dark:border-sky-800 text-xs font-mono text-sky-700 dark:text-sky-400">
                        {render_icon("layers", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                        <span>"System Architecture &amp; Lifecycle"</span>
                    </div>
                    <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight text-zinc-900 dark:text-white">
                        "Designed for deterministic speed."
                    </h1>
                    <p class="text-base sm:text-lg text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                        "Yav combines the raw throughput of Axum 0.8 and Hyper with compile-time RSX code generation, producing a single, self-contained native binary."
                    </p>
                </div>

                // Request Flow Pipeline
                <section class="space-y-6">
                    <div class="flex items-center justify-between border-b border-zinc-200 dark:border-zinc-800 pb-3">
                        <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                            <span>"Three-Stage Execution Pipeline"</span>
                        </h2>
                        <span class="text-xs font-mono text-zinc-400">"Sub-millisecond latency path"</span>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 font-mono text-xs">
                        // Stage 1
                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-4 flex flex-col justify-between shadow-xs card-hover cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":20}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.01}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22}"
                        >
                            <div class="space-y-3">
                                <div class="flex items-center justify-between">
                                    <span class="px-2 py-0.5 rounded text-[10px] bg-zinc-100 dark:bg-zinc-800 font-bold text-zinc-600 dark:text-zinc-400">
                                        "STAGE 01"
                                    </span>
                                    {render_icon("server", IconSize::Sm, Some("w-4 h-4 text-zinc-400"), None)}
                                </div>
                                <h3 class="text-base font-bold text-zinc-900 dark:text-white font-sans">
                                    "Radix Socket Resolution"
                                </h3>
                                <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                    "Incoming HTTP/1.1 or HTTP/2 TCP sockets are handled by Tokio worker threads. Axum evaluates route paths using Matchit radix trees in nanoseconds without dynamic allocation."
                                </p>
                            </div>
                            <span class="text-[11px] text-zinc-400 pt-2 border-t border-zinc-100 dark:border-zinc-800/80">
                                "Input: Raw TCP Stream"
                            </span>
                        </div>

                        // Stage 2
                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-4 flex flex-col justify-between shadow-xs card-hover cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":20}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.01}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22,\"delay_ms\":80}"
                        >
                            <div class="space-y-3">
                                <div class="flex items-center justify-between">
                                    <span class="px-2 py-0.5 rounded text-[10px] bg-amber-50 dark:bg-amber-950/60 font-bold text-amber-600 dark:text-amber-400 border border-amber-200 dark:border-amber-800">
                                        "STAGE 02"
                                    </span>
                                    {render_icon("code", IconSize::Sm, Some("w-4 h-4 text-amber-500"), None)}
                                </div>
                                <h3 class="text-base font-bold text-zinc-900 dark:text-white font-sans">
                                    "RSX Buffer Generation"
                                </h3>
                                <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                    "Your route handler executes and evaluates its compiled RSX tree. Nodes write static string slices and escaped dynamic expressions into a contiguous memory buffer."
                                </p>
                            </div>
                            <span class="text-[11px] text-amber-600 dark:text-amber-400 font-medium pt-2 border-t border-zinc-100 dark:border-zinc-800/80">
                                "Zero VDOM overhead"
                            </span>
                        </div>

                        // Stage 3
                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-4 flex flex-col justify-between shadow-xs card-hover cursor-pointer"
                            data-motion="true"
                            data-motion-inview="true"
                            data-motion-initial="{\"opacity\":0,\"y\":20}"
                            data-motion-animate="{\"opacity\":1,\"y\":0}"
                            data-motion-hover="{\"y\":-3,\"scale\":1.01}"
                            data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":22,\"delay_ms\":160}"
                        >
                            <div class="space-y-3">
                                <div class="flex items-center justify-between">
                                    <span class="px-2 py-0.5 rounded text-[10px] bg-emerald-50 dark:bg-emerald-950/60 font-bold text-emerald-600 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-800">
                                        "STAGE 03"
                                    </span>
                                    {render_icon("zap", IconSize::Sm, Some("w-4 h-4 text-emerald-500"), None)}
                                </div>
                                <h3 class="text-base font-bold text-zinc-900 dark:text-white font-sans">
                                    "Direct Wire Streaming"
                                </h3>
                                <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                    "The completed buffer is passed directly to Hyper with automatic HTTP/2 multiplexing and optional gzip/brotli compression. Browsers receive paint-ready HTML instantly."
                                </p>
                            </div>
                            <span class="text-[11px] text-emerald-600 dark:text-emerald-400 font-medium pt-2 border-t border-zinc-100 dark:border-zinc-800/80">
                                "Time-to-first-byte: &lt; 0.4ms"
                            </span>
                        </div>
                    </div>
                </section>

                // Crate Composition Matrix
                <section class="space-y-6">
                    <div class="flex items-center justify-between border-b border-zinc-200 dark:border-zinc-800 pb-3">
                        <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                            <span>"Modular Crate Composition"</span>
                        </h2>
                        <span class="text-xs font-mono text-zinc-400">"Clear boundaries &amp; zero bloat"</span>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 text-xs font-mono">
                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-2">
                            <div class="flex items-center justify-between">
                                <span class="font-bold text-zinc-900 dark:text-white">"yav-core"</span>
                                <span class="text-zinc-400 text-[10px]">"Foundation"</span>
                            </div>
                            <p class="text-zinc-500 font-sans text-xs leading-relaxed">
                                "Virtual DOM representation, fast HTML string escaping, context provider, and I18n translation engine."
                            </p>
                        </div>

                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-2">
                            <div class="flex items-center justify-between">
                                <span class="font-bold text-zinc-900 dark:text-white">"yav-macros"</span>
                                <span class="text-amber-500 text-[10px]">"Proc Macros"</span>
                            </div>
                            <p class="text-zinc-500 font-sans text-xs leading-relaxed">
                                "Procedural macros: `rsx!`, `#[component]`, and compile-time file-system scanner `auto_routes!`."
                            </p>
                        </div>

                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-2">
                            <div class="flex items-center justify-between">
                                <span class="font-bold text-zinc-900 dark:text-white">"yav-server"</span>
                                <span class="text-sky-500 text-[10px]">"Axum 0.8"</span>
                            </div>
                            <p class="text-zinc-500 font-sans text-xs leading-relaxed">
                                "Application server orchestrator, form actions, CSRF protection, signed cookies, and OpenAPI generator."
                            </p>
                        </div>

                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-2">
                            <div class="flex items-center justify-between">
                                <span class="font-bold text-zinc-900 dark:text-white">"yav-router"</span>
                                <span class="text-emerald-500 text-[10px]">"Matchit"</span>
                            </div>
                            <p class="text-zinc-500 font-sans text-xs leading-relaxed">
                                "High-speed radix tree routing engine with support for dynamic URL parameters and catch-all wildcards."
                            </p>
                        </div>

                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-2">
                            <div class="flex items-center justify-between">
                                <span class="font-bold text-zinc-900 dark:text-white">"yav-build"</span>
                                <span class="text-purple-500 text-[10px]">"Pipeline"</span>
                            </div>
                            <p class="text-zinc-500 font-sans text-xs leading-relaxed">
                                "Supervisor for standalone Tailwind CSS binaries, asset fingerprinting, minification, and bundling."
                            </p>
                        </div>

                        <div class="p-5 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/40 space-y-2">
                            <div class="flex items-center justify-between">
                                <span class="font-bold text-zinc-900 dark:text-white">"yav-cli"</span>
                                <span class="text-zinc-400 text-[10px]">"Toolchain"</span>
                            </div>
                            <p class="text-zinc-500 font-sans text-xs leading-relaxed">
                                "Development CLI (`dev`, `build`, `new`, `update`) with live WebSocket reload and instant file watchers."
                            </p>
                        </div>
                    </div>
                </section>
            </div>
        </AppLayout>
    };
    HtmlPage::new(page)
        .title("Architecture & Internals — Yav.rs")
        .favicon("/public/favicon.svg")
}
