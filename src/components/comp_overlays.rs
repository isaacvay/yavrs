//! 5.15 Drawers, Popovers & Floating Tooltips Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentOverlays() -> Node {
    let script = Node::raw(r#"<script>
window.toggleShowroomDrawer = function() {
    var drawer = document.getElementById('showroom-demo-drawer');
    var backdrop = document.getElementById('showroom-demo-backdrop');
    if (!drawer || !backdrop) return;
    var isOpen = drawer.classList.contains('translate-x-0');
    if (isOpen) {
        drawer.classList.remove('translate-x-0');
        drawer.classList.add('translate-x-full');
        backdrop.classList.add('hidden');
    } else {
        backdrop.classList.remove('hidden');
        setTimeout(function() {
            drawer.classList.remove('translate-x-full');
            drawer.classList.add('translate-x-0');
        }, 10);
    }
};

window.toggleShowroomPopover = function() {
    var pop = document.getElementById('showroom-demo-popover');
    if (!pop) return;
    pop.classList.toggle('hidden');
};
</script>"#);

    rsx! {
        <section id="overlays" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.15"</span>
                <span class="t" data-i18n="comp.ovr.sec">"Drawers & Floating UI"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add drawer popover"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.ovr.title">"Sliding drawers & floating popovers."</h2>
                <button
                    onclick="copyText(this, 'yav add drawer popover tooltip')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add drawer popover"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.ovr.lede">
                "Accessible overlay primitives with focus management, backdrop dimming, collision-aware floating placement, and keyboard escape dismissals. Built with zero external floating libraries."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.ovr.preview_head">"LIVE PREVIEW — OVERLAY PRIMITIVES"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.ovr.preview_sub">"click triggers to open"</span>
                </div>
                <div class="p-8 space-y-8 bg-ink-2 relative overflow-hidden">
                    <div class="flex flex-wrap items-center gap-6">
                        // Drawer Trigger
                        <button onclick="toggleShowroomDrawer()" class="btn btn-oxide flex items-center gap-2 cursor-pointer">
                            {render_icon("layers", IconSize::Sm, Some("w-4 h-4"), None)}
                            <span data-i18n="comp.ovr.drawer_btn">"Toggle Drawer Sheet"</span>
                        </button>

                        // Popover Trigger
                        <div class="relative">
                            <button onclick="toggleShowroomPopover()" class="btn btn-line flex items-center gap-2 cursor-pointer">
                                {render_icon("sliders", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                                <span data-i18n="comp.ovr.pop_btn">"Filter Popover"</span>
                            </button>
                            // Floating Popover Box
                            <div id="showroom-demo-popover" class="hidden absolute left-0 top-full mt-2 w-72 p-4 bg-sunk border border-rule shadow-2xl rounded-xl z-30 space-y-3 font-mono text-xs">
                                <div class="font-bold text-paper flex items-center justify-between border-b border-rule pb-2">
                                    <span data-i18n="comp.ovr.pop_title">"Filter Settings"</span>
                                    <button onclick="toggleShowroomPopover()" class="text-dim hover:text-paper">"✕"</button>
                                </div>
                                <div class="space-y-2 text-dim text-[11.5px]">
                                    <label class="flex items-center gap-2 cursor-pointer">
                                        <input type="checkbox" checked=true class="accent-orange-500" />
                                        <span class="text-paper" data-i18n="comp.ovr.f1">"Active Connections"</span>
                                    </label>
                                    <label class="flex items-center gap-2 cursor-pointer">
                                        <input type="checkbox" checked=true class="accent-orange-500" />
                                        <span class="text-paper" data-i18n="comp.ovr.f2">"Brotli Pre-Compression"</span>
                                    </label>
                                    <label class="flex items-center gap-2 cursor-pointer">
                                        <input type="checkbox" class="accent-orange-500" />
                                        <span data-i18n="comp.ovr.f3">"Show Raw ASM Output"</span>
                                    </label>
                                </div>
                            </div>
                        </div>

                        // Tooltip Demo
                        <div class="group relative inline-block">
                            <button class="px-4 py-2 bg-sunk border border-rule rounded-lg text-paper font-mono text-xs flex items-center gap-2">
                                {render_icon("info", IconSize::Sm, Some("w-3.5 h-3.5 text-dim"), None)}
                                <span data-i18n="comp.ovr.tip_btn">"Hover for Tooltip"</span>
                            </button>
                            <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 hidden group-hover:block bg-ink-2 border border-oxide text-paper font-mono text-[11px] px-3 py-1.5 rounded shadow-xl whitespace-nowrap z-30" data-i18n="comp.ovr.tip_text">
                                "97% bare-metal HTTP speed"
                            </div>
                        </div>
                    </div>

                    // Drawer Overlay Simulated Inside Frame
                    <div id="showroom-demo-backdrop" onclick="toggleShowroomDrawer()" class="hidden absolute inset-0 bg-ink/80 backdrop-blur-sm z-40 transition-opacity"></div>
                    <div id="showroom-demo-drawer" class="absolute top-0 right-0 bottom-0 w-80 max-w-[85%] bg-sunk border-l border-rule p-6 shadow-2xl z-50 transform translate-x-full transition-transform duration-200 ease-in-out space-y-6">
                        <div class="flex items-center justify-between pb-3 border-b border-rule">
                            <span class="font-bold text-paper text-sm" data-i18n="comp.ovr.drawer_title">"Drawer Sheet"</span>
                            <button onclick="toggleShowroomDrawer()" class="text-dim hover:text-paper cursor-pointer font-mono text-xs" data-i18n="comp.ovr.close">
                                "Close [ESC]"
                            </button>
                        </div>
                        <p class="text-xs text-mute leading-relaxed font-mono" data-i18n="comp.ovr.drawer_desc">
                            "Slide-over navigation or settings sheet. Fully accessible with automatic ARIA management and focus trapping."
                        </p>
                        <div class="space-y-2 font-mono text-xs">
                            <div class="p-3 bg-ink-2 rounded border border-rule text-paper" data-i18n="comp.ovr.d1">"✓ Route Analytics"</div>
                            <div class="p-3 bg-ink-2 rounded border border-rule text-paper" data-i18n="comp.ovr.d2">"✓ Cluster Telemetry"</div>
                            <div class="p-3 bg-ink-2 rounded border border-rule text-paper" data-i18n="comp.ovr.d3">"✓ Security Audit Log"</div>
                        </div>
                    </div>
                </div>
            </div>

            // IDE Code Window
            <div class="ide-window">
                <div class="ide-head">
                    <div class="flex items-center h-full">
                        <div class="ide-tab">
                            {render_icon("layers", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/OverlayShowcase.rs"
                        </div>
                    </div>
                    <button data-copy="use yav::ui::{Drawer, DrawerPlacement, Popover, Tooltip};\n\nrsx! {\n    <Drawer id=&quot;settings-drawer&quot; placement={DrawerPlacement::Right} title=&quot;Settings&quot;>\n        <p>&quot;Drawer sheet body content&quot;</p>\n    </Drawer>\n    <Tooltip text=&quot;Hardware accelerated&quot;>\n        <button class=&quot;btn&quot;>&quot;Hover Me&quot;</button>\n    </Tooltip>\n}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                    </button>
                </div>
                <div class="flex bg-[#0b0b0d]">
                    <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                        "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"
                    </div>
                    <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                        <div class="token-comment">"// Drawer Sheet & Accessible Floating Overlays"</div>
                        <div><span class="token-keyword">"use "</span>"yav::ui::{"<span class="token-type">"Drawer"</span>", "<span class="token-type">"DrawerPlacement"</span>", "<span class="token-type">"Popover"</span>", "<span class="token-type">"Tooltip"</span>"};"</div>
                        <div class="pt-1"><span class="token-macro">"rsx!"</span>" {"</div>
                        <div>"    "<span class="token-tag">"<Drawer "</span><span class="token-attr">"id="</span><span class="token-string">"\"settings-drawer\" "</span><span class="token-attr">"placement="</span>"{"<span class="token-type">"DrawerPlacement"</span>"::"<span class="token-fn">"Right"</span>"} "<span class="token-attr">"title="</span><span class="token-string">"\"Settings\""</span><span class="token-tag">">"</span></div>
                        <div>"        "<span class="token-tag">"<p>"</span>"\"Drawer sheet body content\""<span class="token-tag">"</p>"</span></div>
                        <div>"    "<span class="token-tag">"</Drawer>"</span></div>
                        <div>"    "<span class="token-tag">"<Tooltip "</span><span class="token-attr">"text="</span><span class="token-string">"\"Hardware accelerated\""</span><span class="token-tag">">"</span></div>
                        <div>"        "<span class="token-tag">"<Button>"</span>"\"Hover Me\""<span class="token-tag">"</Button>"</span></div>
                        <div>"    "<span class="token-tag">"</Tooltip>"</span></div>
                        <div>"}"</div>
                    </div>
                </div>
                <div class="ide-status">
                    <span data-i18n="comp.ovr.ide_l">"Collision detection · Esc dismiss · Focus trap"</span>
                    <span class="text-ok" data-i18n="comp.ovr.ide_r">"Pure CSS/WASM Hybrid"</span>
                </div>
            </div>

            {script}
        </section>
    }
}
