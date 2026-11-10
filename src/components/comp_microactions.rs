//! 5.14 Zero-JS Micro-Actions & Reactive Signals Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentMicroActions() -> Node {
    let script = Node::raw(r#"<script>
window.testSignalCounter = function(delta) {
    var countEl = document.getElementById('signal-demo-count');
    var totalEl = document.getElementById('signal-demo-total');
    var barEl = document.getElementById('signal-demo-bar');
    if (!countEl || !totalEl || !barEl) return;
    var cur = parseInt(countEl.textContent, 10) || 1;
    var next = Math.max(1, Math.min(20, cur + delta));
    countEl.textContent = next;
    totalEl.textContent = '€' + (next * 49);
    barEl.style.width = (next * 5) + '%';
};

window.switchDemoTab = function(tabKey) {
    var tabs = ['cargo', 'sh', 'npm'];
    tabs.forEach(function(k) {
        var btn = document.getElementById('tab-btn-' + k);
        var panel = document.getElementById('tab-panel-' + k);
        if (btn) {
            if (k === tabKey) {
                btn.classList.add('border-oxide', 'text-paper');
                btn.classList.remove('border-transparent', 'text-dim');
            } else {
                btn.classList.remove('border-oxide', 'text-paper');
                btn.classList.add('border-transparent', 'text-dim');
            }
        }
        if (panel) {
            panel.style.display = (k === tabKey) ? 'block' : 'none';
        }
    });
};
</script>"#);

    rsx! {
        <section id="microactions" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.14"</span>
                <span class="t" data-i18n="comp.micro.sec">"Zero-JS Micro-Actions & Signals"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav:state & yav:*"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.micro.title">"Declarative micro-actions & reactive state."</h2>
                <button
                    onclick="copyText(this, 'yav add motion')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add motion"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.micro.lede">
                "Eliminate handwritten JavaScript event listeners. Yav provides declarative attributes for clipboard copy swaps, modal toggles, keyboard shortcuts, synchronized tabs, client-side list filtering, and reactive signals."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.micro.preview_head">"LIVE PREVIEW — MICRO-ACTIONS & SIGNALS"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.micro.preview_sub">"test interactive widgets"</span>
                </div>
                <div class="p-6 sm:p-8 space-y-8 bg-ink-2">
                    <div class="grid md:grid-cols-2 gap-8">
                        // Widget 1: Reactive Signal Calculator (yav:state)
                        <div class="p-6 rounded-xl border border-rule bg-sunk/60 space-y-4 font-mono text-xs">
                            <div class="flex items-center justify-between pb-2 border-b border-rule">
                                <span class="text-oxide font-bold">"yav:state=\"count: 1, price: 49\""</span>
                                <span class="dot dot-ok"></span>
                            </div>
                            <div class="flex items-center justify-between">
                                <span class="text-paper text-sm font-sans font-medium" data-i18n="comp.micro.qty">"Quantity Selector:"</span>
                                <div class="flex items-center gap-3">
                                    <button onclick="testSignalCounter(-1)" class="w-8 h-8 rounded bg-ink-2 border border-rule text-paper font-bold hover:border-oxide cursor-pointer">
                                        "-"
                                    </button>
                                    <span id="signal-demo-count" class="font-bold text-paper text-base w-6 text-center">"1"</span>
                                    <button onclick="testSignalCounter(1)" class="w-8 h-8 rounded bg-ink-2 border border-rule text-paper font-bold hover:border-oxide cursor-pointer">
                                        "+"
                                    </button>
                                </div>
                            </div>
                            <div class="flex items-center justify-between pt-2">
                                <span class="text-dim" data-i18n="comp.micro.total">"Total Price:"</span>
                                <span id="signal-demo-total" class="text-lg font-bold text-oxide">"€49"</span>
                            </div>
                            // Dynamic gauge track
                            <div class="space-y-1 pt-2">
                                <div class="flex justify-between text-[10px] text-faint">
                                    <span data-i18n="comp.micro.gauge">"Capacity Gauge"</span>
                                    <span data-i18n="comp.micro.max_units">"max 20 units"</span>
                                </div>
                                <div class="h-2 w-full bg-ink-2 rounded overflow-hidden">
                                    <div id="signal-demo-bar" class="h-full bg-oxide rounded transition-all duration-200" style="width: 5%;"></div>
                                </div>
                            </div>
                        </div>

                        // Widget 2: Synchronized Tabs & Copy Feedback (yav:tab-group & yav:copy)
                        <div class="p-6 rounded-xl border border-rule bg-sunk/60 space-y-4 font-mono text-xs">
                            <div class="flex items-center justify-between pb-2 border-b border-rule">
                                <span class="text-oxide font-bold">"yav:tab-group & yav:copy"</span>
                                <span class="chip text-[10px]" data-i18n="comp.micro.chip">"< 2.1 KB runtime"</span>
                            </div>
                            <div class="flex gap-2 border-b border-rule pb-2">
                                <button id="tab-btn-cargo" onclick="switchDemoTab('cargo')" class="px-3 py-1 border-b-2 border-oxide text-paper font-semibold cursor-pointer">
                                    "Cargo"
                                </button>
                                <button id="tab-btn-sh" onclick="switchDemoTab('sh')" class="px-3 py-1 border-b-2 border-transparent text-dim hover:text-paper cursor-pointer">
                                    "Shell"
                                </button>
                                <button id="tab-btn-npm" onclick="switchDemoTab('npm')" class="px-3 py-1 border-b-2 border-transparent text-dim hover:text-paper cursor-pointer" data-i18n="comp.micro.tab_nonode">
                                    "No Node"
                                </button>
                            </div>
                            <div class="min-h-[50px]">
                                <div id="tab-panel-cargo">
                                    <div class="flex items-center justify-between p-3 bg-ink-2 rounded border border-rule">
                                        <code class="text-paper">"cargo install yav-cli"</code>
                                        <button onclick="copyText(this, 'cargo install yav-cli')" class="text-dim hover:text-oxide cursor-pointer">
                                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                                        </button>
                                    </div>
                                </div>
                                <div id="tab-panel-sh" style="display:none;">
                                    <div class="flex items-center justify-between p-3 bg-ink-2 rounded border border-rule">
                                        <code class="text-paper">"curl -fsSL https://yav.rs/install.sh | sh"</code>
                                        <button onclick="copyText(this, 'curl -fsSL https://yav.rs/install.sh | sh')" class="text-dim hover:text-oxide cursor-pointer">
                                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                                        </button>
                                    </div>
                                </div>
                                <div id="tab-panel-npm" style="display:none;">
                                    <div class="p-3 bg-ink-2 rounded border border-rule text-dim text-[11px]" data-i18n="comp.micro.nonode_desc">
                                        "Zero npm / node_modules required. Standalone binary runs everywhere."
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // IDE Code Window
            <div class="ide-window">
                <div class="ide-head">
                    <div class="flex items-center h-full">
                        <div class="ide-tab">
                            {render_icon("sliders", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/MicroActionsDemo.rs"
                        </div>
                    </div>
                    <button data-copy="<div yav:state=&quot;count: 1, price: 49&quot;>\n    <button yav:on-click=&quot;count = count + 1&quot;>&quot;+&quot;</button>\n    <span yav:text=&quot;count&quot;>1</span>\n    <span yav:text=&quot;'€' + (count * price)&quot;>€49</span>\n</div>" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                    </button>
                </div>
                <div class="flex bg-[#0b0b0d]">
                    <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                        "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"
                    </div>
                    <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                        <div class="token-comment">"// Declarative Micro-Signals & Tab Group Attributes"</div>
                        <div><span class="token-tag">"<div "</span><span class="token-attr">"yav:state="</span><span class="token-string">"\"count: 1, unit_price: 49\""</span><span class="token-tag">">"</span></div>
                        <div>"    "<span class="token-tag">"<button "</span><span class="token-attr">"yav:on-click="</span><span class="token-string">"\"count = count + 1\""</span><span class="token-tag">">"</span>"\"+\""<span class="token-tag">"</button>"</span></div>
                        <div>"    "<span class="token-tag">"<span "</span><span class="token-attr">"yav:text="</span><span class="token-string">"\"count\""</span><span class="token-tag">">"</span>"\"1\""<span class="token-tag">"</span>"</span></div>
                        <div>"    "<span class="token-tag">"<span "</span><span class="token-attr">"yav:text="</span><span class="token-string">"\"'€' + (count * unit_price)\""</span><span class="token-tag">">"</span>"\"€49\""<span class="token-tag">"</span>"</span></div>
                        <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"yav:style-width="</span><span class="token-string">"\"(count * 5) + '%'\""</span><span class="token-tag">" />"</span></div>
                        <div><span class="token-tag">"</div>"</span></div>
                    </div>
                </div>
                <div class="ide-status">
                    <span data-i18n="comp.micro.ide_l">"Direct HTML5 attribute hydration"</span>
                    <span class="text-ok" data-i18n="comp.micro.ide_r">"Zero JS Framework Overhead"</span>
                </div>
            </div>

            {script}
        </section>
    }
}
