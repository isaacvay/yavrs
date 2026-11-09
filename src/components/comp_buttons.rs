//! 5.1 Buttons & Actions Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentButtons() -> Node {
    rsx! {
        <section id="buttons" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.1"</span>
                <span class="t">"Buttons & Actions"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add button"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"Tactile engineering actions."</h2>
                <button
                    onclick="copyText(this, 'yav add button')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add button"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Precision button primitives calibrated for high-density engineering consoles, command bars, and telemetry interfaces. Supports solid primary, oxide accent, line boundary, ghost, button groups, embedded vector icons, and loading states."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        "LIVE PREVIEW — BUTTON VARIANTS"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"hover / tap / focus / active"</span>
                </div>
                <div class="p-8 space-y-8 bg-ink-2">
                    // Row 1: Primary Variants
                    <div class="space-y-3">
                        <div class="text-[11px] font-mono text-dim uppercase tracking-wider">"Core Variants"</div>
                        <div class="flex flex-wrap items-center gap-4">
                            <button class="btn btn-solid">
                                "Solid Primary"
                                <span class="arr">"→"</span>
                            </button>
                            <button class="btn btn-oxide">
                                "Oxide Accent"
                                <span class="arr">"→"</span>
                            </button>
                            <button class="btn btn-line">
                                "Line Boundary"
                            </button>
                            <button class="btn btn-ghost">
                                "Ghost Action"
                            </button>
                            <button class="btn btn-sm btn-solid">
                                "Compact Sm"
                            </button>
                            <button class="btn btn-sm border border-red-500/40 text-red-400 hover:bg-red-950/20">
                                "Destructive"
                            </button>
                        </div>
                    </div>

                    // Row 2: Button Groups, Icons, and Loading States
                    <div class="pt-6 border-t border-[rgba(236,233,227,0.08)] space-y-3">
                        <div class="text-[11px] font-mono text-dim uppercase tracking-wider">"Groups, Icons & States"</div>
                        <div class="flex flex-wrap items-center gap-6">
                            // Segmented Button Group
                            <div class="btn-group">
                                <button class="btn btn-sm btn-line">"Execute"</button>
                                <button class="btn btn-sm btn-line">"Dry Run"</button>
                                <button class="btn btn-sm btn-line text-mute">"Cancel"</button>
                            </div>

                            // Icon Action Button
                            <button class="btn btn-sm btn-line flex items-center gap-2">
                                {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span>"Open Console"</span>
                            </button>

                            // Loading state button
                            <button class="btn btn-sm btn-line flex items-center gap-2 opacity-80 cursor-wait">
                                <span class="w-3.5 h-3.5 border-2 border-oxide border-t-transparent rounded-full animate-spin"></span>
                                <span>"Compiling..."</span>
                            </button>

                            // Copy Command button
                            <button class="btn btn-sm btn-line flex items-center gap-2 hover:border-oxide/40" onclick="copyText(this, 'cargo build --release')">
                                {render_icon("copy", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-copy-label="" class="text-oxide font-medium">"Copy Command"</span>
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
                                "src/components/buttons.rs"
                            </div>
                        </div>
                        <button data-copy="<button class=&quot;btn btn-solid&quot;>Primary</button>\n<button class=&quot;btn btn-oxide&quot;>Oxide</button>\n<div class=&quot;btn-group&quot;><button class=&quot;btn btn-sm btn-line&quot;>Execute</button></div>" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// 1. Solid Primary Button"</div>
                            <div>
                                <span class="token-tag">"<button "</span>
                                <span class="token-attr">"class="</span>
                                <span class="token-string">"\"btn btn-solid\""</span>
                                <span class="token-tag">">"</span>
                                <span>"\"Solid Primary\""</span>
                                <span class="token-tag">"</button>"</span>
                            </div>
                            <div class="token-comment pt-1">"// 2. Oxide Accent Action"</div>
                            <div>
                                <span class="token-tag">"<button "</span>
                                <span class="token-attr">"class="</span>
                                <span class="token-string">"\"btn btn-oxide\""</span>
                                <span class="token-tag">">"</span>
                                <span>"\"Deploy Target\""</span>
                                <span class="token-tag">"</button>"</span>
                            </div>
                            <div class="token-comment pt-1">"// 3. Segmented Button Group"</div>
                            <div><span class="token-tag">"<div"</span>" "<span class="token-attr">"class="</span><span class="token-string">"\"btn-group\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<button"</span>" "<span class="token-attr">"class="</span><span class="token-string">"\"btn btn-sm btn-line\""</span><span class="token-tag">">"</span><span>"\"Execute\""</span><span class="token-tag">"</button>"</span></div>
                            <div><span class="token-tag">"</div>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"Zero JS runtime · Native HTML button primitives"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"W3C Accessible"</span>
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
                            <span class="text-faint text-[11px] ml-2">"bash — yav-cli scaffold"</span>
                        </div>
                        <button onclick="copyText(this, 'yav add button')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add button"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/button.rs [1.2 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
