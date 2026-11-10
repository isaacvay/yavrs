//! 5.8 Navigation & Tabs Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentNavigation() -> Node {
    rsx! {
        <section id="navigation" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.8"</span>
                <span class="t" data-i18n="comp.nav.sec">"Navigation & Tabs"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add tabs"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.nav.title">"Engineered navigation."</h2>
                <button
                    onclick="copyText(this, 'yav add tabs')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add tabs"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.nav.lede">
                "Segmented tabs, breadcrumb trails, and pagination controls engineered with exact pixel alignment, active hairline indicators, and zero client hydration overhead."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.nav.preview_head">"NAVIGATION PRIMITIVES"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.nav.preview_sub">"tabs & breadcrumbs"</span>
                </div>
                <div class="p-8 space-y-8 bg-ink-2">
                    // Breadcrumb trail
                    <div class="space-y-2">
                        <div class="text-[11px] font-mono text-dim uppercase tracking-wider" data-i18n="comp.nav.bread_lbl">"Breadcrumb Trail"</div>
                        <div class="flex items-center gap-2 font-mono text-[12.5px] text-dim">
                            <a href="/" class="hover:text-paper transition-colors">"src"</a>
                            <span class="text-faint">"/"</span>
                            <a href="/docs" class="hover:text-paper transition-colors">"routes"</a>
                            <span class="text-faint">"/"</span>
                            <a href="/docs" class="hover:text-paper transition-colors">"api"</a>
                            <span class="text-faint">"/"</span>
                            <span class="text-oxide">"users.rs"</span>
                        </div>
                    </div>

                    // Tab Navigation Preview with interactive state
                    <div class="space-y-2">
                        <div class="text-[11px] font-mono text-dim uppercase tracking-wider" data-i18n="comp.nav.tab_lbl">"Segmented Tab Control"</div>
                        <div class="flex items-center border border-[rgba(236,233,227,0.08)] bg-sunk overflow-x-auto" id="demo-tabs-bar">
                            <button class="tab-btn active" data-i18n="comp.nav.t1" onclick="this.parentElement.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active')); this.classList.add('active');">"Overview"</button>
                            <button class="tab-btn" data-i18n="comp.nav.t2" onclick="this.parentElement.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active')); this.classList.add('active');">"Performance"</button>
                            <button class="tab-btn" data-i18n="comp.nav.t3" onclick="this.parentElement.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active')); this.classList.add('active');">"Security Log"</button>
                            <button class="tab-btn" data-i18n="comp.nav.t4" onclick="this.parentElement.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active')); this.classList.add('active');">"Settings"</button>
                        </div>
                    </div>

                    // Pagination Bar with interactive active indicator
                    <div class="space-y-2">
                        <div class="text-[11px] font-mono text-dim uppercase tracking-wider" data-i18n="comp.nav.page_lbl">"Pagination Rail"</div>
                        <div class="flex items-center justify-between p-3 border border-[rgba(236,233,227,0.08)] bg-sunk font-mono text-[12px]">
                            <button class="text-dim hover:text-paper transition-colors" data-i18n="comp.nav.prev">"← Prev"</button>
                            <div class="flex items-center gap-2" id="demo-pagination-rail">
                                <span class="px-2 py-0.5 bg-ink border border-oxide text-paper font-semibold">"01"</span>
                                <span class="px-2 py-0.5 text-dim hover:text-paper cursor-pointer">"02"</span>
                                <span class="px-2 py-0.5 text-dim hover:text-paper cursor-pointer">"03"</span>
                                <span class="text-dim">"..."</span>
                                <span class="px-2 py-0.5 text-dim hover:text-paper cursor-pointer">"12"</span>
                            </div>
                            <button class="text-paper hover:text-oxide transition-colors" data-i18n="comp.nav.next">"Next →"</button>
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
                                "src/components/navigation.rs"
                            </div>
                        </div>
                        <button onclick="copyText(this, '<div class=\"tab-bar\"><button class=\"tab-btn active\">Overview</button></div>')" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Segmented Tabs control with active state"</div>
                            <div><span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"flex items-center border border-rule bg-sunk\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<button "</span><span class="token-attr">"class="</span><span class="token-string">"\"tab-btn active\""</span><span class="token-tag">">"</span>"\"Overview\""<span class="token-tag">"</button>"</span></div>
                            <div>"    "<span class="token-tag">"<button "</span><span class="token-attr">"class="</span><span class="token-string">"\"tab-btn\""</span><span class="token-tag">">"</span>"\"Performance\""<span class="token-tag">"</button>"</span></div>
                            <div>"    "<span class="token-tag">"<button "</span><span class="token-attr">"class="</span><span class="token-string">"\"tab-btn\""</span><span class="token-tag">">"</span>"\"Settings\""<span class="token-tag">"</button>"</span></div>
                            <div><span class="token-tag">"</div>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="comp.nav.ide_l">"Strict keyboard focus ring · Pure CSS transition"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="comp.nav.ide_r">"W3C Tablist"</span></span>
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
                        <button onclick="copyText(this, 'yav add tabs')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_cmd" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add tabs"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/tabs.rs [1.4 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
