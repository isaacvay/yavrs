//! 5.13 Cursor Spotlight & 3D Tilt Plates Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentCursor() -> Node {
    let script = Node::raw(r#"<script>
(function() {
    var plate = document.getElementById('demo-tilt-plate');
    var coords = document.getElementById('demo-mouse-coords');
    if (!plate) return;

    plate.addEventListener('mousemove', function(e) {
        var rect = plate.getBoundingClientRect();
        var x = e.clientX - rect.left;
        var y = e.clientY - rect.top;
        var rx = ((x / rect.width) - 0.5) * 2;
        var ry = ((y / rect.height) - 0.5) * 2;
        var rotX = -ry * 14;
        var rotY = rx * 14;
        plate.style.transform = 'perspective(800px) rotateX(' + rotX + 'deg) rotateY(' + rotY + 'deg)';
        plate.style.setProperty('--spotlight-x', x + 'px');
        plate.style.setProperty('--spotlight-y', y + 'px');
        if (coords) {
            coords.removeAttribute('data-i18n');
            coords.textContent = 'x: ' + Math.round(x) + 'px · y: ' + Math.round(y) + 'px · rx: ' + rx.toFixed(2);
        }
    });

    plate.addEventListener('mouseleave', function() {
        plate.style.transform = 'perspective(800px) rotateX(0deg) rotateY(0deg)';
        if (coords) {
            coords.setAttribute('data-i18n', 'comp.cursor.idle_coords');
            coords.textContent = window.yavT ? window.yavT('comp.cursor.idle_coords', 'x: 0px · y: 0px (idle)') : 'x: 0px · y: 0px (idle)';
        }
    });

    // Magnetic button demo
    var magBtn = document.getElementById('demo-mag-btn');
    if (magBtn) {
        magBtn.addEventListener('mousemove', function(e) {
            var rect = magBtn.getBoundingClientRect();
            var mx = (e.clientX - rect.left - rect.width / 2) * 0.35;
            var my = (e.clientY - rect.top - rect.height / 2) * 0.35;
            magBtn.style.transform = 'translate3d(' + mx + 'px, ' + my + 'px, 0)';
        });
        magBtn.addEventListener('mouseleave', function() {
            magBtn.style.transform = 'translate3d(0, 0, 0)';
        });
    }
})();
</script>"#);

    rsx! {
        <section id="cursor" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.13"</span>
                <span class="t" data-i18n="comp.cursor.sec">"Cursor & 3D Tilt"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add cursor-spotlight"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.cursor.title">"Reactive 3D tilt & cursor spotlights."</h2>
                <button
                    onclick="copyText(this, 'yav add cursor-spotlight')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add cursor-spotlight"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.cursor.lede">
                "Hardware-accelerated mouse coordinate injection publishing CSS variables (--yav-mouse-x, --yav-mouse-y, --yav-mouse-rx, --yav-mouse-ry). Powers 3D elastic tilt perspectives, radial spotlight glows, and magnetic buttons in ~1.3 KB."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.cursor.preview_head">"LIVE PREVIEW — 3D TILT & SPOTLIGHT"</span>
                    </span>
                    <span id="demo-mouse-coords" data-i18n="comp.cursor.hover_hint" class="text-oxide font-mono text-[11px]">"hover card to test 3D tilt"</span>
                </div>
                <div class="p-8 space-y-8 bg-ink-2">
                    <div class="grid md:grid-cols-2 gap-8 items-center">
                        // 3D Tilt Plate with dynamic spotlight glow
                        <div
                            id="demo-tilt-plate"
                            class="p-8 rounded-2xl border border-rule bg-sunk/80 space-y-4 cursor-pointer transition-transform duration-75 ease-out select-none relative overflow-hidden"
                            style="transform-style: preserve-3d; will-change: transform;"
                        >
                            <div class="flex items-center justify-between">
                                <span class="font-mono text-xs text-oxide font-bold">"TiltPlate max_deg=14"</span>
                                <span class="dot dot-ok"></span>
                            </div>
                            <h3 class="text-2xl font-bold text-paper" data-i18n="comp.cursor.surface_title">"Interactive 3D Surface"</h3>
                            <p class="text-sm text-mute leading-relaxed" data-i18n="comp.cursor.surface_desc">
                                "Move your pointer across this card. High-precision normalized mouse vectors compute fluid 3D perspective pitch and yaw."
                            </p>
                            <div class="pt-4 border-t border-rule flex items-center justify-between text-xs font-mono text-dim">
                                <span>"--yav-mouse-rx / ry"</span>
                                <span class="text-paper" data-i18n="comp.cursor.gpu">"GPU Composited"</span>
                            </div>
                        </div>

                        // Magnetic Button Showcase
                        <div class="p-8 rounded-2xl border border-rule bg-sunk/40 space-y-6 flex flex-col items-center justify-center text-center">
                            <span class="font-mono text-xs text-dim" data-i18n="comp.cursor.mag_head">"MAGNETIC BUTTON ATTRACTION"</span>
                            <button
                                id="demo-mag-btn"
                                class="btn btn-oxide px-8 py-3 rounded-xl font-medium shadow-lg transition-transform duration-100 ease-out cursor-pointer"
                                style="will-change: transform;"
                            >
                                {render_icon("zap", IconSize::Sm, Some("w-4 h-4 mr-2"), None)}
                                <span data-i18n="comp.cursor.mag_btn">"Magnetic Button"</span>
                            </button>
                            <p class="text-xs text-dim max-w-xs font-mono" data-i18n="comp.cursor.mag_desc">
                                "Pulls subtly toward cursor coordinates within activation radius."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // IDE Code Window
            <div class="ide-window">
                <div class="ide-head">
                    <div class="flex items-center h-full">
                        <div class="ide-tab">
                            {render_icon("box", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/InteractiveCard.rs"
                        </div>
                    </div>
                    <button data-copy="use yav::ui::{CursorSpotlight, TiltPlate, Magnetic};\n\nrsx! {\n    <CursorSpotlight radius_px={450}>\n        <TiltPlate max_deg={12.0} glare=true class=&quot;p-8 border rounded-2xl&quot;>\n            <h3>&quot;3D Interactive Card&quot;</h3>\n            <Magnetic strength={0.35}>\n                <button class=&quot;btn btn-oxide&quot;>&quot;Magnetic&quot;</button>\n            </Magnetic>\n        </TiltPlate>\n    </CursorSpotlight>\n}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                    </button>
                </div>
                <div class="flex bg-[#0b0b0d]">
                    <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                        "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"
                    </div>
                    <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                        <div class="token-comment">"// 3D Tilt, Cursor Spotlight & Magnetic Physics"</div>
                        <div><span class="token-keyword">"use "</span>"yav::ui::{"<span class="token-type">"CursorSpotlight"</span>", "<span class="token-type">"TiltPlate"</span>", "<span class="token-type">"Magnetic"</span>"};"</div>
                        <div class="pt-1"><span class="token-macro">"rsx!"</span>" {"</div>
                        <div>"    "<span class="token-tag">"<CursorSpotlight "</span><span class="token-attr">"radius_px="</span>"{"<span class="token-num">"450"</span>"}"<span class="token-tag">">"</span></div>
                        <div>"        "<span class="token-tag">"<TiltPlate "</span><span class="token-attr">"max_deg="</span>"{"<span class="token-num">"12.0"</span>"} "<span class="token-attr">"glare="</span><span class="token-bool">"true"</span>" "<span class="token-attr">"class="</span><span class="token-string">"\"p-8 border rounded-2xl\""</span><span class="token-tag">">"</span></div>
                        <div>"            "<span class="token-tag">"<h3>"</span>"\"3D Interactive Card\""<span class="token-tag">"</h3>"</span></div>
                        <div>"            "<span class="token-tag">"<Magnetic "</span><span class="token-attr">"strength="</span>"{"<span class="token-num">"0.35"</span>"}"<span class="token-tag">">"</span></div>
                        <div>"                "<span class="token-tag">"<button "</span><span class="token-attr">"class="</span><span class="token-string">"\"btn btn-oxide\""</span><span class="token-tag">">"</span>"\"Action\""<span class="token-tag">"</button>"</span></div>
                        <div>"            "<span class="token-tag">"</Magnetic>"</span></div>
                        <div>"        "<span class="token-tag">"</TiltPlate>"</span></div>
                        <div>"    "<span class="token-tag">"</CursorSpotlight>"</span></div>
                        <div>"}"</div>
                    </div>
                </div>
                <div class="ide-status">
                    <span data-i18n="comp.cursor.ide_l">"Passive rAF coordinate loop · ~1.3 KB runtime"</span>
                    <span class="text-ok" data-i18n="comp.cursor.ide_r">"GPU Perspective Matrix"</span>
                </div>
            </div>

            {script}
        </section>
    }
}
