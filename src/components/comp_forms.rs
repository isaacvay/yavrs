//! 5.6 Form Controls & Range Inputs Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentForms() -> Node {
    rsx! {
        <section id="forms" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.6"</span>
                <span class="t">"Form Controls & Inputs"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add input"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"Precision input ergonomics."</h2>
                <button
                    onclick="copyText(this, 'yav add input')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add input"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Server-validated inputs, mechanical toggle switches, and precision sliders designed to fit seamlessly into engineering workflows. Styled with hairline borders and tactile focus rings."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        "INTERACTIVE CONTROLS"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"live state binding"</span>
                </div>
                <div class="p-8 space-y-8 bg-ink-2">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        // Text Input with Icon Adornment & Keyboard Shortcut
                        <div class="space-y-2">
                            <label class="label block text-[11px]">"Route Filter / Full-text Search"</label>
                            <div class="relative flex items-center">
                                <div class="absolute left-3.5 pointer-events-none text-dim">
                                    {render_icon("search", IconSize::Sm, Some("w-4 h-4"), None)}
                                </div>
                                <input
                                    type="text"
                                    class="input-field pl-10 pr-12 font-mono text-[13px]"
                                    placeholder="Filter crates, endpoints or components..."
                                />
                                <div class="absolute right-3 pointer-events-none">
                                    <kbd class="kbd">"⌘K"</kbd>
                                </div>
                            </div>
                            <span class="text-[11px] font-mono text-dim">"Full-text regex filter against compiled Radix routes"</span>
                        </div>

                        // Range Slider with live readout
                        <div class="space-y-2">
                            <div class="flex justify-between items-center">
                                <label class="label text-[11px]">"Tokio Worker Concurrency"</label>
                                <span id="slider-readout" class="font-mono text-[12px] text-oxide font-bold">"64 workers"</span>
                            </div>
                            <div class="pt-3">
                                <input
                                    type="range"
                                    min="4"
                                    max="128"
                                    value="64"
                                    step="4"
                                    class="range"
                                    oninput="document.getElementById('slider-readout').textContent = this.value + ' workers'"
                                />
                            </div>
                            <div class="flex justify-between text-[10px] font-mono text-dim pt-1">
                                <span>"4 threads"</span>
                                <span>"Default: 64"</span>
                                <span>"128 threads"</span>
                            </div>
                        </div>
                    </div>

                    // Switch Toggles & Checkboxes
                    <div class="pt-6 border-t border-[rgba(236,233,227,0.08)] grid grid-cols-1 sm:grid-cols-2 gap-6">
                        <div class="flex items-center justify-between p-4 border border-[rgba(236,233,227,0.08)] bg-sunk">
                            <div>
                                <div class="text-[13.5px] font-medium text-paper">"Brotli Pre-Compression"</div>
                                <div class="text-[11px] font-mono text-dim">"Generate .br assets at compile time (quality: 11)"</div>
                            </div>
                            <label class="switch ml-4 cursor-pointer">
                                <input type="checkbox" checked="" />
                                <span class="switch-slider"></span>
                            </label>
                        </div>

                        <div class="flex items-center justify-between p-4 border border-[rgba(236,233,227,0.08)] bg-sunk">
                            <div>
                                <div class="text-[13.5px] font-medium text-paper">"Strict CSRF Protection"</div>
                                <div class="text-[11px] font-mono text-dim">"Validate HMAC tokens on every mutation"</div>
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
                            <span data-copy-label="" class="text-oxide font-medium">"copy rsx"</span>
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
                        <span>"Strict W3C accessible markup · Native keyboard support"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"Zero Layout Shift"</span>
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
                        <button onclick="copyText(this, 'yav add input')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add input"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/input.rs [1.8 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
