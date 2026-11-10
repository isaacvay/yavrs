//! 5.11 Spring Motion Dynamics Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentMotion() -> Node {
    rsx! {
        <section id="motion" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.11"</span>
                <span class="t" data-i18n="comp.motion.sec">"Spring Physics"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add motion"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.motion.title">"Mechanical spring dynamics."</h2>
                <button
                    onclick="copyText(this, 'yav add motion')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add motion"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.motion.lede">
                "Micro-interactions governed by harmonic oscillator physics (stiffness, damping, mass) instead of arbitrary cubic-bezier timing curves. Driven by Web Animations API in less than 2 KB runtime."
            </p>

            // Live Interactive Physics Cards
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                // Spring 1: Snappy Recoil
                <div
                    class="plate p-8 space-y-6 cursor-pointer select-none bg-ink-2 hover:bg-sunk transition-colors"
                    data-motion="true"
                    data-motion-hover="{\"scale\":1.02,\"y\":-4}"
                    data-motion-tap="{\"scale\":0.97}"
                    data-motion-transition="{\"is_spring\":true,\"stiffness\":320,\"damping\":18}"
                >
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="font-mono text-[11px] text-oxide" data-i18n="comp.motion.c1_meta">"STIFFNESS: 320 · DAMPING: 18"</span>
                        <span class="dot dot-ok"></span>
                    </div>
                    <h3 class="text-[22px] text-paper" data-i18n="comp.motion.c1_title">"Snappy Recoil"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="comp.motion.c1_desc">
                        "Click or hover over this card to trigger rapid spring acceleration with zero perceptual settling delay."
                    </p>
                    <div class="font-mono text-[11px] text-dim pt-2 border-t border-[rgba(236,233,227,0.08)] flex justify-between">
                        <span data-i18n="comp.motion.c1_l">"Hover scale 1.02 / y -4px"</span>
                        <span class="text-oxide">"while_tap 0.97"</span>
                    </div>
                </div>

                // Spring 2: Tactile Wobble
                <div
                    class="plate p-8 space-y-6 cursor-pointer select-none bg-ink-2 hover:bg-sunk transition-colors"
                    data-motion="true"
                    data-motion-hover="{\"scale\":1.02,\"rotate\":1}"
                    data-motion-tap="{\"scale\":0.96,\"rotate\":-1}"
                    data-motion-transition="{\"is_spring\":true,\"stiffness\":240,\"damping\":15}"
                >
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="font-mono text-[11px] text-oxide" data-i18n="comp.motion.c2_meta">"STIFFNESS: 240 · DAMPING: 15"</span>
                        <span class="dot"></span>
                    </div>
                    <h3 class="text-[22px] text-paper" data-i18n="comp.motion.c2_title">"Tactile Wobble"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="comp.motion.c2_desc">
                        "Subtle physical angular momentum response on click for satisfying tactile control feedback."
                    </p>
                    <div class="font-mono text-[11px] text-dim pt-2 border-t border-[rgba(236,233,227,0.08)] flex justify-between">
                        <span data-i18n="comp.motion.c2_l">"Angular rotate +1°"</span>
                        <span class="text-oxide">"while_tap -1°"</span>
                    </div>
                </div>

                // Spring 3: Heavy Inertia
                <div
                    class="plate p-8 space-y-6 cursor-pointer select-none bg-ink-2 hover:bg-sunk transition-colors"
                    data-motion="true"
                    data-motion-hover="{\"scale\":1.015,\"y\":-2}"
                    data-motion-tap="{\"scale\":0.98}"
                    data-motion-transition="{\"is_spring\":true,\"stiffness\":160,\"damping\":26}"
                >
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="font-mono text-[11px] text-oxide" data-i18n="comp.motion.c3_meta">"STIFFNESS: 160 · DAMPING: 26"</span>
                        <span class="dot text-dim"></span>
                    </div>
                    <h3 class="text-[22px] text-paper" data-i18n="comp.motion.c3_title">"Heavy Inertia"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="comp.motion.c3_desc">
                        "High critical damping model mimicking dense materials, ideal for structural modal panels and sliding drawers."
                    </p>
                    <div class="font-mono text-[11px] text-dim pt-2 border-t border-[rgba(236,233,227,0.08)] flex justify-between">
                        <span data-i18n="comp.motion.c3_l">"High mass resistance"</span>
                        <span class="text-paper" data-i18n="comp.motion.c3_r">"Smooth return"</span>
                    </div>
                </div>

                // Spring 4: Elastic Bounce
                <div
                    class="plate p-8 space-y-6 cursor-pointer select-none bg-ink-2 hover:bg-sunk transition-colors"
                    data-motion="true"
                    data-motion-hover="{\"scale\":1.03,\"y\":-5}"
                    data-motion-tap="{\"scale\":0.95}"
                    data-motion-transition="{\"is_spring\":true,\"stiffness\":400,\"damping\":12}"
                >
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="font-mono text-[11px] text-oxide" data-i18n="comp.motion.c4_meta">"STIFFNESS: 400 · DAMPING: 12"</span>
                        <span class="dot dot-ok"></span>
                    </div>
                    <h3 class="text-[22px] text-paper" data-i18n="comp.motion.c4_title">"Elastic Bounce"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="comp.motion.c4_desc">
                        "Under-damped spring oscillator that oscillates smoothly around resting equilibrium before settling."
                    </p>
                    <div class="font-mono text-[11px] text-dim pt-2 border-t border-[rgba(236,233,227,0.08)] flex justify-between">
                        <span data-i18n="comp.motion.c4_l">"Low damping ratio (0.3)"</span>
                        <span class="text-oxide" data-i18n="comp.motion.c4_r">"Spring oscillation"</span>
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
                                "src/components/motion_card.rs"
                            </div>
                        </div>
                        <button data-copy="<div data-motion=&quot;true&quot; data-motion-hover=&quot;{'y':-4}&quot; data-motion-transition=&quot;{'is_spring':true,'stiffness':320,'damping':18}&quot;>Card</div>" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Declarative Web Animations spring motion attrs"</div>
                            <div><span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"plate p-8\""</span></div>
                            <div>"    "<span class="token-attr">"data-motion="</span><span class="token-string">"\"true\""</span></div>
                            <div>"    "<span class="token-attr">"data-motion-hover="</span><span class="token-string">"\"{\\\"scale\\\":1.02, \\\"y\\\":-4}\""</span></div>
                            <div>"    "<span class="token-attr">"data-motion-tap="</span><span class="token-string">"\"{\\\"scale\\\":0.97}\""</span></div>
                            <div>"    "<span class="token-attr">"data-motion-transition="</span><span class="token-string">"\"{\\\"is_spring\\\":true, \\\"stiffness\\\":320, \\\"damping\\\":18}\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-string">"\"Snappy Recoil\""</span><span class="token-tag">"</div>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="comp.motion.ide_l">"Calculates dynamic analytical spring position on UI thread"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"60–120 FPS"</span>
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
                        <button onclick="copyText(this, 'yav add motion')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_cmd" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add motion"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/motion.rs [1.8 KB pure RSX + WebAnimations]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
