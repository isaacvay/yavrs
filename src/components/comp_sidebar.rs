//! Sidebar navigation for Components Showroom with independent scrolling and live search filter.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentSidebar() -> Node {
    rsx! {
        <aside class="hidden lg:block lg:col-span-3 sticky top-20 h-[calc(100vh-5.5rem)] select-none">
            <nav class="toc sidebar-scroll h-full overflow-y-auto overscroll-contain pr-3 space-y-6 text-xs" aria-label="Component catalog navigation">
                
                // Live filter input for component navigation
                <div class="relative pb-2">
                    <input 
                        id="comp-search-input" 
                        type="text" 
                        placeholder="Filter components..." 
                        oninput="filterCompNav(this.value)"
                        class="w-full bg-sunk border border-rule rounded px-3 py-1.5 text-[12px] font-mono text-paper placeholder:text-dim outline-none focus:border-oxide transition-colors"
                    />
                    <span class="absolute right-2.5 top-2 text-[10px] text-faint font-mono">"ESC"</span>
                </div>

                // Group 1: Actions & Controls
                <div class="comp-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "1. Actions & Controls"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#buttons" class="comp-nav-item">"5.1 Buttons & Actions"</a>
                        <a href="#badges" class="comp-nav-item">"5.2 Status Chips & Badges"</a>
                        <a href="#forms" class="comp-nav-item">"5.3 Form Controls & Inputs"</a>
                    </div>
                </div>

                // Group 2: Data & Structure
                <div class="comp-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("activity", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "2. Data & Structure"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#statcards" class="comp-nav-item">"5.4 KPI & StatCards"</a>
                        <a href="#datatable" class="comp-nav-item">"5.5 Radix Data Table"</a>
                        <a href="#authcard" class="comp-nav-item">"5.6 Auth Card & Login"</a>
                    </div>
                </div>

                // Group 3: Feedback & Overlays
                <div class="comp-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("layers", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "3. Feedback & Overlays"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#feedback" class="comp-nav-item">"5.7 Alerts & Dialogs"</a>
                        <a href="#navigation" class="comp-nav-item">"5.8 Navigation & Tabs"</a>
                        <a href="#commandmenu" class="comp-nav-item">"5.9 Command Menu (⌘K)"</a>
                    </div>
                </div>

                // Group 4: Primitives & Physics
                <div class="comp-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("box", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "4. Motion, Cursor & Themes"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#icons" class="comp-nav-item">"5.10 Vector Icon Explorer"</a>
                        <a href="#motion" class="comp-nav-item">"5.11 Spring Physics Dynamics"</a>
                        <a href="#themes" class="comp-nav-item">"5.12 Multi-Theme (8 Themes)"</a>
                        <a href="#cursor" class="comp-nav-item">"5.13 Cursor & 3D Tilt"</a>
                        <a href="#microactions" class="comp-nav-item">"5.14 Micro-Actions & Signals"</a>
                    </div>
                </div>

                // Group 5: Overlays & Technical
                <div class="comp-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "5. Overlays & Technical"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#overlays" class="comp-nav-item">"5.15 Drawers & Popovers"</a>
                        <a href="#codeblock" class="comp-nav-item">"5.16 CodeBlock & Steppers"</a>
                    </div>
                </div>

                // Group 6: Enterprise Widgets & Charts
                <div class="comp-nav-group space-y-2">
                    <div class="label text-[11px] font-mono uppercase tracking-wider text-oxide font-semibold flex items-center gap-1.5">
                        {render_icon("activity", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        "6. Enterprise & Visual"
                    </div>
                    <div class="space-y-0.5">
                        <a href="#charts" class="comp-nav-item">"5.17 Enterprise SVG Charts"</a>
                        <a href="#enterprise" class="comp-nav-item">"5.18 DataGrid, Kanban & Flow"</a>
                        <a href="#visual-widgets" class="comp-nav-item">"5.19 3D Mesh & Widgets"</a>
                    </div>
                </div>

                // Runtime guarantee card
                <div class="plate bg-sunk p-4 text-[12px] font-mono text-dim space-y-2 border border-rule">
                    <span class="text-paper flex items-center gap-2">
                        <span class="dot dot-ok"></span>
                        "0 KB Client Runtime"
                    </span>
                    <p class="text-[11px] leading-relaxed">
                        "Every component compiles directly into native Rust RSX functions. No JavaScript bundle, instant First Contentful Paint."
                    </p>
                    <div class="pt-2 border-t border-[rgba(236,233,227,0.08)] flex justify-between text-[10px] text-faint">
                        <span>"60+ Primitives"</span>
                        <span class="text-oxide">"13,000+ Icons"</span>
                    </div>
                </div>
            </nav>
        </aside>
    }
}
