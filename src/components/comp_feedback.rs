//! 5.7 Overlays, Alerts & Modals Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentFeedback() -> Node {
    rsx! {
        <section id="feedback" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.7"</span>
                <span class="t" data-i18n="comp.fb.sec">"Overlays & Feedback"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add alert dialog"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.fb.title">"Alerts & Dialogs."</h2>
                <button
                    onclick="copyText(this, 'yav add alert dialog')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add alert dialog"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.fb.lede">
                "Structured alerts with severity cues, stacked toast telemetry notifications, and accessible modal confirmation dialogs for high-impact infrastructure workflows."
            </p>

            <div class="space-y-6">
                // Alert Banners (OK, Oxide, Caution)
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="alert-box alert-ok">
                        <div class="text-[#9cc69b] pt-0.5">
                            {render_icon("shield", IconSize::Sm, Some("w-5 h-5"), None)}
                        </div>
                        <div class="space-y-1">
                            <div class="text-[13.5px] font-medium text-paper" data-i18n="comp.fb.a1_title">"Memory Safety Verified"</div>
                            <p class="text-[12.5px] text-mute leading-relaxed" data-i18n="comp.fb.a1_desc">
                                "All templates are validated at compile-time by procedural macros. Zero arbitrary script injection risks."
                            </p>
                        </div>
                    </div>

                    <div class="alert-box alert-oxide">
                        <div class="text-oxide pt-0.5">
                            {render_icon("zap", IconSize::Sm, Some("w-5 h-5"), None)}
                        </div>
                        <div class="space-y-1">
                            <div class="text-[13.5px] font-medium text-paper" data-i18n="comp.fb.a2_title">"Release Optimization"</div>
                            <p class="text-[12.5px] text-mute leading-relaxed" data-i18n="comp.fb.a2_desc">
                                "Self-contained binary target. Static assets pre-compressed with Brotli level 11 and Blake3 content digests."
                            </p>
                        </div>
                    </div>
                </div>

                // Simulated Interactive Modal Dialog
                <div class="plate p-6 space-y-6 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between pb-4 border-b border-[rgba(236,233,227,0.08)]">
                        <div class="flex items-center gap-3">
                            <span class="dot text-oxide"></span>
                            <span class="font-mono text-[13px] text-paper" data-i18n="comp.fb.modal_head">"Deploy Target Confirmation"</span>
                        </div>
                        <button class="text-dim hover:text-paper transition-colors font-mono text-[12px]">"ESC [✕]"</button>
                    </div>

                    <div class="space-y-3">
                        <p class="text-[13.5px] text-mute leading-relaxed">
                            <span data-i18n="comp.fb.modal_q1">"Are you sure you want to deploy build "</span>
                            <strong class="text-paper font-mono">"v0.1.5-prod-58b2"</strong>
                            <span data-i18n="comp.fb.modal_q2">" to the production cluster?"</span>
                        </p>
                        <div class="p-3 bg-sunk border border-[rgba(236,233,227,0.06)] font-mono text-[11.5px] text-dim space-y-1">
                            <div data-i18n="comp.fb.cluster">"Cluster: iad (Northern Virginia)"</div>
                            <div data-i18n="comp.fb.instances">"Instances: 3 x shared-cpu-1x (256MB)"</div>
                            <div data-i18n="comp.fb.health">"Health check: /api/health -> 200 OK (0.21ms)"</div>
                        </div>
                    </div>

                    <div class="flex justify-end items-center gap-3 pt-3 border-t border-[rgba(236,233,227,0.08)]">
                        <button class="btn btn-sm btn-ghost" data-i18n="comp.fb.cancel">"Cancel"</button>
                        <button class="btn btn-sm btn-oxide" data-i18n="comp.fb.confirm">"Confirm & Ship"</button>
                    </div>
                </div>

                // Floating Toast Notifications Preview Stack
                <div class="plate p-4 bg-ink-2 space-y-3">
                    <div class="text-[11px] font-mono text-dim uppercase tracking-wider flex items-center justify-between">
                        <span data-i18n="comp.fb.toast_head">"Toast Notification Stack"</span>
                        <span class="text-ok" data-i18n="comp.fb.toast_auto">"auto-dismiss: 4000ms"</span>
                    </div>
                    <div class="grid sm:grid-cols-2 gap-3">
                        <div class="p-3 bg-sunk border border-ok/30 rounded flex items-center justify-between font-mono text-xs shadow-lg">
                            <div class="flex items-center gap-2.5">
                                <span class="dot dot-ok"></span>
                                <span class="text-paper" data-i18n="comp.fb.toast1">"Release binary compiled (0.28s)"</span>
                            </div>
                            <span class="text-dim text-[10px]" data-i18n="comp.fb.now">"now"</span>
                        </div>
                        <div class="p-3 bg-sunk border border-rule rounded flex items-center justify-between font-mono text-xs shadow-lg">
                            <div class="flex items-center gap-2.5">
                                <span class="dot text-oxide"></span>
                                <span class="text-paper" data-i18n="comp.fb.toast2">"Copied snippet to clipboard"</span>
                            </div>
                            <span class="text-dim text-[10px]" data-i18n="comp.fb.ago">"2s ago"</span>
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
                                "src/components/alert.rs"
                            </div>
                        </div>
                        <button onclick="copyText(this, '<div class=\"alert-box alert-ok\"><div class=\"font-medium\">Memory Safe</div></div>')" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Alert Box with semantic severity"</div>
                            <div><span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"alert-box alert-ok\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-ok pt-0.5\""</span><span class="token-tag">">"</span>"{"<span class="token-fn">"render_icon"</span>"("<span class="token-string">"\"shield\""</span>", "<span class="token-type">"IconSize"</span>"::"<span class="token-fn">"Sm"</span>", "<span class="token-type">"None"</span>", "<span class="token-type">"None"</span>")}"<span class="token-tag">"</div>"</span></div>
                            <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"space-y-1\""</span><span class="token-tag">">"</span></div>
                            <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"font-medium text-paper\""</span><span class="token-tag">">"</span>"\"Memory Safety Verified\""<span class="token-tag">"</div>"</span></div>
                            <div>"    "<span class="token-tag">"</div>"</span></div>
                            <div><span class="token-tag">"</div>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="comp.fb.ide_l">"Accessible WAI-ARIA role=\"alert\" and dialog modals"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="comp.fb.ide_r">"Zero Hydration"</span></span>
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
                        <button onclick="copyText(this, 'yav add alert dialog toast')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_cmd" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add alert dialog toast"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/alert.rs, dialog.rs, toast.rs [3.6 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
