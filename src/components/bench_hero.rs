//! Benchmark Page Header, Breadcrumbs & KPI Metric Cards.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn BenchHero() -> Node {
    rsx! {
        <div>
            // Sub-nav Breadcrumb Bar
            <div class="border-b border-rule bg-ink/90 backdrop-blur-md sticky top-14 z-20">
                <div class="wrap py-3 flex flex-wrap items-center justify-between gap-4 text-xs font-mono">
                    <div class="flex items-center gap-2 text-dim">
                        <a href="/" class="hover:text-paper transition-colors flex items-center gap-1.5 text-mute">
                            {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            <span>"yav"</span>
                        </a>
                        <span class="text-faint">"/"</span>
                        <span class="text-paper font-semibold" data-i18n="bench.crumb.main">"benchmarks"</span>
                        <span class="text-faint">"/"</span>
                        <span class="text-oxide" data-i18n="bench.crumb.sub">"verified-metrics"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="flex items-center gap-2 px-2.5 py-1 rounded bg-sunk border border-rule text-dim text-[11px]">
                            <span class="dot dot-ok"></span>
                            <span class="text-paper font-semibold">"wrk 4.2.0 · k6"</span>
                            <span class="text-dim" data-i18n="bench.hero.hw_pill">"· 8 vCPU Bare Metal"</span>
                        </div>
                        <a href="#reproduce" class="hidden sm:inline-flex items-center gap-1.5 text-mute hover:text-paper transition-colors text-[11.5px]">
                            {render_icon("refresh-cw", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-i18n="bench.hero.repro_link">"Reproduce results"</span>
                        </a>
                    </div>
                </div>
            </div>

            // Page Header Hero Section
            <section class="wrap pt-12 sm:pt-16">
                <div class="fade flex items-center gap-6 label pb-6">
                    <span class="label-oxide">"04"</span>
                    <span data-i18n="bench.hero.kicker">"Independent Benchmarks · Hardware Verified"</span>
                </div>
                <h1 class="h-page text-paper">
                    <span class="line-mask"><span style="--d:80ms" data-i18n="bench.hero.title.1">"High throughput."</span></span>
                    <span class="line-mask"><span style="--d:180ms" class="serif text-oxide" data-i18n="bench.hero.title.2">"Microsecond latency."</span></span>
                </h1>
                <p class="lede max-w-3xl pt-8 fade" style="--d:350ms" data-i18n="bench.hero.lede">
                    "Rigorous, reproducible performance measurements against popular full-stack and API frameworks under identical concurrent load on dedicated multi-core hardware. Zero cherry-picking, zero synthetic bias."
                </p>

                // Hardware environment badges
                <div class="pt-8 flex flex-wrap items-center gap-2.5 font-mono text-[11.5px] text-mute">
                    <span class="px-3 py-1.5 plate bg-ink-2 flex items-center gap-2 border border-rule rounded">
                        {render_icon("cpu", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        <span class="text-paper font-medium">"AMD EPYC™ 9654"</span>
                        <span class="text-dim" data-i18n="bench.hw.cpu">"· 8 Dedicated vCPUs"</span>
                    </span>
                    <span class="px-3 py-1.5 plate bg-ink-2 flex items-center gap-2 border border-rule rounded">
                        {render_icon("database", IconSize::Sm, Some("w-3.5 h-3.5 text-ok"), None)}
                        <span class="text-paper font-medium">"16 GB DDR5 ECC"</span>
                        <span class="text-dim">"· 4800 MT/s"</span>
                    </span>
                    <span class="px-3 py-1.5 plate bg-ink-2 flex items-center gap-2 border border-rule rounded">
                        {render_icon("server", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        <span class="text-paper font-medium">"Linux Kernel 6.8"</span>
                        <span class="text-dim">"· TCP BBR · epoll"</span>
                    </span>
                    <span class="px-3 py-1.5 plate bg-ink-2 flex items-center gap-2 border border-rule rounded">
                        {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5 text-[#eab308]"), None)}
                        <span class="text-paper font-medium">"wrk 4.2.0"</span>
                        <span class="text-dim" data-i18n="bench.hw.conns">"· 200–1000 conns"</span>
                    </span>
                </div>
            </section>

            // Headline KPI Stat Grid (4-column responsive)
            <section class="mt-12 sm:mt-16 border-y border-rule">
                <div class="wrap grid grid-cols-2 lg:grid-cols-4" data-stagger="">
                    // Stat 1: Throughput
                    <div class="py-10 pr-6 border-r border-b lg:border-b-0 border-rule">
                        <div class="label pb-4 flex items-center justify-between">
                            <span data-i18n="bench.kpi1.lbl">"Peak Throughput"</span>
                            <span class="text-ok text-[11px] font-mono" data-i18n="bench.kpi1.badge">"14× vs Next.js"</span>
                        </div>
                        <div class="font-mono text-[clamp(36px,4.5vw,60px)] leading-none tracking-[-0.05em] text-paper">
                            <span data-count="124500">"124,500"</span>
                        </div>
                        <div class="text-[12px] font-mono text-dim pt-2" data-i18n="bench.kpi1.sub">"req/s · 200 persistent conns"</div>
                    </div>

                    // Stat 2: Median TTFB
                    <div class="py-10 px-6 border-b lg:border-b-0 lg:border-r border-rule">
                        <div class="label pb-4 flex items-center justify-between">
                            <span data-i18n="bench.kpi2.lbl">"Median TTFB"</span>
                            <span class="text-oxide text-[11px] font-mono" data-i18n="bench.kpi2.badge">"280 microseconds"</span>
                        </div>
                        <div class="font-mono text-[clamp(36px,4.5vw,60px)] leading-none tracking-[-0.05em] text-oxide">
                            <span data-count="0.28">"0.28"</span><span class="text-[20px] font-normal text-dim ml-1">"ms"</span>
                        </div>
                        <div class="text-[12px] font-mono text-dim pt-2" data-i18n="bench.kpi2.sub">"p50 latency to first byte"</div>
                    </div>

                    // Stat 3: 99th Percentile Latency
                    <div class="py-10 pr-6 lg:px-6 border-r border-rule">
                        <div class="label pb-4 flex items-center justify-between">
                            <span data-i18n="bench.kpi3.lbl">"p99 Tail Latency"</span>
                            <span class="text-ok text-[11px] font-mono" data-i18n="bench.kpi3.badge">"jitter-free"</span>
                        </div>
                        <div class="font-mono text-[clamp(36px,4.5vw,60px)] leading-none tracking-[-0.05em] text-paper">
                            <span data-count="1.12">"1.12"</span><span class="text-[20px] font-normal text-dim ml-1">"ms"</span>
                        </div>
                        <div class="text-[12px] font-mono text-dim pt-2" data-i18n="bench.kpi3.sub">"99% of requests < 1.12ms"</div>
                    </div>

                    // Stat 4: Idle RSS Memory
                    <div class="py-10 pl-6">
                        <div class="label pb-4 flex items-center justify-between">
                            <span data-i18n="bench.kpi4.lbl">"Idle Memory Footprint"</span>
                            <span class="text-ok text-[11px] font-mono" data-i18n="bench.kpi4.badge">"51× lighter"</span>
                        </div>
                        <div class="font-mono text-[clamp(36px,4.5vw,60px)] leading-none tracking-[-0.05em] text-paper">
                            <span data-count="8.2">"8.2"</span><span class="text-[20px] font-normal text-dim ml-1">"MB"</span>
                        </div>
                        <div class="text-[12px] font-mono text-dim pt-2" data-i18n="bench.kpi4.sub">"24 MB max under 1,000 conns"</div>
                    </div>
                </div>
            </section>
        </div>
    }
}
