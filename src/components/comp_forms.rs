//! 5.6 Form Controls & Range Inputs Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentForms() -> Node {
    rsx! {
        <section id="forms" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.3"</span>
                <span class="t" data-i18n="comp.forms.badge">"Form Controls & Inputs"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add input"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.forms.title">"Precision input ergonomics."</h2>
                <button
                    onclick="copyText(this, 'yav add input')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add input"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.forms.lede">
                "Server-validated inputs, mechanical toggle switches, and precision sliders designed to fit seamlessly into engineering workflows. Styled with hairline borders and tactile focus rings."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.forms.preview_head">"INTERACTIVE CONTROLS"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.forms.preview_sub">"live state binding"</span>
                </div>
                <div class="p-8 space-y-8 bg-ink-2">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        // Text Input with Icon Adornment & Keyboard Shortcut
                        <div class="space-y-2">
                            <label class="label block text-[11px]" data-i18n="comp.forms.filter_lbl">"Route Filter / Full-text Search"</label>
                            <div class="relative flex items-center">
                                <div class="absolute left-3.5 pointer-events-none text-dim">
                                    {render_icon("search", IconSize::Sm, Some("w-4 h-4"), None)}
                                </div>
                                <input
                                    type="text"
                                    class="input-field pl-10 pr-12 font-mono text-[13px]"
                                    placeholder="Filter crates, endpoints or components..."
                                    data-i18n-placeholder="comp.forms.filter_ph"
                                />
                                <div class="absolute right-3 pointer-events-none">
                                    <kbd class="kbd">"⌘K"</kbd>
                                </div>
                            </div>
                            <span class="text-[11px] font-mono text-dim" data-i18n="comp.forms.filter_hint">"Full-text regex filter against compiled Radix routes"</span>
                        </div>

                        // Range Slider with live readout
                        <div class="space-y-2">
                            <div class="flex justify-between items-center">
                                <label class="label text-[11px]" data-i18n="comp.forms.slider_lbl">"Tokio Worker Concurrency"</label>
                                <span id="slider-readout" class="font-mono text-[12px] text-oxide font-bold">"64"<span data-i18n="comp.forms.workers_suffix">" workers"</span></span>
                            </div>
                            <div class="pt-3">
                                <input
                                    id="comp-workers-slider"
                                    type="range"
                                    min="4"
                                    max="128"
                                    value="64"
                                    step="4"
                                    class="range"
                                    oninput="window.updateWorkersReadout(this.value)"
                                />
                            </div>
                            <div class="flex justify-between text-[10px] font-mono text-dim pt-1">
                                <span data-i18n="comp.forms.min_threads">"4 threads"</span>
                                <span data-i18n="comp.forms.def_threads">"Default: 64"</span>
                                <span data-i18n="comp.forms.max_threads">"128 threads"</span>
                            </div>
                        </div>
                    </div>

                    // Switch Toggles & Checkboxes
                    <div class="pt-6 border-t border-[rgba(236,233,227,0.08)] grid grid-cols-1 sm:grid-cols-2 gap-6">
                        <div class="flex items-center justify-between p-4 border border-[rgba(236,233,227,0.08)] bg-sunk">
                            <div>
                                <div class="text-[13.5px] font-medium text-paper" data-i18n="comp.forms.sw1.t">"Brotli Pre-Compression"</div>
                                <div class="text-[11px] font-mono text-dim" data-i18n="comp.forms.sw1.d">"Generate .br assets at compile time (quality: 11)"</div>
                            </div>
                            <label class="switch ml-4 cursor-pointer">
                                <input type="checkbox" checked="" />
                                <span class="switch-slider"></span>
                            </label>
                        </div>

                        <div class="flex items-center justify-between p-4 border border-[rgba(236,233,227,0.08)] bg-sunk">
                            <div>
                                <div class="text-[13.5px] font-medium text-paper" data-i18n="comp.forms.sw2.t">"Strict CSRF Protection"</div>
                                <div class="text-[11px] font-mono text-dim" data-i18n="comp.forms.sw2.d">"Validate HMAC tokens on every mutation"</div>
                            </div>
                            <label class="switch ml-4 cursor-pointer">
                                <input type="checkbox" checked="" />
                                <span class="switch-slider"></span>
                            </label>
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
                                "src/components/forms.rs"
                            </div>
                        </div>
                        <button data-copy="<input type=&quot;text&quot; class=&quot;input-field&quot; placeholder=&quot;Search...&quot; />\n<input type=&quot;range&quot; min=&quot;4&quot; max=&quot;128&quot; class=&quot;range&quot; />" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Input with keyboard adornment and range slider"</div>
                            <div><span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"relative flex items-center\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<input "</span><span class="token-attr">"type="</span><span class="token-string">"\"text\""</span>" "<span class="token-attr">"class="</span><span class="token-string">"\"input-field pl-10 pr-12\""</span>" "<span class="token-attr">"placeholder="</span><span class="token-string">"\"Search...\""</span><span class="token-tag">" />"</span></div>
                            <div>"    "<span class="token-tag">"<kbd "</span><span class="token-attr">"class="</span><span class="token-string">"\"kbd absolute right-3\""</span><span class="token-tag">">"</span>"\"⌘K\""<span class="token-tag">"</kbd>"</span></div>
                            <div><span class="token-tag">"</div>"</span></div>
                            <div class="token-comment pt-1">"// Mechanical Switch Slider"</div>
                            <div><span class="token-tag">"<label "</span><span class="token-attr">"class="</span><span class="token-string">"\"switch\""</span><span class="token-tag">">"</span><span class="token-tag">"<input "</span><span class="token-attr">"type="</span><span class="token-string">"\"checkbox\" "</span><span class="token-attr">"checked"</span><span class="token-tag">" />"</span><span class="token-tag">"<span "</span><span class="token-attr">"class="</span><span class="token-string">"\"switch-slider\""</span><span class="token-tag">"></span>"</span><span class="token-tag">"</label>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="comp.forms.ide_l">"Strict W3C accessible markup · Native keyboard support"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="comp.forms.ide_r">"Zero Layout Shift"</span></span>
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
                        <button onclick="copyText(this, 'yav add input')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_cmd" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add input"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/input.rs [1.8 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
            <script>
                {r#"
                window.updateWorkersReadout = function(val) {
                    var el = document.getElementById('slider-readout');
                    if (!el) return;
                    var suffix = window.yavT ? window.yavT('comp.forms.workers_suffix', ' workers') : ' workers';
                    el.textContent = val + suffix;
                };
                window.addEventListener('yav:langchange', function() {
                    var s = document.getElementById('comp-workers-slider');
                    if (s) window.updateWorkersReadout(s.value);
                });
                "#}
            </script>
        </section>
    }
}
