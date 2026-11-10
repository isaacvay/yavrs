//! 5.2 Status Chips & Telemetry Badges Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentBadges() -> Node {
    rsx! {
        <section id="badges" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.2"</span>
                <span class="t" data-i18n="comp.badge.badge">"Status Chips & Badges"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add badge"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.badge.title">"Live telemetry chips."</h2>
                <button
                    onclick="copyText(this, 'yav add badge')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add badge"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.badge.lede">
                "Compact indicators designed for build statuses, versioning flags, runtime metrics, and server health monitors. Emits zero runtime JavaScript and renders as lightweight semantic inline markup."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-ok"></span>
                        <span data-i18n="comp.badge.preview_head">"LIVE TELEMETRY CHIPS"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.badge.preview_sub">"live telemetry state"</span>
                </div>
                <div class="p-8 space-y-6 bg-ink-2">
                    // Live Chips Grid
                    <div class="flex flex-wrap items-center gap-3">
                        <span class="chip">
                            <span class="dot dot-ok pulse-dot"></span>
                            <span data-i18n="comp.badge.c1">"System nominal · 0.28ms"</span>
                        </span>
                        <span class="chip">
                            <span class="dot pulse-dot text-oxide"></span>
                            <span data-i18n="comp.badge.c2">"Hot reload :35729"</span>
                        </span>
                        <span class="chip font-mono">
                            "v0.1.5-release"
                        </span>
                        <span class="chip font-mono text-dim" data-i18n="comp.badge.c4">
                            "0 KB hydration"
                        </span>
                        <span class="chip font-mono">
                            <span class="text-oxide">"Axum 0.8"</span>
                        </span>
                        <span class="chip font-mono border-dashed border-[rgba(236,233,227,0.2)]">
                            "Brotli 11"
                        </span>
                        <span class="chip font-mono text-paper">
                            <span class="text-ok">"18 MB RSS"</span>
                        </span>
                        <span class="chip font-mono text-faint">
                            "blake3:9e4b1a"
                        </span>
                    </div>

                    // Interactive Filter Chips Demo
                    <div class="pt-4 border-t border-[rgba(236,233,227,0.08)] space-y-2">
                        <div class="text-[11px] font-mono text-dim uppercase tracking-wider" data-i18n="comp.badge.filter_lbl">"Interactive Filter Badges"</div>
                        <div class="flex flex-wrap items-center gap-2" id="demo-filter-chips">
                            <button class="chip bg-sunk border-oxide text-paper font-semibold cursor-pointer" onclick="this.classList.toggle('border-oxide'); this.classList.toggle('text-paper'); this.classList.toggle('text-dim');">
                                <span class="dot dot-ok"></span>
                                <span data-i18n="comp.badge.f1">"production (active)"</span>
                            </button>
                            <button class="chip bg-sunk border-rule text-dim hover:text-paper cursor-pointer transition-colors" onclick="this.classList.toggle('border-oxide'); this.classList.toggle('text-paper'); this.classList.toggle('text-dim');">
                                "canary"
                            </button>
                            <button class="chip bg-sunk border-rule text-dim hover:text-paper cursor-pointer transition-colors" onclick="this.classList.toggle('border-oxide'); this.classList.toggle('text-paper'); this.classList.toggle('text-dim');">
                                "arm64"
                            </button>
                            <button class="chip bg-sunk border-rule text-dim hover:text-paper cursor-pointer transition-colors" onclick="this.classList.toggle('border-oxide'); this.classList.toggle('text-paper'); this.classList.toggle('text-dim');">
                                "lto:fat"
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            // Code Preview Windows: Tabbed IDE + Terminal
            <div class="space-y-3 pt-2">
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/badges.rs"
                            </div>
                        </div>
                        <button onclick="copyText(this, '<span class=\"chip\"><span class=\"dot dot-ok pulse-dot\"></span>System nominal</span>')" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Telemetry Chip with Pulsing Status Indicator"</div>
                            <div>
                                <span class="token-tag">"<span "</span>
                                <span class="token-attr">"class="</span>
                                <span class="token-string">"\"chip\""</span>
                                <span class="token-tag">">"</span>
                            </div>
                            <div>
                                "    "<span class="token-tag">"<span "</span>
                                <span class="token-attr">"class="</span>
                                <span class="token-string">"\"dot dot-ok pulse-dot\""</span>
                                <span class="token-tag">"></span>"</span>
                            </div>
                            <div>
                                "    "<span class="token-string">"\"System nominal · 0.28ms\""</span>
                            </div>
                            <div>
                                <span class="token-tag">"</span>"</span>
                            </div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="comp.badge.ide_l">"Pure CSS pulse keyframe · 0 KB hydration"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="comp.badge.ide_r">"Optimal Contrast"</span></span>
                    </div>
                </div>

                // Terminal Scaffolding Command
                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-1.5">
                                <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                            </div>
                            <span class="text-faint text-[11px] ml-2" data-i18n="comp.common.term_head">"bash — yav-cli scaffold"</span>
                        </div>
                        <button onclick="copyText(this, 'yav add badge')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_cmd" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add badge"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/badge.rs [0.8 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
