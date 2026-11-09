//! Documentation Section 4: Interactivity & UI (Zero-JS Micro-Actions, Cursor Tracking, Spring Motion, Signals, 8 Themes, HTMX, 13,000+ Icons, UI Scaffolding)

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn DocsInteractivity() -> Node {
    rsx! {
        <div class="space-y-20">
            // =========================================================================
            // 4.1 ZERO-JS DECLARATIVE INTERACTIVITY & MICRO-ACTIONS
            // =========================================================================
            <section id="zero-js-microactions" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.1"</span>
                    <span class="t">"Zero-JS Micro-Actions & HTML5 Attributes"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav:* attrs"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Rich client interactivity without writing a line of JavaScript."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Yav v0.1.5 introduces declarative "
                    <code class="inline">"yav:*"</code>
                    " attributes that wire accessible dialogs, drawers, copy buttons, synchronized tabs, global keyboard shortcuts, and live search filters powered by an ultra-compact (< 2.1 KB) micro-runtime."
                </p>

                // IDE LOOK WINDOW: MICRO-ACTIONS
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/DeclarativeNav.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-oxide font-mono">"yav:* micro-actions"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// 1. Drawer toggle with click-outside and auto-close on resize"</div>
                        <div><span class="token-tag">"<button "</span><span class="token-attr">"yav:toggle="</span><span class="token-string">"\"mobile-drawer\" "</span><span class="token-attr">"yav:group="</span><span class="token-string">"\"nav-menus\""</span><span class="token-tag">">"</span>"Menu"<span class="token-tag">"</button>"</span></div>
                        <div><span class="token-tag">"<dialog "</span><span class="token-attr">"id="</span><span class="token-string">"\"mobile-drawer\" "</span><span class="token-attr">"yav:click-outside="</span><span class="token-string">"\"close\" "</span><span class="token-attr">"yav:close-on-resize="</span><span class="token-string">"\"1024\""</span><span class="token-tag">">"</span>"..."<span class="token-tag">"</dialog>"</span></div>
                        <div class="pt-2 token-comment">"// 2. Global Hotkey (Cmd+K / Ctrl+K) binding"</div>
                        <div><span class="token-tag">"<button "</span><span class="token-attr">"yav:toggle="</span><span class="token-string">"\"search-modal\" "</span><span class="token-attr">"yav:hotkey="</span><span class="token-string">"\"cmd+k\""</span><span class="token-tag">">"</span>"Search (Cmd+K)"<span class="token-tag">"</button>"</span></div>
                        <div class="pt-2 token-comment">"// 3. Clipboard copy with 2-second label swap and toast notification"</div>
                        <div><span class="token-tag">"<button "</span><span class="token-attr">"yav:copy="</span><span class="token-string">"\"cargo install yav-cli\" "</span><span class="token-attr">"yav:copy-label="</span><span class="token-string">"\"Copié !\""</span><span class="token-tag">">"</span>"Copy Command"<span class="token-tag">"</button>"</span></div>
                        <div class="pt-2 token-comment">"// 4. Synchronized multi-pane tab group"</div>
                        <div><span class="token-tag">"<div "</span><span class="token-attr">"yav:tab-group="</span><span class="token-string">"\"install-tabs\""</span><span class="token-tag">">"</span></div>
                        <div>"    "<span class="token-tag">"<button "</span><span class="token-attr">"yav:tab="</span><span class="token-string">"\"cargo\""</span><span class="token-tag">">"</span>"Cargo"<span class="token-tag">"</button>"</span>" "<span class="token-tag">"<button "</span><span class="token-attr">"yav:tab="</span><span class="token-string">"\"curl\""</span><span class="token-tag">">"</span>"cURL"<span class="token-tag">"</button>"</span></div>
                        <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"yav:tab-panel="</span><span class="token-string">"\"cargo\""</span><span class="token-tag">">"</span>"cargo install yav-cli"<span class="token-tag">"</div>"</span></div>
                        <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"yav:tab-panel="</span><span class="token-string">"\"curl\" "</span><span class="token-attr">"hidden"</span><span class="token-tag">">"</span>"curl -fsSL https://yav.rs/install.sh | sh"<span class="token-tag">"</div>"</span></div>
                        <div><span class="token-tag">"</div>"</span></div>
                    </div>
                    <div class="ide-status">
                        <span>"~2.1 KB micro-runtime · Tree-shaken on static pages"</span>
                        <span class="text-ok">"Zero JS boilerplate"</span>
                    </div>
                </div>

                <div class="table-scroll pb-2">
                    <table class="dtable min-w-[620px]">
                        <thead>
                            <tr>
                                <th>"Attribute"</th>
                                <th>"Value Example"</th>
                                <th>"Behavior & Accessibility"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="us">
                                <td class="text-paper font-semibold">"yav:toggle"</td>
                                <td class="text-oxide">"\"modal-id\""</td>
                                <td>"Toggles [open] or [hidden] state, manages aria-expanded, ESC key closes"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"yav:click-outside"</td>
                                <td class="text-oxide">"\"close\""</td>
                                <td>"Dismisses dialog or popover when clicking anywhere outside target"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"yav:group"</td>
                                <td class="text-oxide">"\"nav-menus\""</td>
                                <td>"Exclusive popover group: opening one closes all other elements in the group"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"yav:close-on-resize"</td>
                                <td class="text-oxide">"\"1024\""</td>
                                <td>"Closes mobile drawers when browser viewport width exceeds threshold"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"yav:hotkey"</td>
                                <td class="text-oxide">"\"cmd+k\""</td>
                                <td>"Binds global keyboard shortcut with cross-platform Cmd / Ctrl normalisation"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"yav:copy"</td>
                                <td class="text-oxide">"\"text to copy\""</td>
                                <td>"Copies string to clipboard; yav:copy-label swaps button text for 2 seconds"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"yav:filter"</td>
                                <td class="text-oxide">"\"input\""</td>
                                <td>"Client-side search filter targeting selectors (yav:filter-target=\".card\")"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"yav:optimistic"</td>
                                <td class="text-oxide">"\"like-btn\""</td>
                                <td>"Applies instant UI mutations before server response with auto-rollback on network failure"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"yav:presence-room"</td>
                                <td class="text-oxide">"\"doc:42\""</td>
                                <td>"Wires collaborative live presence beacons, cursor tracks, and member count over SSE/WS"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"yav:offline-queue"</td>
                                <td class="text-oxide">"\"sync-orders\""</td>
                                <td>"Local-First IndexedDB mutation buffer: auto-drains and merges with CRDT on reconnect"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>

            // =========================================================================
            // 4.2 NATIVE CURSOR TRACKING & 3D TILT ENGINE
            // =========================================================================
            <section id="cursor-tracking" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.2"</span>
                    <span class="t">"Cursor Tracking & Reactive 3D Effects"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav:cursor"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Hardware-accelerated mouse coordinates & 3D perspective."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "The cursor runtime (~1.3 KB) tracks pointer position using passive "
                    <code class="inline">"requestAnimationFrame"</code>
                    " loops and publishes high-precision reactive CSS variables onto elements: "
                    <code class="inline">"--yav-mouse-x"</code>
                    ", "
                    <code class="inline">"--yav-mouse-y"</code>
                    ", and normalized coordinates."
                </p>

                <div class="grid md:grid-cols-2 gap-6">
                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("box", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "Declarative Attributes"
                            </div>
                            <span class="text-[10px] text-oxide font-mono">"HTML5 Attrs"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// 3D Tilt plate with elastic physics"</div>
                            <div><span class="token-tag">"<div "</span><span class="token-attr">"yav:cursor="</span><span class="token-string">"\"tilt\" "</span><span class="token-attr">"yav:tilt-max="</span><span class="token-string">"\"12\" "</span><span class="token-attr">"class="</span><span class="token-string">"\"card\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<h3>"</span>"3D Perspective Card"<span class="token-tag">"</h3>"</span></div>
                            <div><span class="token-tag">"</div>"</span></div>
                            <div class="pt-2 token-comment">"// Magnetic button attraction to cursor"</div>
                            <div><span class="token-tag">"<button "</span><span class="token-attr">"yav:cursor="</span><span class="token-string">"\"magnetic\" "</span><span class="token-attr">"yav:magnetic-strength="</span><span class="token-string">"\"0.35\""</span><span class="token-tag">">"</span></div>
                            <div>"    \"Bouton Aimanté\""</div>
                            <div><span class="token-tag">"</button>"</span></div>
                        </div>
                    </div>

                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "Type-Safe Rust Components"
                            </div>
                            <span class="text-[10px] text-ok font-mono">"yav::ui::cursor"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Pre-built components from yav::ui"</div>
                            <div><span class="token-keyword">"use "</span>"yav::ui::{"<span class="token-type">"CursorSpotlight"</span>", "<span class="token-type">"TiltPlate"</span>", "<span class="token-type">"Magnetic"</span>"};"</div>
                            <div class="pt-1"><span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"    "<span class="token-tag">"<CursorSpotlight "</span><span class="token-attr">"radius_px="</span>"{"<span class="token-num">"450"</span>"}"<span class="token-tag">">"</span></div>
                            <div>"        "<span class="token-tag">"<TiltPlate "</span><span class="token-attr">"max_deg="</span>"{"<span class="token-num">"10.0"</span>"} "<span class="token-attr">"glare="</span><span class="token-bool">"true"</span><span class="token-tag">">"</span></div>
                            <div>"            "<span class="token-tag">"<Magnetic "</span><span class="token-attr">"strength="</span>"{"<span class="token-num">"0.4"</span>"}"<span class="token-tag">">"</span></div>
                            <div>"                "<span class="token-tag">"<Button>"</span>"Action"<span class="token-tag">"</Button>"</span></div>
                            <div>"            "<span class="token-tag">"</Magnetic>"</span></div>
                            <div>"        "<span class="token-tag">"</TiltPlate>"</span></div>
                            <div>"    "<span class="token-tag">"</CursorSpotlight>"</span></div>
                            <div>"}"</div>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 4.3 SPRING PHYSICS & MOTION ENGINE
            // =========================================================================
            <section id="motion-springs" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.3"</span>
                    <span class="t">"Spring Physics & WAAPI Motion Engine"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"solveSpring"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Mechanical mass, stiffness, and damping physics."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Motion in Yav is defined using physical harmonic oscillator equations ("
                    <code class="inline">"stiffness"</code>
                    ", "
                    <code class="inline">"damping"</code>
                    ", "
                    <code class="inline">"mass"</code>
                    ") solved into exact analytical sine/cosine curves and fed directly to "
                    <code class="inline">"Element.animate()"</code>
                    ". Initial states are pre-compiled into inline CSS during SSR for guaranteed zero-FOUC."
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("activity", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/MotionShowcase.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono">"< 1.8 KB WAAPI Runtime"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// 1. Declarative motion element with hover and tap recoils"</div>
                        <div><span class="token-tag">"<div "</span><span class="token-attr">"yav:initial="</span><span class="token-string">"\"opacity:0, y:24, scale:0.95\" "</span><span class="token-attr">"yav:animate="</span><span class="token-string">"\"opacity:1, y:0, scale:1\" "</span><span class="token-attr">"yav:spring="</span><span class="token-string">"\"320, 18, 1\" "</span><span class="token-attr">"yav:hover="</span><span class="token-string">"\"scale:1.02, y:-4\""</span><span class="token-tag">">"</span></div>
                        <div>"    \"Card content animated with spring physics\""</div>
                        <div><span class="token-tag">"</div>"</span></div>
                        <div class="pt-2 token-comment">"// 2. Specialized UI Motion Components"</div>
                        <div><span class="token-keyword">"use "</span>"yav::ui::{CountUp, AnimatedBar, TypewriterCode};"</div>
                        <div><span class="token-tag">"<CountUp "</span><span class="token-attr">"to="</span><span class="token-num">"{124500.0} "</span><span class="token-attr">"prefix="</span><span class="token-string">"\"+\" "</span><span class="token-attr">"suffix="</span><span class="token-string">"\" req/s\" "</span><span class="token-attr">"duration_ms="</span><span class="token-num">"{1400} "</span><span class="token-tag">"/>"</span></div>
                        <div><span class="token-tag">"<AnimatedBar "</span><span class="token-attr">"target_pct="</span><span class="token-num">"{97.4} "</span><span class="token-attr">"duration_ms="</span><span class="token-num">"{1200} "</span><span class="token-tag">"/>"</span></div>
                        <div><span class="token-tag">"<TypewriterCode "</span><span class="token-attr">"speed_ms="</span><span class="token-num">"{14} "</span><span class="token-attr">"skip_on_click="</span><span class="token-bool">"true"</span><span class="token-tag">">"</span></div>
                        <div>"    "<span class="token-tag">"<div>"</span>"\"pub fn main() { println!(\\\"Fast\\\"); }\""<span class="token-tag">"</div>"</span></div>
                        <div><span class="token-tag">"</TypewriterCode>"</span></div>
                    </div>
                    <div class="ide-status">
                        <span>"solveSpring(k, c, m) analytical harmonic oscillator solver"</span>
                        <span class="text-ok">"GPU Composited"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 4.4 DECLARATIVE REACTIVE SIGNALS
            // =========================================================================
            <section id="micro-signals" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.4"</span>
                    <span class="t">"Declarative Reactive Signals (yav:state)"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav:state"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Instant client calculators and dynamic gauges in markup."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Build interactive state islands, dynamic price calculators, live sliders, and gauge bars directly with declarative HTML5 directives without writing JavaScript functions."
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("sliders", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/CounterCalculator.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-oxide font-mono">"~0.9 KB signals runtime"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// Reactive local state island with two-way binding and inline style computation"</div>
                        <div><span class="token-tag">"<div "</span><span class="token-attr">"yav:state="</span><span class="token-string">"\"count: 1, unit_price: 49\" "</span><span class="token-attr">"class="</span><span class="token-string">"\"p-6 border rounded-xl\""</span><span class="token-tag">">"</span></div>
                        <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"flex items-center gap-4\""</span><span class="token-tag">">"</span></div>
                        <div>"        "<span class="token-tag">"<button "</span><span class="token-attr">"yav:on-click="</span><span class="token-string">"\"count = Math.max(1, count - 1)\""</span><span class="token-tag">">"</span>"\"-\""<span class="token-tag">"</button>"</span></div>
                        <div>"        "<span class="token-tag">"<span "</span><span class="token-attr">"yav:text="</span><span class="token-string">"\"count\" "</span><span class="token-attr">"class="</span><span class="token-string">"\"font-bold\""</span><span class="token-tag">">"</span>"\"1\""<span class="token-tag">"</span>"</span></div>
                        <div>"        "<span class="token-tag">"<button "</span><span class="token-attr">"yav:on-click="</span><span class="token-string">"\"count = count + 1\""</span><span class="token-tag">">"</span>"\"+\""<span class="token-tag">"</button>"</span></div>
                        <div>"    "<span class="token-tag">"</div>"</span></div>
                        <div>"    "<span class="token-tag">"<p "</span><span class="token-attr">"class="</span><span class="token-string">"\"mt-4\""</span><span class="token-tag">">"</span>"\"Total : \" "<span class="token-tag">"<span "</span><span class="token-attr">"yav:text="</span><span class="token-string">"\"'€' + (count * unit_price)\""</span><span class="token-tag">">"</span>"\"€49\""<span class="token-tag">"</span></p>"</span></div>
                        <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"track h-2 bg-zinc-800 rounded mt-4\""</span><span class="token-tag">">"</span></div>
                        <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"yav:style-width="</span><span class="token-string">"\"Math.min(100, count * 10) + '%'\" "</span><span class="token-attr">"class="</span><span class="token-string">"\"h-full bg-orange-500 rounded\""</span><span class="token-tag">" />"</span></div>
                        <div>"    "<span class="token-tag">"</div>"</span></div>
                        <div><span class="token-tag">"</div>"</span></div>
                    </div>
                    <div class="ide-status">
                        <span>"yav:state · yav:bind · yav:text · yav:on-click · yav:style-*"</span>
                        <span class="text-ok">"Reactive Signal Graph"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 4.5 MULTI-THEME DESIGN SYSTEM (8 THEMES)
            // =========================================================================
            <section id="theme-system" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.5"</span>
                    <span class="t">"Multi-Theme Design System (8 Themes)"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"ThemeSelector"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"8 built-in themes with cookie & localStorage persistence."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Yav v0.1.5 features a complete zero-dependency multi-theme engine equipped with 8 pre-calibrated themes, instant switching, and compile-time anti-FOUC injection ensuring zero flash of unstyled theme on page reload."
                </p>

                <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-4 text-xs font-mono">
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-oxide">
                        <div class="text-paper font-semibold">"Editorial Blueprint"</div>
                        <p class="text-mute text-[11px]">"Flagship carbon #0b0b0c & rust oxide #e0592f"</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-[#6366f1]">
                        <div class="text-paper font-semibold">"Obsidian Minimal"</div>
                        <p class="text-mute text-[11px]">"OLED black #09090b & electric indigo #6366f1"</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-[#c85a32]">
                        <div class="text-paper font-semibold">"Nordic Paper"</div>
                        <p class="text-mute text-[11px]">"Alabaster #f7f5f0 & terracotta #c85a32"</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-[#00ff66]">
                        <div class="text-paper font-semibold">"Cyber Terminal"</div>
                        <p class="text-mute text-[11px]">"Matrix green #00ff66 & phosphor amber #ffb000"</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-[#2aa198]">
                        <div class="text-paper font-semibold">"Solarized Lab"</div>
                        <p class="text-mute text-[11px]">"Parchment #fdf6e3 & precision cyan #2aa198"</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-[#8b5cf6]">
                        <div class="text-paper font-semibold">"Glass Aurora"</div>
                        <p class="text-mute text-[11px]">"Midnight #070913 & frosted violet #8b5cf6"</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-[#2563eb]">
                        <div class="text-paper font-semibold">"Classic Light"</div>
                        <p class="text-mute text-[11px]">"Pure white #ffffff & royal blue #2563eb"</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-[#38bdf8]">
                        <div class="text-paper font-semibold">"Slate Dark"</div>
                        <p class="text-mute text-[11px]">"Slate 900 #0f172a & sky 400 #38bdf8"</p>
                    </div>
                </div>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("moon", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/layouts/RootLayout.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono">"Zero-FOUC Guarantee"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// 1. Inject all themes CSS variables & anti-FOUC script in head"</div>
                        <div><span class="token-keyword">"use "</span>"yav::ui::{"<span class="token-type">"Theme"</span>", "<span class="token-type">"ThemeSelector"</span>"};"</div>
                        <div class="pt-1"><span class="token-tag">"<head>"</span></div>
                        <div>"    {"<span class="token-type">"Theme"</span>"::"<span class="token-fn">"inject_all_themes_css"</span>"()}"</div>
                        <div>"    "<span class="token-tag">"<link "</span><span class="token-attr">"rel="</span><span class="token-string">"\"stylesheet\" "</span><span class="token-attr">"href="</span><span class="token-string">"\"/public/styles.css\" "</span><span class="token-tag">"/>"</span></div>
                        <div><span class="token-tag">"</head>"</span></div>
                        <div class="pt-2 token-comment">"// 2. Insert ThemeSelector dropdown in navbar or header"</div>
                        <div><span class="token-tag">"<ThemeSelector "</span><span class="token-attr">"default_theme="</span><span class="token-string">"\"editorial-blueprint\" "</span><span class="token-tag">"/>"</span></div>
                    </div>
                    <div class="ide-status">
                        <span>"Persists in localStorage and document.cookie"</span>
                        <span class="text-ok">"Inline Anti-FOUC Script"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 4.6 SSR TREE-SHAKING RUNTIMES & WEBSOCKET LIVE SWAP
            // =========================================================================
            <section id="tree-shaking" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.6"</span>
                    <span class="t">"SSR Tree-Shaking Runtimes & WebSockets"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"0 KB JS Guarantee"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Only pay for what your page actually renders."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "During HTML serialization, Yav scans the rendered DOM string and tree-shakes client runtimes before transmitting the response. If a page uses no interactive attributes, exactly "
                    <strong class="text-paper">"0 bytes"</strong>
                    " of client JavaScript are injected. For real-time updates, "
                    <code class="inline">"WsBroadcast"</code>
                    " pushes server-rendered HTML fragments over WebSockets."
                </p>

                <div class="table-scroll pb-2">
                    <table class="dtable min-w-[580px]">
                        <thead>
                            <tr>
                                <th>"Detected Attributes on Page"</th>
                                <th>"Injected Micro-Runtime"</th>
                                <th>"Bundle Overhead"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="us">
                                <td class="text-paper font-semibold">"yav:toggle, yav:copy, yav:hotkey, yav:tab, yav:filter"</td>
                                <td class="text-oxide">"yav-micro-actions"</td>
                                <td>"~2.1 KB"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"yav:cursor, CursorSpotlight, TiltPlate, Magnetic"</td>
                                <td class="text-oxide">"yav-cursor-runtime"</td>
                                <td>"~1.3 KB"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"yav:animate, CountUp, AnimatedBar, TypewriterCode"</td>
                                <td class="text-oxide">"yav-motion-runtime"</td>
                                <td>"~1.8 KB"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"yav:state, yav:bind, yav:text, yav:on-click"</td>
                                <td class="text-oxide">"yav-signals-runtime"</td>
                                <td>"~0.9 KB"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-ok font-semibold">"None of the above (Static Page)"</td>
                                <td class="text-ok">"None"</td>
                                <td class="text-ok font-bold">"0 KB JS"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>

            // =========================================================================
            // 4.7 HTMX INTERACTIVITY & PARTIAL SWAPS
            // =========================================================================
            <section id="htmx-interactivity" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.7"</span>
                    <span class="t">"HTMX Integration & Partial HTML Swaps"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"HtmlFragment"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Rich dynamic UI without complex client-side SPA state."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "All standard HTMX attributes ("
                    <code class="inline">"hx-get"</code>
                    ", "
                    <code class="inline">"hx-post"</code>
                    ", "
                    <code class="inline">"hx-target"</code>
                    ", "
                    <code class="inline">"hx-swap"</code>
                    ", "
                    <code class="inline">"hx-trigger"</code>
                    ") are first-class attributes in RSX. Server handlers return lightweight "
                    <code class="inline">"HtmlFragment"</code>
                    " partials."
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/routes/api/counter.rs"
                            </div>
                        </div>
                        <button data-copy="use yav::prelude::*;\nuse yav::server::HtmlFragment;\n\npub async fn increment_counter() -> HtmlFragment {\n    let new_total = 42;\n    HtmlFragment::new(rsx! {\n        <div id=&quot;counter-box&quot; class=&quot;p-4 border rounded-xl flex items-center gap-4&quot;>\n            <span class=&quot;text-2xl font-bold&quot;>Total: {new_total}</span>\n            <button hx-post=&quot;/api/counter&quot; hx-target=&quot;#counter-box&quot; hx-swap=&quot;outerHTML&quot;>\n                +1 Increment\n            </button>\n        </div>\n    })\n    .hx_trigger(&quot;counterUpdated&quot;)\n    .hx_push_url(&quot;/dashboard?updated=true&quot;)\n}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"<br/>"13"<br/>"14"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"use "</span>"yav::prelude::*;"</div>
                            <div><span class="token-keyword">"use "</span>"yav::server::"<span class="token-type">"HtmlFragment"</span>";"</div>
                            <div class="pt-1"><span class="token-keyword">"pub async fn "</span><span class="token-fn">"increment_counter"</span>"() -> "<span class="token-type">"HtmlFragment"</span>" {"</div>
                            <div>"    "<span class="token-keyword">"let "</span>"new_total = "<span class="token-num">"42"</span>"; "<span class="token-comment">"// fetched from state or database"</span></div>
                            <div>"    "<span class="token-type">"HtmlFragment"</span>"::new("<span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"id="</span><span class="token-string">"\"counter-box\" "</span><span class="token-attr">"class="</span><span class="token-string">"\"p-4 border rounded-xl flex items-center gap-4\""</span><span class="token-tag">">"</span></div>
                            <div>"            "<span class="token-tag">"<span "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-2xl font-bold\""</span><span class="token-tag">">"</span>"Total: {new_total}"<span class="token-tag">"</span>"</span></div>
                            <div>"            "<span class="token-tag">"<button "</span><span class="token-attr">"hx-post="</span><span class="token-string">"\"/api/counter\" "</span><span class="token-attr">"hx-target="</span><span class="token-string">"\"#counter-box\" "</span><span class="token-attr">"hx-swap="</span><span class="token-string">"\"outerHTML\""</span><span class="token-tag">">"</span></div>
                            <div>"                +1 Increment"</div>
                            <div>"            "<span class="token-tag">"</button>"</span></div>
                            <div>"        "<span class="token-tag">"</div>"</span></div>
                            <div>"    })"</div>
                            <div>"    .hx_trigger("<span class="token-string">"\"counterUpdated\""</span>")"</div>
                            <div>"    .hx_push_url("<span class="token-string">"\"/dashboard?updated=true\""</span>")"</div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"HtmlFragment · Lightweight swap without <!DOCTYPE html>"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"0 kB Client JS"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 4.8 13,000+ ICON SYSTEM
            // =========================================================================
            <section id="icons-catalog" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.8"</span>
                    <span class="t">"Built-in 13,000+ Icon System"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav-icons"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Embedded SVGs. Zero external webfont requests."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Yav incorporates over 13,000 pre-optimized SVG icons organized across 3 aesthetic families: <strong class=\"text-paper\">Classic</strong> (Lucide), <strong class=\"text-paper\">Minimal</strong> (MynaUI), and <strong class=\"text-paper\">System</strong> (Phosphor), embedded directly in the binary with zero runtime HTTP requests."
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("star", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/icons_usage.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-oxide font-mono">"yav-icons"</span>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"use "</span>"yav::icons::{"<span class="token-fn">"render_icon"</span>", "<span class="token-type">"IconSize"</span>"};"</div>
                            <div class="pt-1 token-comment">"// 1. Functional render helper with size and class overrides"</div>
                            <div><span class="token-keyword">"let "</span>"search = render_icon("<span class="token-string">"\"search\""</span>", "<span class="token-type">"IconSize"</span>"::Md, Some("<span class="token-string">"\"text-oxide\""</span>"), None);"</div>
                            <div class="pt-1 token-comment">"// 2. Accessible icon-only button with aria-label"</div>
                            <div><span class="token-keyword">"let "</span>"close = render_icon("<span class="token-string">"\"x\""</span>", "<span class="token-type">"IconSize"</span>"::Sm, Some("<span class="token-string">"\"text-paper\""</span>"), Some("<span class="token-string">"\"Close Dialog\""</span>"));"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"Lucide + MynaUI + Phosphor embedded SVGs"</span>
                        <span>"0 External Network Requests"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 4.9 UI SCAFFOLDING (YAV ADD)
            // =========================================================================
            <section id="ui-scaffolding" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"4.9"</span>
                    <span class="t">"UI Component Scaffolding (yav add - 60+ Primitives)"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"60+ primitives"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">"Copy-paste ownership. No opaque UI black-boxes."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    "Similar to shadcn/ui, "
                    <code class="inline">"yav add <component>"</code>
                    " copies production-tested, beautifully-styled component source files directly into your project's "
                    <code class="inline">"src/components/"</code>
                    " directory. Over 60 enterprise primitives (Charts, DataGrid, Kanban, FlowCanvas, DevTools, Spreadsheets, 3D Mesh, Media Player, and more) are available for instant scaffolding."
                </p>

                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-1.5">
                                <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                            </div>
                            <span class="text-faint text-[11px] ml-2">"zsh — yav add UI suite"</span>
                        </div>
                        <button data-copy="yav add charts data-grid kanban-board flow-canvas devtools spreadsheet model-viewer-3d" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#070709]">
                        <div class="token-comment">"# Add enterprise interactive widgets (Charts, DataGrid, Kanban, FlowCanvas, DevTools)"</div>
                        <div><span class="text-oxide">"$ "</span>"yav add charts data-grid kanban-board flow-canvas devtools spreadsheet model-viewer-3d"</div>
                        <div class="token-comment pt-2">"# Add authentication card with input validation and submit state"</div>
                        <div><span class="text-oxide">"$ "</span>"yav add auth-card"</div>
                        <div class="token-comment pt-2">"# Add multi-theme switcher with 8 themes and anti-FOUC script"</div>
                        <div><span class="text-oxide">"$ "</span>"yav add theme-selector"</div>
                        <div class="token-comment pt-2">"# Add 3D cursor spotlight and magnetic hover plates"</div>
                        <div><span class="text-oxide">"$ "</span>"yav add cursor-spotlight motion"</div>
                        <div class="token-comment pt-2">"# Add command palette, searchable tables, sliding drawers, and KPI cards"</div>
                        <div><span class="text-oxide">"$ "</span>"yav add command-menu data-table drawer stat-card carousel stepper"</div>
                    </div>
                </div>
            </section>
        </div>
    }
}
