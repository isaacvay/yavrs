//! Interactive UI Component Gallery & Icon Explorer for Yav.rs.
//! Inspired by Tailwind UI, Catalyst, and Framer Motion.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn components(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="max-w-7xl mx-auto px-4 sm:px-8 py-10 space-y-16">
                // Header (Tailwind & Next.js style)
                <div class="space-y-4 max-w-3xl">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-purple-50 dark:bg-purple-950/40 border border-purple-200 dark:border-purple-800 text-xs font-mono text-purple-700 dark:text-purple-400">
                        {render_icon("box", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                        <span>"yav-ui &amp; yav-icons Catalog • Built-in"</span>
                    </div>
                    <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight text-zinc-900 dark:text-white">
                        "Built-in Component System"
                    </h1>
                    <p class="text-base sm:text-lg text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                        "Accessible, server-rendered components and 1,000+ vector icons pre-optimized in Rust. No heavy client-side UI libraries required."
                    </p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-12 gap-10">
                    // Sidebar (3 cols)
                    <aside class="hidden lg:block lg:col-span-3 space-y-6 text-xs font-mono sticky top-24 h-fit">
                        <span class="text-[10px] uppercase font-bold text-zinc-400 tracking-wider">"COMPONENT CATALOG"</span>
                        <ul class="space-y-1 text-zinc-600 dark:text-zinc-400">
                            <li><a href="#buttons" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"01 Buttons &amp; Actions"</a></li>
                            <li><a href="#badges" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"02 Status Badges &amp; Tags"</a></li>
                            <li><a href="#icons" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"03 Vector Icon Catalog"</a></li>
                            <li><a href="#motion" class="block py-1 text-amber-500 font-semibold hover:text-amber-600 transition-colors">"04 Motion &amp; Spring Physics"</a></li>
                        </ul>
                        <div class="p-3.5 rounded-xl bg-zinc-100 dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 space-y-1">
                            <span class="font-bold text-zinc-900 dark:text-white block text-[11px]">"Zero JS Overhead"</span>
                            <p class="text-[11px] text-zinc-500 font-sans">"All components compile to pure HTML and CSS utility strings at build time."</p>
                        </div>
                    </aside>

                    // Content (9 cols)
                    <div class="lg:col-span-9 space-y-16">
                        // Section 1: Buttons
                        <section id="buttons" class="space-y-6">
                            <div class="border-b border-zinc-200 dark:border-zinc-800 pb-3 flex items-center justify-between">
                                <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                                    <span>"Buttons &amp; Actions"</span>
                                </h2>
                                <span class="text-xs font-mono text-zinc-400">"&lt;Button variant=...&gt;"</span>
                            </div>

                            <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-6 shadow-xs">
                                <div class="flex flex-wrap items-center gap-4">
                                    <button class="px-4 py-2.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 dark:bg-white dark:hover:bg-zinc-100 text-white dark:text-zinc-900 text-xs font-semibold shadow-xs transition-all active:scale-95 cursor-pointer">
                                        "Primary Button"
                                    </button>

                                    <button class="px-4 py-2.5 rounded-xl bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-800 dark:hover:bg-zinc-700 text-zinc-800 dark:text-zinc-200 text-xs font-medium border border-zinc-300/80 dark:border-zinc-700 shadow-xs transition-all active:scale-95 cursor-pointer">
                                        "Secondary"
                                    </button>

                                    <button class="px-4 py-2.5 rounded-xl border border-zinc-300 dark:border-zinc-700 hover:border-zinc-400 dark:hover:border-zinc-600 text-zinc-800 dark:text-zinc-200 text-xs font-medium transition-all active:scale-95 cursor-pointer">
                                        "Outline"
                                    </button>

                                    <button class="px-4 py-2.5 rounded-xl bg-amber-500 hover:bg-amber-600 text-zinc-950 font-bold text-xs shadow-xs transition-all active:scale-95 cursor-pointer flex items-center gap-1.5">
                                        {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                        <span>"Accent Action"</span>
                                    </button>

                                    <button class="px-4 py-2.5 rounded-xl bg-red-600 hover:bg-red-700 text-white text-xs font-semibold shadow-xs transition-all active:scale-95 cursor-pointer flex items-center gap-1.5">
                                        <span>"Destructive"</span>
                                    </button>
                                </div>

                                <div class="p-4 rounded-xl bg-zinc-950 font-mono text-xs text-zinc-300 space-y-1 overflow-x-auto shadow-inner">
                                    <div class="text-zinc-500">"// RSX Button Variant Syntax"</div>
                                    <div><span class="token-tag">"&lt;button"</span> <span class="token-attr">"class="</span><span class="token-string">"\"px-4 py-2 rounded-xl bg-zinc-900 text-white\""</span><span class="token-tag">"&gt;"</span><span>"\"Save changes\""</span><span class="token-tag">"&lt;/button&gt;"</span></div>
                                </div>
                            </div>
                        </section>

                        // Section 2: Badges
                        <section id="badges" class="space-y-6">
                            <div class="border-b border-zinc-200 dark:border-zinc-800 pb-3 flex items-center justify-between">
                                <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                                    <span>"Status Badges &amp; Tags"</span>
                                </h2>
                                <span class="text-xs font-mono text-zinc-400">"&lt;Badge variant=...&gt;"</span>
                            </div>

                            <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-6 shadow-xs">
                                <div class="flex flex-wrap gap-3">
                                    <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-emerald-50 dark:bg-emerald-950/60 border border-emerald-200 dark:border-emerald-800 text-xs font-mono text-emerald-700 dark:text-emerald-400">
                                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                                        "Operational"
                                    </span>

                                    <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-amber-50 dark:bg-amber-950/60 border border-amber-200 dark:border-amber-800 text-xs font-mono text-amber-700 dark:text-amber-400">
                                        <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
                                        "Rebuilding"
                                    </span>

                                    <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-sky-50 dark:bg-sky-950/60 border border-sky-200 dark:border-sky-800 text-xs font-mono text-sky-700 dark:text-sky-400">
                                        <span class="w-1.5 h-1.5 rounded-full bg-sky-500"></span>
                                        "Axum 0.8 Route"
                                    </span>

                                    <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-purple-50 dark:bg-purple-950/60 border border-purple-200 dark:border-purple-800 text-xs font-mono text-purple-700 dark:text-purple-400">
                                        "Rust 2021"
                                    </span>

                                    <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-xs font-mono text-zinc-700 dark:text-zinc-300">
                                        "0 KB Hydration"
                                    </span>
                                </div>
                            </div>
                        </section>

                        // Section 3: Vector Icons
                        <section id="icons" class="space-y-6">
                            <div class="border-b border-zinc-200 dark:border-zinc-800 pb-3 flex items-center justify-between">
                                <div>
                                    <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                                        <span>"Integrated Vector Icons"</span>
                                    </h2>
                                    <p class="text-xs text-zinc-500 font-sans mt-0.5">
                                        "Over 1,000+ vector icons compiled directly into your binary via `yav::icons`."
                                    </p>
                                </div>
                                <span class="text-xs font-mono text-zinc-400">"render_icon(\"name\", ...)"</span>
                            </div>

                            <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 shadow-xs">
                                <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 gap-4 text-xs font-mono">
                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/50 flex flex-col items-center justify-center gap-2.5 text-center hover:border-amber-500/60 transition-colors">
                                        {render_icon("terminal", IconSize::Md, Some("text-amber-500"), None)}
                                        <span class="text-zinc-600 dark:text-zinc-400 text-[11px]">"terminal"</span>
                                    </div>

                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/50 flex flex-col items-center justify-center gap-2.5 text-center hover:border-amber-500/60 transition-colors">
                                        {render_icon("server", IconSize::Md, Some("text-sky-500"), None)}
                                        <span class="text-zinc-600 dark:text-zinc-400 text-[11px]">"server"</span>
                                    </div>

                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/50 flex flex-col items-center justify-center gap-2.5 text-center hover:border-amber-500/60 transition-colors">
                                        {render_icon("shield", IconSize::Md, Some("text-emerald-500"), None)}
                                        <span class="text-zinc-600 dark:text-zinc-400 text-[11px]">"shield"</span>
                                    </div>

                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/50 flex flex-col items-center justify-center gap-2.5 text-center hover:border-amber-500/60 transition-colors">
                                        {render_icon("cpu", IconSize::Md, Some("text-purple-500"), None)}
                                        <span class="text-zinc-600 dark:text-zinc-400 text-[11px]">"cpu"</span>
                                    </div>

                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/50 flex flex-col items-center justify-center gap-2.5 text-center hover:border-amber-500/60 transition-colors">
                                        {render_icon("box", IconSize::Md, Some("text-amber-500"), None)}
                                        <span class="text-zinc-600 dark:text-zinc-400 text-[11px]">"box"</span>
                                    </div>

                                    <div class="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/50 flex flex-col items-center justify-center gap-2.5 text-center hover:border-amber-500/60 transition-colors">
                                        {render_icon("activity", IconSize::Md, Some("text-emerald-500"), None)}
                                        <span class="text-zinc-600 dark:text-zinc-400 text-[11px]">"activity"</span>
                                    </div>
                                </div>
                            </div>
                        </section>

                        // Section 4: Motion & Spring Physics
                        <section id="motion" class="space-y-6 pt-8 border-t border-zinc-200 dark:border-zinc-800">
                            <div class="border-b border-zinc-200 dark:border-zinc-800 pb-3 flex items-center justify-between">
                                <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                                    <span>"Motion &amp; Spring Physics (yav-motion)"</span>
                                </h2>
                                <span class="text-xs font-mono text-amber-500 font-semibold">"&lt;Motion /&gt; • 0 KB Extra JS"</span>
                            </div>

                            <p class="text-xs sm:text-sm text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "Declarative, physics-based motion primitives directly inspired by Framer Motion. Spring dynamics, hover/tap micro-interactions, scroll progress tracking, and inView reveal triggers powered by CSS transforms and a lightweight client runtime."
                            </p>

                            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                                // Interactive Card 1: Spring Physics
                                <div
                                    class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 flex flex-col justify-between space-y-4 cursor-pointer select-none card-hover"
                                    data-motion="true"
                                    data-motion-hover="{\"scale\":1.04,\"y\":-4}"
                                    data-motion-tap="{\"scale\":0.96}"
                                    data-motion-transition="{\"is_spring\":true,\"stiffness\":300,\"damping\":16}"
                                >
                                    <div class="space-y-2">
                                        <div class="w-8 h-8 rounded-lg bg-amber-500/10 text-amber-500 flex items-center justify-center">
                                            {render_icon("activity", IconSize::Sm, None, None)}
                                        </div>
                                        <h3 class="text-sm font-bold text-zinc-900 dark:text-white">"Spring Physics"</h3>
                                        <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed">
                                            "Hover over or tap this card. Notice the snappy momentum snap governed by stiffness=300 and damping=16."
                                        </p>
                                    </div>
                                    <div class="px-2.5 py-1 rounded bg-amber-50 dark:bg-amber-950/40 border border-amber-200 dark:border-amber-800/80 text-[10px] font-mono text-amber-700 dark:text-amber-400 text-center font-medium">
                                        "Hover &amp; Press me"
                                    </div>
                                </div>

                                // Interactive Card 2: 3D Gesture
                                <div
                                    class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 flex flex-col justify-between space-y-4 cursor-pointer select-none card-hover"
                                    data-motion="true"
                                    data-motion-hover="{\"scale\":1.03,\"rotate\":2}"
                                    data-motion-tap="{\"scale\":0.95,\"rotate\":-2}"
                                    data-motion-transition="{\"is_spring\":true,\"stiffness\":260,\"damping\":18}"
                                >
                                    <div class="space-y-2">
                                        <div class="w-8 h-8 rounded-lg bg-sky-500/10 text-sky-500 flex items-center justify-center">
                                            {render_icon("zap", IconSize::Sm, None, None)}
                                        </div>
                                        <h3 class="text-sm font-bold text-zinc-900 dark:text-white">"Gesture Micro-Interactions"</h3>
                                        <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed">
                                            "Combines scale and rotation during click gesture for tactile physical response on interactive UI."
                                        </p>
                                    </div>
                                    <div class="px-2.5 py-1 rounded bg-sky-50 dark:bg-sky-950/40 border border-sky-200 dark:border-sky-800/80 text-[10px] font-mono text-sky-700 dark:text-sky-400 text-center font-medium">
                                        "while_tap &amp; while_hover"
                                    </div>
                                </div>

                                // Interactive Card 3: Scroll Progress
                                <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 flex flex-col justify-between space-y-4 shadow-xs">
                                    <div class="space-y-2">
                                        <div class="w-8 h-8 rounded-lg bg-emerald-500/10 text-emerald-500 flex items-center justify-center">
                                            {render_icon("shield", IconSize::Sm, None, None)}
                                        </div>
                                        <h3 class="text-sm font-bold text-zinc-900 dark:text-white">"Scroll Progress Bar"</h3>
                                        <p class="text-xs text-zinc-500 dark:text-zinc-400 font-sans leading-relaxed">
                                            "Look at the top edge of your browser window. The &lt;ScrollProgressBar /&gt; component tracks document scroll depth in real-time."
                                        </p>
                                    </div>
                                    <div class="px-2.5 py-1 rounded bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-200 dark:border-emerald-800/80 text-[10px] font-mono text-emerald-700 dark:text-emerald-400 text-center font-medium">
                                        "Active at top: h-[2.5px]"
                                    </div>
                                </div>
                            </div>
                        </section>
                    </div>
                </div>
            </div>
        </AppLayout>
    };
    HtmlPage::new(page)
        .title("UI Component System — Yav.rs")
        .favicon("/public/favicon.svg")
}