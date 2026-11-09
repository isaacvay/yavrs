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
                        <span class="text-paper font-semibold">"components"</span>
                        <span class="text-faint">"/"</span>
                        <span class="text-oxide">"ui-catalog"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <button onclick="copyText(this, 'yav add auth-card data-table command-menu')" class="btn btn-sm btn-line text-[11px] font-mono flex items-center gap-2">
                            <span class="text-oxide">"$"</span>
                            <span>"yav add <component>"</span>
                            <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-2">"copy"</span>
                        </button>
                        <div class="hidden sm:flex items-center gap-2 px-2.5 py-1 rounded bg-sunk border border-rule text-dim text-[11px]">
                            <span class="dot dot-ok"></span>
                            <span class="text-paper font-semibold">"60+ primitives"</span>
                            <span class="text-dim">"· 13,000+ icons"</span>
                        </div>
                    </div>
                </div>
            </div>

            // Main Catalog Showcase & Sticky Side Navigation (Direct Workspace, No Hero)
            <div class="wrap py-8 sm:py-10">
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
                                    <span class="text-faint text-[10px] uppercase">"Previous Section"</span>
                                    <div class="text-paper group-hover:text-oxide font-medium flex items-center gap-2">
                                        {render_icon("arrow-left", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                        "04 · Verified Benchmarks"
                                    </div>
                                </div>
                                <span class="text-dim text-right text-[11px]">"124k req/s"</span>
                            </a>

                            <a href="/docs" class="plate p-5 bg-ink-2 hover:border-oxide transition-colors group flex items-center justify-between">
                                <div class="space-y-1">
                                    <span class="text-faint text-[10px] uppercase">"Next Section"</span>
                                    <div class="text-paper group-hover:text-oxide font-medium flex items-center gap-2">
                                        "06 · Engineering Documentation"
                                        {render_icon("arrow-right", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                    </div>
                                </div>
                                <span class="text-dim text-right text-[11px]">"Manual"</span>
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
