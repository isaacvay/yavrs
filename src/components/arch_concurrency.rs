//! Concurrency Model & Multi-Core Scaling Component (Airy & Generous).

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ArchConcurrency() -> Node {
    rsx! {
        <section id="concurrency" class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"2.5"</span>
                <span class="t" data-i18n="arch.conc.badge">"Concurrency Model"</span>
                <span class="sec-rule"></span>
                <span class="r" data-i18n="arch.conc.meta">"Tokio work-stealing"</span>
            </div>

            <div class="pt-6 grid lg:grid-cols-12 gap-10 lg:gap-12 items-start pb-6">
                // Left Column: 6 cols for generous heading space & matching other sections
                <div class="lg:col-span-6 space-y-7 reveal">
                    <div class="space-y-4 pl-5 sm:pl-6 border-l-2 border-oxide/50">
                        <h2 class="h-section text-paper leading-[1.18]">
                            <span data-i18n="arch.conc.title.lead">"8 cores, 124,500 req/s. "</span>
                            <span class="serif text-oxide" data-i18n="arch.conc.title.accent">"Zero lock contention."</span>
                        </h2>
                        <p class="text-[15.5px] text-mute leading-[1.8]" data-i18n="arch.conc.p1">
                            "In multi-threaded web applications, performance bottlenecks rarely come from raw computation. They stem from thread synchronization: global mutexes, lock contention on session stores, and cache ping-pong across CPU cores."
                        </p>
                    </div>

                    <p class="text-[15.5px] text-mute leading-[1.8] pl-5 sm:pl-6" data-i18n="arch.conc.p2">
                        "Yav's architecture eliminates shared mutable state on the request path entirely. Route trees are immutable. Sessions are cryptographic cookies. Telemetry relies strictly on lock-free hardware atomics."
                    </p>
                </div>

                // Right Column: 6 cols for symmetric dual-pane plate
                <div class="lg:col-span-6 reveal" style="--d:140ms">
                    <div class="plate bg-sunk shadow-2xl">
                        <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                        <div class="plate-head h-12 px-6">
                            <span class="flex items-center gap-2.5 font-medium tracking-wide">
                                {render_icon("cpu", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                                <span class="text-[12px]" data-i18n="arch.conc.plate_title">"TOKIO WORK-STEALING THREAD ARCHITECTURE"</span>
                            </span>
                            <span class="text-dim font-mono text-[11px]" data-i18n="arch.conc.plate_sub">"8 Hardware Workers"</span>
                        </div>

                        <div class="p-6 sm:p-7 space-y-5 font-mono text-[12px]">
                            <div class="grid grid-cols-2 gap-3 sm:gap-3.5">
                                <div class="p-4 border border-[rgba(236,233,227,0.08)] bg-ink-2/50 text-center space-y-1 shadow-sm">
                                    <span class="text-dim text-[11px] block" data-i18n="arch.conc.w0">"Worker 0"</span>
                                    <span class="text-oxide font-bold text-[15px] block">"15,560 req/s"</span>
                                    <span class="text-[10px] text-dim block" data-i18n="arch.conc.c0">"core #0 (epoll)"</span>
                                </div>
                                <div class="p-4 border border-[rgba(236,233,227,0.08)] bg-ink-2/50 text-center space-y-1 shadow-sm">
                                    <span class="text-dim text-[11px] block" data-i18n="arch.conc.w1">"Worker 1"</span>
                                    <span class="text-oxide font-bold text-[15px] block">"15,570 req/s"</span>
                                    <span class="text-[10px] text-dim block" data-i18n="arch.conc.c1">"core #1 (epoll)"</span>
                                </div>
                                <div class="p-4 border border-[rgba(236,233,227,0.08)] bg-ink-2/50 text-center space-y-1 shadow-sm">
                                    <span class="text-dim text-[11px] block" data-i18n="arch.conc.w2">"Worker 2"</span>
                                    <span class="text-oxide font-bold text-[15px] block">"15,540 req/s"</span>
                                    <span class="text-[10px] text-dim block" data-i18n="arch.conc.c2">"core #2 (epoll)"</span>
                                </div>
                                <div class="p-4 border border-[rgba(236,233,227,0.08)] bg-ink-2/50 text-center space-y-1 shadow-sm">
                                    <span class="text-dim text-[11px] block" data-i18n="arch.conc.w3">"Worker 3"</span>
                                    <span class="text-oxide font-bold text-[15px] block">"15,590 req/s"</span>
                                    <span class="text-[10px] text-dim block" data-i18n="arch.conc.c3">"core #3 (epoll)"</span>
                                </div>
                            </div>

                            <div class="p-4 border border-[rgba(236,233,227,0.08)] bg-[rgba(236,233,227,0.02)] space-y-2.5">
                                <div class="flex flex-wrap items-center justify-between text-dim gap-2">
                                    <span data-i18n="arch.conc.strat_lbl">"Task Queue Balancing Strategy"</span>
                                    <span class="text-paper font-semibold" data-i18n="arch.conc.strat_val">"LIFO worker slots + Cross-thread stealing"</span>
                                </div>
                                <div class="flex flex-wrap items-center justify-between text-dim gap-2 pt-1 border-t border-[rgba(236,233,227,0.06)]">
                                    <span data-i18n="arch.conc.sync_lbl">"Synchronization Overhead"</span>
                                    <span class="text-oxide font-bold" data-i18n="arch.conc.sync_val">"0 mutex acquisitions on GET routes"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // Section 2.6 — Design Manifesto
        <section id="manifesto" class="wrap pt-12 sm:pt-16 pb-16 sm:pb-24">
            <div class="sec-head reveal">
                <span class="n">"2.6"</span>
                <span class="t" data-i18n="arch.man.badge">"Design Manifesto"</span>
                <span class="sec-rule"></span>
                <span class="r" data-i18n="arch.man.meta">"compile-time specialization"</span>
            </div>

            <div class="pt-6 max-w-4xl reveal pl-6 sm:pl-10 lg:pl-12 border-l-2 border-oxide">
                <blockquote class="space-y-6">
                    <p class="text-[26px] sm:text-[34px] lg:text-[42px] leading-[1.3] text-paper font-normal tracking-[-0.03em]">
                        <span data-i18n="arch.man.quote.a">"The fastest code is the code that never runs. Yav moves template work to "</span>
                        <span class="serif text-oxide" data-i18n="arch.man.quote.b">"compile time"</span>
                        <span data-i18n="arch.man.quote.c">" and leaves the request path with nothing left to decide."</span>
                    </p>
                    <footer class="pt-4 flex flex-wrap items-center gap-4 text-dim font-mono text-[12.5px]">
                        <span class="text-oxide font-semibold" data-i18n="arch.man.attr">"— Architectural Invariant"</span>
                        <span class="text-dim/50">"•"</span>
                        <span data-i18n="arch.man.f1">"Zero VDOM tree allocation"</span>
                        <span class="text-dim/50">"•"</span>
                        <span data-i18n="arch.man.f2">"Zero JS runtime bridging"</span>
                        <span class="text-dim/50">"•"</span>
                        <span data-i18n="arch.man.f3">"Deterministic machine code"</span>
                    </footer>
                </blockquote>
            </div>
        </section>
    }
}
