//! Interactive UI Component Gallery & Icon Explorer for Yav.rs.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn components(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="page-shell page-components max-w-7xl mx-auto px-4 sm:px-8 py-10 space-y-16">
                // Header
                <div class="component-intro space-y-4 max-w-3xl">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-purple-50 dark:bg-purple-950/40 border border-purple-200 dark:border-purple-800 text-xs font-mono text-purple-700 dark:text-purple-400">
                        {render_icon("box", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                        <span>"yav-ui &amp; yav-icons Catalog"</span>
                    </div>
                    <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight text-zinc-900 dark:text-white">
                        "Built-in Component System"
                    </h1>
                    <p class="text-base sm:text-lg text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                        "Accessible, server-rendered components and 1,000+ vector icons pre-optimized in Rust. No heavy client-side UI libraries required."
                    </p>
                </div>

                <div class="component-catalog">
                    <aside class="catalog-sidebar" aria-label="Component catalog sections">
                        <span class="section-eyebrow">"CATALOG"</span>
                        <a href="#buttons"><span>"01"</span>"Buttons"</a>
                        <a href="#badges"><span>"02"</span>"Status &amp; tags"</a>
                        <a href="#icons"><span>"03"</span>"Icon set"</a>
                        <a href="#motion"><span>"04"</span>"Motion &amp; Physics"</a>
                        <div class="catalog-note">
                            <strong>"Server-first UI"</strong>
                            <p>"Small, accessible building blocks rendered with Rust."</p>
                        </div>
                    </aside>
                    <div class="component-content">

                // Section 1: Buttons
                <section id="buttons" class="component-section space-y-6">
                    <div class="border-b border-zinc-200 dark:border-zinc-800 pb-3 flex items-center justify-between">
                        <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                            <span>"Buttons"</span>
                        </h2>
                        <span class="text-xs font-mono text-zinc-400">"&lt;Button variant=...&gt;"</span>
                    </div>

                    <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 space-y-6">
                        <div class="button-showcase flex flex-wrap items-center gap-4">
                            <button class="px-4 py-2 rounded-xl bg-zinc-900 hover:bg-zinc-800 dark:bg-white dark:hover:bg-zinc-100 text-white dark:text-zinc-900 text-xs font-semibold shadow-xs transition-all active:scale-95 cursor-pointer">
                                "Primary Button"
                            </button>

                            <button class="px-4 py-2 rounded-xl bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-800 dark:hover:bg-zinc-700 text-zinc-800 dark:text-zinc-200 text-xs font-medium border border-zinc-300/80 dark:border-zinc-700 shadow-xs transition-all active:scale-95 cursor-pointer">
                                "Secondary"
                            </button>

                            <button class="px-4 py-2 rounded-xl border border-zinc-300 dark:border-zinc-700 hover:border-zinc-400 dark:hover:border-zinc-600 text-zinc-800 dark:text-zinc-200 text-xs font-medium transition-all active:scale-95 cursor-pointer">
                                "Outline"
                            </button>

                            <button class="px-4 py-2 rounded-xl bg-amber-500 hover:bg-amber-600 text-white text-xs font-semibold shadow-xs transition-all active:scale-95 cursor-pointer flex items-center gap-1.5">
                                {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                <span>"Accent Action"</span>
                            </button>

                            <button class="px-4 py-2 rounded-xl bg-red-600 hover:bg-red-700 text-white text-xs font-semibold shadow-xs transition-all active:scale-95 cursor-pointer flex items-center gap-1.5">
                                <span>"Destructive"</span>
                            </button>
                        </div>
                        <div class="component-code-sample">
                            <span>"RSX · BUTTON VARIANT"</span>
                            <code>&lt;button class="button-primary"&gt;Save changes&lt;/button&gt;</code>
                            <small>"Semantic HTML, styled at build time."</small>
                        </div>
                    </div>
                </section>

                // Section 2: Badges & Tags
                <section id="badges" class="component-section space-y-6">
                    <div class="border-b border-zinc-200 dark:border-zinc-800 pb-3 flex items-center justify-between">
                        <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                            <span>"Status Badges &amp; Tags"</span>
                        </h2>
                        <span class="text-xs font-mono text-zinc-400">"&lt;Badge variant=...&gt;"</span>
                    </div>

                    <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 flex flex-wrap gap-3">
                        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-emerald-50 dark:bg-emerald-950/60 border border-emerald-200 dark:border-emerald-800 text-xs font-mono text-emerald-700 dark:text-emerald-400">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                            "Operational"
                        </span>

                        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-amber-50 dark:bg-amber-950/60 border border-amber-200 dark:border-amber-800 text-xs font-mono text-amber-700 dark:text-amber-400">
                            <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
                            "Pending Rebuild"
                        </span>

                        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-sky-50 dark:bg-sky-950/60 border border-sky-200 dark:border-sky-800 text-xs font-mono text-sky-700 dark:text-sky-400">
                            <span class="w-1.5 h-1.5 rounded-full bg-sky-500"></span>
                            "Axum 0.8 Route"
                        </span>

                        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-purple-50 dark:bg-purple-950/60 border border-purple-200 dark:border-purple-800 text-xs font-mono text-purple-700 dark:text-purple-400">
                            "Rust 2021"
                        </span>

                        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-xs font-mono text-zinc-700 dark:text-zinc-300">
                            "Static Asset"
                        </span>
                    </div>
                </section>

                // Section 3: Vector Icon Showcase
                <section id="icons" class="component-section space-y-6">
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

                    <div class="p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50">
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

                // Section 4: Yav Motion & Spring Physics
                <section id="motion" class="component-section space-y-6 pt-8 border-t border-zinc-200 dark:border-zinc-800">
                    <div class="border-b border-zinc-200 dark:border-zinc-800 pb-3 flex items-center justify-between">
                        <h2 class="text-xl font-bold text-zinc-900 dark:text-white flex items-center gap-2">
                            <span>"Motion &amp; Spring Physics (yav-motion)"</span>
                        </h2>
                        <span class="text-xs font-mono text-amber-500 font-semibold">"&lt;Motion /&gt; • 0 KB Extra JS"</span>
                    </div>

                    <p class="text-xs sm:text-sm text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                        "Declarative, physics-based motion primitives directly inspired by Framer Motion. Spring dynamics, hover/tap micro-interactions, scroll progress tracking, and inView reveal triggers powered by CSS transforms and a lightweight client runtime."
                    </p>

                    // Interactive Motion Playground Grid
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // Interactive Card 1: Spring Physics on Hover & Tap
                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 flex flex-col justify-between space-y-4 cursor-pointer select-none"
                            data-motion="true"
                            data-motion-hover='{"scale":1.04,"y":-4}'
                            data-motion-tap='{"scale":0.96}'
                            data-motion-transition='{"is_spring":true,"stiffness":300,"damping":16}'
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

                        // Interactive Card 2: 3D Tilt / Rotate Gesture
                        <div
                            class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 flex flex-col justify-between space-y-4 cursor-pointer select-none"
                            data-motion="true"
                            data-motion-hover='{"scale":1.03,"rotate":2}'
                            data-motion-tap='{"scale":0.95,"rotate":-2}'
                            data-motion-transition='{"is_spring":true,"stiffness":260,"damping":18}'
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

                        // Interactive Card 3: Scroll Progress Indicator
                        <div class="p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 flex flex-col justify-between space-y-4">
                            <div class="space-y-2">
                                <div class="w-8 h-8 rounded-lg bg-emerald-500/10 text-emerald-500 flex items-center justify-center">
                                    {render_icon("sliders", IconSize::Sm, None, None)}
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

                    // Motion Rust Code Sample
                    <div class="p-5 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-zinc-950 text-zinc-200 font-mono text-xs overflow-x-auto leading-relaxed shadow-inner">
                        <div class="text-zinc-500 mb-2 font-sans text-[11px]">"// Declarative Rust RSX Motion Syntax"</div>
                        <div><span class="token-macro">"rsx!"</span><span> " {"</span></div>
                        <div class="pl-4"><span class="token-tag">"&lt;Motion"</span></div>
                        <div class="pl-8"><span class="token-attr">"initial="</span><span class="token-val">"{MotionState::new().opacity(0.0).y(20.0)}"</span></div>
                        <div class="pl-8"><span class="token-attr">"animate="</span><span class="token-val">"{MotionState::new().opacity(1.0).y(0.0)}"</span></div>
                        <div class="pl-8"><span class="token-attr">"while_hover="</span><span class="token-val">"{MotionState::new().scale(1.03)}"</span></div>
                        <div class="pl-8"><span class="token-attr">"while_tap="</span><span class="token-val">"{MotionState::new().scale(0.97)}"</span></div>
                        <div class="pl-8"><span class="token-attr">"transition="</span><span class="token-val">"{Transition::spring().stiffness(260.0).damping(18.0)}"</span></div>
                        <div class="pl-8"><span class="token-attr">"in_view="</span><span class="token-val">"{true}"</span></div>
                        <div class="pl-4"><span class="token-tag">"&gt;"</span></div>
                        <div class="pl-8"><span class="token-tag">"&lt;Card&gt;"</span><span>"\"Physical spring animations compiled in Rust\""</span><span class="token-tag">"&lt;/Card&gt;"</span></div>
                        <div class="pl-4"><span class="token-tag">"&lt;/Motion&gt;"</span></div>
                        <div><span>"}"</span></div>
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
