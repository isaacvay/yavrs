//! Architecture Hero Section — Blueprint & Design Philosophy (Airy & Generous).

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ArchHero() -> Node {
    rsx! {
        <section class="wrap pt-12 sm:pt-16 pb-12 sm:pb-16 border-b border-[rgba(236,233,227,0.08)]">
            // Top figure breadcrumb & status pill
            <div class="fade flex flex-wrap items-center justify-between gap-4 pb-6 sm:pb-8" style="--d:0ms">
                <div class="flex items-center gap-6 label">
                    <span class="label-oxide font-mono text-[13px]">"Fig. 02"</span>
                    <span class="text-[12px] tracking-widest uppercase text-paper-2">"Architectural Specification"</span>
                </div>
                <div class="chip px-4 py-1.5 h-auto">
                    <span class="dot dot-ok pulse-dot"></span>
                    <span class="text-[12px]">"Axum 0.8 · Tokio 1.40 · Matchit Radix"</span>
                </div>
            </div>

            // Main headline with generous line height & tracking
            <div class="max-w-5xl space-y-3">
                <h1 class="h-page text-paper tracking-[-0.04em]">
                    <span class="line-mask block">
                        <span style="--d:80ms">"Designed for"</span>
                    </span>
                    <span class="line-mask block pt-2 sm:pt-3">
                        <span style="--d:180ms" class="serif text-oxide">"deterministic"</span>" speed."
                    </span>
                </h1>
            </div>

            // Two-column layout with home page rhythm
            <div class="grid lg:grid-cols-12 gap-10 lg:gap-14 pt-8 sm:pt-10 items-start">
                <div class="lg:col-span-6 space-y-8 fade" style="--d:320ms">
                    <div class="space-y-6 pl-5 sm:pl-7 border-l-2 border-oxide/50">
                        <p class="lede text-[18px] sm:text-[20px] leading-[1.6] text-paper">
                            "Axum and Hyper handle the raw socket. A proc-macro turns your RSX into straight-line contiguous buffer writes. "
                            <strong class="font-semibold text-paper">"The result is a single native binary with zero interpreter, zero virtual DOM and zero garbage collector between the request and the response."</strong>
                        </p>
                        <p class="text-[15.5px] text-mute leading-[1.8]">
                            "Traditional SSR frameworks load an entire JavaScript runtime, parse trees dynamically on the heap, and run garbage collection passes during peak load. Yav computes template topology at compile-time and streams bytes directly into kernel socket buffers."
                        </p>
                    </div>
                    <div class="flex flex-wrap items-center gap-4 pt-4 font-mono text-[13.5px]">
                        <a href="#pipeline" class="btn btn-oxide px-6 py-3">
                            <span>"Explore Execution Pipeline"</span>
                            {render_icon("arrow-right", IconSize::Sm, Some("w-4 h-4 ml-2"), None)}
                        </a>
                        <a href="#crates" class="btn btn-line px-6 py-3">
                            {render_icon("layers", IconSize::Sm, Some("w-4 h-4 mr-2 text-oxide"), None)}
                            <span>"Crates Breakdown"</span>
                        </a>
                    </div>
                </div>

                // Architectural telemetry spec card with roomy padding
                <div class="lg:col-span-6 fade" style="--d:440ms">
                    <div class="plate bg-sunk shadow-2xl">
                        <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                        <div class="plate-head h-12 px-6">
                            <span class="flex items-center gap-2.5 font-medium tracking-wider text-[12px]">
                                {render_icon("cpu", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                                "ARCHITECTURAL INVARIANTS"
                            </span>
                            <span class="text-dim font-mono text-[11px]">"target: x86_64 / aarch64"</span>
                        </div>
                        <div class="p-8 sm:p-10 grid grid-cols-2 gap-8 sm:gap-10 font-mono">
                            <div class="border-b border-r border-[rgba(236,233,227,0.08)] pb-8 pr-6 sm:pr-8 pl-1 sm:pl-2">
                                <span class="label text-[11px] block pb-2 text-dim">"Template VDOM"</span>
                                <span class="text-[32px] sm:text-[36px] font-bold text-paper block">"0 B"</span>
                                <span class="text-[12px] text-dim block pt-2 leading-relaxed">"Direct byte buffer writing"</span>
                            </div>
                            <div class="border-b border-[rgba(236,233,227,0.08)] pb-8 pl-4 sm:pl-6">
                                <span class="label text-[11px] block pb-2 text-dim">"Median TTFB"</span>
                                <span class="text-[32px] sm:text-[36px] font-bold text-oxide block">"0.28 ms"</span>
                                <span class="text-[12px] text-dim block pt-2 leading-relaxed">"Microsecond socket turnaround"</span>
                            </div>
                            <div class="border-r border-[rgba(236,233,227,0.08)] pt-6 pr-6 sm:pr-8 pl-1 sm:pl-2">
                                <span class="label text-[11px] block pb-2 text-dim">"Path Resolution"</span>
                                <span class="text-[32px] sm:text-[36px] font-bold text-paper block">"~40 ns"</span>
                                <span class="text-[12px] text-dim block pt-2 leading-relaxed">"Matchit Radix O(k) tree"</span>
                            </div>
                            <div class="pt-6 pl-4 sm:pl-6">
                                <span class="label text-[11px] block pb-2 text-dim">"Idle Footprint"</span>
                                <span class="text-[32px] sm:text-[36px] font-bold text-paper block">"8.2 MB"</span>
                                <span class="text-[12px] text-dim block pt-2 leading-relaxed">"No Node.js or JVM runtime"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
