//! UI Component Gallery & Icon Catalog — Editorial Blueprint.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;
use crate::components::*;

pub async fn components(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            // Sticky sub-navigation breadcrumbs bar
            <div class="border-b border-rule bg-ink/90 backdrop-blur-md sticky top-14 z-20">
                <div class="wrap py-3 flex flex-wrap items-center justify-between gap-4 text-xs font-mono">
                    <div class="flex items-center gap-2 text-dim">
                        <a href="/" class="hover:text-paper transition-colors flex items-center gap-1.5 text-mute">
                            {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            <span>"yav"</span>
                        </a>
                        <span class="text-faint">"/"</span>
                        <span class="text-paper font-semibold" data-i18n="comp.crumb.main">"components"</span>
                        <span class="text-faint">"/"</span>
                        <span class="text-oxide" data-i18n="comp.crumb.sub">"ui-catalog"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <button onclick="copyText(this, 'yav add auth-card data-table command-menu')" class="btn btn-sm btn-line text-[11px] font-mono flex items-center gap-2">
                            <span class="text-oxide">"$"</span>
                            <span>"yav add <component>"</span>
                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-2">"copy"</span>
                        </button>
                        <div class="hidden sm:flex items-center gap-2 px-2.5 py-1 rounded bg-sunk border border-rule text-dim text-[11px]">
                            <span class="dot dot-ok"></span>
                            <span class="text-paper font-semibold" data-i18n="comp.pill.prims">"60+ primitives"</span>
                            <span class="text-dim" data-i18n="comp.pill.icons">"· 13,000+ icons"</span>
                        </div>
                    </div>
                </div>
            </div>

            // Main Catalog Showcase & Sticky Side Navigation (Direct Workspace, No Hero)
            <div class="wrap py-8 sm:py-10">
                // Mobile & Tablet Quick Jump Navigation Pill Bar
                <div class="lg:hidden pb-6">
                    <div class="label pb-2 text-[10.5px] text-dim font-mono flex items-center justify-between">
                        <span data-i18n="comp.mobile.jump">"Jump to primitive:"</span>
                        <span class="text-oxide" data-i18n="comp.mobile.cats">"19 categories"</span>
                    </div>
                    <div class="flex items-center gap-2 overflow-x-auto pb-2 table-scroll text-xs font-mono">
                        <a href="#buttons" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_1">"5.1 Buttons"</a>
                        <a href="#badges" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_2">"5.2 Badges"</a>
                        <a href="#forms" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_3">"5.3 Forms"</a>
                        <a href="#statcards" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_4">"5.4 KPI Cards"</a>
                        <a href="#datatable" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_5">"5.5 Data Table"</a>
                        <a href="#authcard" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_6">"5.6 Auth Card"</a>
                        <a href="#feedback" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_7">"5.7 Alerts"</a>
                        <a href="#navigation" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_8">"5.8 Navigation"</a>
                        <a href="#commandmenu" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_9">"5.9 Command Menu"</a>
                        <a href="#icons" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_10">"5.10 Vector Icons"</a>
                        <a href="#motion" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_11">"5.11 Motion"</a>
                        <a href="#themes" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_12">"5.12 Themes"</a>
                        <a href="#cursor" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_13">"5.13 Cursor & 3D"</a>
                        <a href="#microactions" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_14">"5.14 Micro-Actions"</a>
                        <a href="#overlays" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_15">"5.15 Drawers"</a>
                        <a href="#codeblock" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_16">"5.16 CodeBlock"</a>
                        <a href="#charts" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_17">"5.17 Charts"</a>
                        <a href="#enterprise" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_18">"5.18 Enterprise"</a>
                        <a href="#visual-widgets" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0" data-i18n="comp.pill.5_19">"5.19 3D & Widgets"</a>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 xl:gap-12 items-start">
                    // Sticky independently scrollable sidebar
                    <ComponentSidebar />

                    // Main Component Showcase Articles
                    <article class="lg:col-span-9 space-y-20 min-w-0">
                        <ComponentButtons />
                        <ComponentBadges />
                        <ComponentForms />
                        <ComponentStatCards />
                        <ComponentDataTable />
                        <ComponentAuthCard />
                        <ComponentFeedback />
                        <ComponentNavigation />
                        <ComponentCommandMenu />
                        <ComponentIcons />
                        <ComponentMotion />
                        <ComponentThemes />
                        <ComponentCursor />
                        <ComponentMicroActions />
                        <ComponentOverlays />
                        <ComponentCodeBlock />
                        <ComponentCharts />
                        <ComponentEnterprise />
                        <ComponentVisualWidgets />

                        // Bottom Navigation Cards
                        <div class="pt-12 border-t border-[rgba(236,233,227,0.08)] grid sm:grid-cols-2 gap-4 font-mono text-xs">
                            <a href="/benchmarks" class="plate p-5 bg-ink-2 hover:border-oxide transition-colors group flex items-center justify-between">
                                <div class="space-y-1">
                                    <span class="text-faint text-[10px] uppercase" data-i18n="comp.nav.prev_k">"Previous Section"</span>
                                    <div class="text-paper group-hover:text-oxide font-medium flex items-center gap-2">
                                        {render_icon("arrow-left", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                        <span data-i18n="comp.nav.prev_t">"04 · Verified Benchmarks"</span>
                                    </div>
                                </div>
                                <span class="text-dim text-right text-[11px]">"124k req/s"</span>
                            </a>

                            <a href="/docs" class="plate p-5 bg-ink-2 hover:border-oxide transition-colors group flex items-center justify-between">
                                <div class="space-y-1">
                                    <span class="text-faint text-[10px] uppercase" data-i18n="comp.nav.next_k">"Next Section"</span>
                                    <div class="text-paper group-hover:text-oxide font-medium flex items-center gap-2">
                                        <span data-i18n="comp.nav.next_t">"06 · Engineering Documentation"</span>
                                        {render_icon("arrow-right", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                    </div>
                                </div>
                                <span class="text-dim text-right text-[11px]" data-i18n="comp.nav.next_badge">"Manual"</span>
                            </a>
                        </div>
                    </article>
                </div>
            </div>

            // Client helper script for navigation filter
            <script>
                {r#"
                function filterCompNav(query) {
                    var q = (query || '').toLowerCase().trim();
                    var items = document.querySelectorAll('.comp-nav-item');
                    items.forEach(function(item) {
                        var text = item.textContent.toLowerCase();
                        if (!q || text.indexOf(q) !== -1) {
                            item.style.display = '';
                        } else {
                            item.style.display = 'none';
                        }
                    });
                }
                "#}
            </script>
        </AppLayout>
    };
    website_page(
        "Components & Icon Catalog — Yav.rs",
        "Inspect 60+ interactive UI primitives, zero-JS micro-actions, cursor engines, and 13,000+ vector icons optimized for zero-overhead Rust SSR and WASM applications.",
        "/components",
        page,
    )
}
