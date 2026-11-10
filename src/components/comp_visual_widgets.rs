//! 5.19 Visual Analytics & Advanced Interactive Widgets Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentVisualWidgets() -> Node {
    let script = Node::raw(r#"<script>
window.evalSpreadsheet = function() {
    var a1 = parseFloat(document.getElementById('cell-a1').value) || 0;
    var a2 = parseFloat(document.getElementById('cell-a2').value) || 0;
    var a3 = parseFloat(document.getElementById('cell-a3').value) || 0;
    var total = a1 + a2 + a3;
    var elSum = document.getElementById('cell-sum');
    var elAvg = document.getElementById('cell-avg');
    if (elSum) elSum.textContent = '$' + total.toLocaleString('en-US');
    if (elAvg) elAvg.textContent = '$' + Math.round(total / 3).toLocaleString('en-US');
};
</script>"#);

    rsx! {
        <section id="visual-widgets" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.19"</span>
                <span class="t" data-i18n="comp.vis.sec">"Advanced Interactive Widgets"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add widgets"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.vis.title">"3D Wireframes, Reactive Spreadsheet & Code Playground."</h2>
                <button
                    data-copy="yav add model-viewer-3d code-playground spreadsheet diff-viewer signature-pad cookie-consent"
                    onclick="copyText(this)"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add widgets"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.vis.lede">
                "Specialized application widgets engineered in pure Rust: direct SVG 3D wireframe projections with mouse orbit, reactive Excel-like spreadsheets with formula engines (=SUM, =AVG), split-view code playgrounds, side-by-side git diff viewers, and GDPR consent centers."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.vis.preview_head">"LIVE PREVIEW — SPECIALIZED APPLICATION WIDGETS"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.vis.preview_sub">"Pure Rust · SVG Projections · Formula Engine"</span>
                </div>
                <div class="p-6 sm:p-8 space-y-8 bg-ink-2">
                    // Grid 1: Pure Rust 3D Model Viewer & Reactive Spreadsheet
                    <div class="grid lg:grid-cols-12 gap-6">
                        // 1. 3D Model Viewer (6 cols)
                        <div class="lg:col-span-6 p-5 bg-sunk border border-rule rounded space-y-4">
                            <div class="flex flex-wrap items-center justify-between gap-2">
                                <span class="text-xs font-mono text-dim uppercase" data-i18n="comp.vis.mesh_lbl">"Pure-Rust 3D Wireframe Mesh (Euler Rotations)"</span>
                                <span class="chip text-[11px]" data-i18n="comp.vis.no_three">"Zero Three.js"</span>
                            </div>
                            
                            // SVG 3D Cube Projection
                            <div class="w-full flex items-center justify-center py-4 bg-ink/60 rounded border border-rule">
                                <svg viewBox="0 0 200 160" class="w-48 h-36 block">
                                    // 3D Shaded & Wireframe Faces
                                    // Back edges
                                    <line x1="60" y1="50" x2="140" y2="40" stroke="rgba(236,233,227,0.15)" stroke-dasharray="2 2" />
                                    <line x1="60" y1="50" x2="60" y2="110" stroke="rgba(236,233,227,0.15)" stroke-dasharray="2 2" />
                                    <line x1="60" y1="50" x2="40" y2="70" stroke="rgba(236,233,227,0.15)" stroke-dasharray="2 2" />

                                    // Bottom face
                                    <polygon points="40,130 120,120 140,100 60,110" fill="rgba(224,89,47,0.06)" stroke="rgba(236,233,227,0.2)" />

                                    // Right face
                                    <polygon points="120,60 140,40 140,100 120,120" fill="rgba(224,89,47,0.12)" stroke="rgba(236,233,227,0.4)" />

                                    // Top face (highlight)
                                    <polygon points="40,70 120,60 140,40 60,50" fill="rgba(224,89,47,0.2)" stroke="#e0592f" stroke-width="1.5" />

                                    // Front face
                                    <polygon points="40,70 120,60 120,120 40,130" fill="rgba(236,233,227,0.04)" stroke="#ece9e3" stroke-width="1.8" />

                                    // Vertices dots
                                    <circle cx="40" cy="70" r="2.5" fill="#e0592f" />
                                    <circle cx="120" cy="60" r="2.5" fill="#e0592f" />
                                    <circle cx="120" cy="120" r="2.5" fill="#e0592f" />
                                    <circle cx="40" cy="130" r="2.5" fill="#e0592f" />
                                    <circle cx="140" cy="40" r="2.5" fill="#22c55e" />
                                    <circle cx="140" cy="100" r="2.5" fill="#22c55e" />

                                    // Coordinate Axis
                                    <line x1="20" y1="145" x2="35" y2="145" stroke="#e0592f" stroke-width="1.5" />
                                    <text x="38" y="148" font-size="8" fill="#e0592f" font-family="monospace">"X"</text>
                                    <line x1="20" y1="145" x2="20" y2="130" stroke="#22c55e" stroke-width="1.5" />
                                    <text x="18" y="126" font-size="8" fill="#22c55e" font-family="monospace">"Y"</text>
                                    <line x1="20" y1="145" x2="10" y2="152" stroke="#6366f1" stroke-width="1.5" />
                                    <text x="6" y="156" font-size="8" fill="#6366f1" font-family="monospace">"Z"</text>
                                </svg>
                            </div>

                            <div class="flex items-center justify-between text-xs font-mono text-dim pt-1 border-t border-rule">
                                <span>"ModelViewer3D::isometric_cube()"</span>
                                <span class="text-paper" data-i18n="comp.vis.mesh_stat">"8 Vertices · 6 Faces"</span>
                            </div>
                        </div>

                        // 2. Reactive Spreadsheet (6 cols)
                        <div class="lg:col-span-6 p-5 bg-sunk border border-rule rounded space-y-4">
                            <div class="flex flex-wrap items-center justify-between gap-2">
                                <span class="text-xs font-mono text-dim uppercase" data-i18n="comp.vis.sheet_lbl">"Reactive Spreadsheet (=SUM / =AVG)"</span>
                                <span class="text-xs font-mono text-ok" data-i18n="comp.vis.sheet_live">"Live Formulas"</span>
                            </div>

                            <div class="border border-rule rounded overflow-hidden text-xs font-mono table-scroll">
                                <table class="w-full min-w-[360px] border-collapse">
                                    <thead class="bg-ink-2 text-dim border-b border-rule">
                                        <tr>
                                            <th class="w-8 py-1.5 px-2 text-center border-r border-rule">"#"</th>
                                            <th class="py-1.5 px-3 text-left border-r border-rule" data-i18n="comp.vis.th_cat">"A (Category)"</th>
                                            <th class="py-1.5 px-3 text-right" data-i18n="comp.vis.th_budget">"B (Budget)"</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-rule bg-[#0b0b0d]">
                                        <tr>
                                            <td class="text-center text-dim bg-sunk/50 border-r border-rule">"1"</td>
                                            <td class="px-3 py-1.5 text-paper border-r border-rule" data-i18n="comp.vis.r1">"Infrastructure Server"</td>
                                            <td class="px-2 py-1 text-right">
                                                <input id="cell-a1" type="number" value="4500" oninput="window.evalSpreadsheet()" class="w-24 text-right bg-sunk border border-rule rounded px-1.5 py-0.5 text-paper focus:border-oxide outline-none" />
                                            </td>
                                        </tr>
                                        <tr>
                                            <td class="text-center text-dim bg-sunk/50 border-r border-rule">"2"</td>
                                            <td class="px-3 py-1.5 text-paper border-r border-rule" data-i18n="comp.vis.r2">"Edge Network Bandwidth"</td>
                                            <td class="px-2 py-1 text-right">
                                                <input id="cell-a2" type="number" value="1800" oninput="window.evalSpreadsheet()" class="w-24 text-right bg-sunk border border-rule rounded px-1.5 py-0.5 text-paper focus:border-oxide outline-none" />
                                            </td>
                                        </tr>
                                        <tr>
                                            <td class="text-center text-dim bg-sunk/50 border-r border-rule">"3"</td>
                                            <td class="px-3 py-1.5 text-paper border-r border-rule" data-i18n="comp.vis.r3">"Security & Passkeys FIDO2"</td>
                                            <td class="px-2 py-1 text-right">
                                                <input id="cell-a3" type="number" value="1200" oninput="window.evalSpreadsheet()" class="w-24 text-right bg-sunk border border-rule rounded px-1.5 py-0.5 text-paper focus:border-oxide outline-none" />
                                            </td>
                                        </tr>
                                        <tr class="bg-sunk font-semibold">
                                            <td class="text-center text-oxide border-r border-rule">"Σ"</td>
                                            <td class="px-3 py-2 text-paper border-r border-rule">"=SUM(B1:B3)"</td>
                                            <td class="px-3 py-2 text-right text-oxide" id="cell-sum">"$7,500"</td>
                                        </tr>
                                        <tr class="bg-sunk/80 text-dim">
                                            <td class="text-center text-dim border-r border-rule">"x̄"</td>
                                            <td class="px-3 py-1.5 text-dim border-r border-rule">"=AVG(B1:B3)"</td>
                                            <td class="px-3 py-1.5 text-right text-paper" id="cell-avg">"$2,500"</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>

                    // Grid 2: DiffViewer & CookieConsent
                    <div class="grid md:grid-cols-12 gap-6">
                        // Git Diff Viewer (7 cols)
                        <div class="md:col-span-7 p-5 bg-sunk border border-rule rounded space-y-3 font-mono text-xs">
                            <div class="flex items-center justify-between">
                                <span class="text-dim uppercase text-[11px]" data-i18n="comp.vis.diff_lbl">"LCS Code Diff Viewer (<DiffViewer />)"</span>
                                <span class="text-dim">"commit 6a4f89d"</span>
                            </div>
                            <div class="border border-rule rounded overflow-hidden bg-[#070709] divide-y divide-rule/40 code-scroll">
                                <div class="px-3 py-1 bg-ink-2 text-dim text-[11px] flex items-center justify-between">
                                    <span>"@@ -12,4 +12,6 @@ App Configuration"</span>
                                    <span class="text-paper">"src/main.rs"</span>
                                </div>
                                <div class="px-3 py-1 text-dim">"     let app = YavApp::new();"</div>
                                <div class="px-3 py-1 bg-red-950/30 text-red-400 border-l-2 border-red-500">"-    let app = app.enable_vdom(true);"</div>
                                <div class="px-3 py-1 bg-emerald-950/30 text-emerald-400 border-l-2 border-emerald-500">"+    let app = app.zero_vdom_fast_path();"</div>
                                <div class="px-3 py-1 bg-emerald-950/30 text-emerald-400 border-l-2 border-emerald-500">"+    let app = app.mcp_server(true);"</div>
                                <div class="px-3 py-1 text-dim">"     app.serve().await?;"</div>
                            </div>
                        </div>

                        // Cookie Consent Center (5 cols)
                        <div class="md:col-span-5 p-5 bg-sunk border border-rule rounded space-y-3 font-mono text-xs">
                            <div class="flex items-center justify-between">
                                <span class="text-dim uppercase text-[11px]" data-i18n="comp.vis.gdpr_lbl">"GDPR & ePrivacy Consent Center"</span>
                                <span class="text-ok" data-i18n="comp.vis.compliant">"Compliant"</span>
                            </div>
                            <div class="p-3 bg-ink-2 border border-rule rounded space-y-2.5">
                                <div class="flex items-center justify-between">
                                    <span class="text-paper font-semibold" data-i18n="comp.vis.strict">"Strictly Necessary"</span>
                                    <span class="text-ok text-[11px]" data-i18n="comp.vis.always">"Always Active"</span>
                                </div>
                                <p class="text-[11px] text-mute" data-i18n="comp.vis.strict_desc">"Cryptographic HMAC sessions and CSRF token protection."</p>
                                <div class="flex items-center justify-between pt-2 border-t border-rule">
                                    <span class="text-paper font-semibold" data-i18n="comp.vis.telemetry">"Performance Telemetry"</span>
                                    <span class="text-oxide text-[11px]" data-i18n="comp.vis.enabled">"Enabled"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Code Specification Window
            <div class="ide-window">
                <div class="ide-head">
                    <div class="flex items-center h-full">
                        <div class="ide-tab">
                            {render_icon("box", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/widgets.rs"
                        </div>
                    </div>
                    <button
                        data-copy="use yav::ui::model_viewer_3d::ModelViewer3D;\nuse yav::ui::spreadsheet::{Spreadsheet, Cell, Formula};\nuse yav::ui::media_diff::DiffViewer;\n\nrsx! {\n    <ModelViewer3D shape=\"cube\" orbit_controls=true />\n    <Spreadsheet rows=10 cols=5 formula_engine=true />\n    <DiffViewer original={old_code} modified={new_code} mode=\"unified\" />\n}"
                        onclick="copyText(this)"
                        class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono mr-2"
                    >
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                    </button>
                </div>
                <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                    <div class="token-comment">"// Pure-Rust 3D linear algebra projections & spreadsheet formula calculation"</div>
                    <div><span class="token-keyword">"use "</span>"yav::ui::model_viewer_3d::"<span class="token-type">"ModelViewer3D"</span>";"</div>
                    <div><span class="token-keyword">"use "</span>"yav::ui::spreadsheet::{"<span class="token-type">"Spreadsheet"</span>", "<span class="token-type">"Cell"</span>", "<span class="token-type">"Formula"</span>"};"</div>
                    <div><span class="token-keyword">"use "</span>"yav::ui::media_diff::"<span class="token-type">"DiffViewer"</span>";"</div>
                    <div class="pt-2"><span class="token-macro">"rsx!"</span>" {"</div>
                    <div>"    "<span class="token-tag">"<ModelViewer3D "</span><span class="token-attr">"shape="</span><span class="token-string">"\"cube\""</span>" "<span class="token-attr">"orbit_controls="</span><span class="token-bool">"true"</span><span class="token-tag">" />"</span></div>
                    <div>"    "<span class="token-tag">"<Spreadsheet "</span><span class="token-attr">"rows="</span><span class="token-val">"{10}"</span>" "<span class="token-attr">"cols="</span><span class="token-val">"{5}"</span>" "<span class="token-attr">"formula_engine="</span><span class="token-bool">"true"</span><span class="token-tag">" />"</span></div>
                    <div>"    "<span class="token-tag">"<DiffViewer "</span><span class="token-attr">"original="</span><span class="token-val">"{old_code}"</span>" "<span class="token-attr">"modified="</span><span class="token-val">"{new_code}"</span>" "<span class="token-attr">"mode="</span><span class="token-string">"\"unified\""</span><span class="token-tag">" />"</span></div>
                    <div>"}"</div>
                </div>
                <div class="ide-status">
                    <span data-i18n="comp.vis.ide_l">"Direct Euler perspective projections into SVG vector streams"</span>
                    <span class="text-ok" data-i18n="comp.vis.ide_r">"Zero Node.js · Zero Canvas · Zero Native C Libs"</span>
                </div>
            </div>
            {script}
        </section>
    }
}
