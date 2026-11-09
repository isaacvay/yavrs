//! Documentation — Editorial Blueprint & Complete Engineering Reference for Yav.rs.
//! Industry-standard docs shell: Direct docs access (no hero banner), independent sticky sidebar,
//! search filter, breadcrumbs, IDE & Terminal styled blocks, on-this-page TOC.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;
use crate::components::*;

pub async fn docs(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            // Standard Docs Sub-Header / Breadcrumb Bar (Sticky under site header)
            <div class="border-b border-rule bg-ink/90 backdrop-blur-md sticky top-14 z-20">
                <div class="wrap py-3 flex flex-wrap items-center justify-between gap-4 text-xs font-mono">
                    <div class="flex items-center gap-2 text-dim">
                        <a href="/" class="hover:text-paper transition-colors flex items-center gap-1.5 text-mute">
                            {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            <span>"yav"</span>
                        </a>
                        <span class="text-faint">"/"</span>
                        <span class="text-paper font-semibold">"docs"</span>
                        <span class="text-faint">"/"</span>
                        <span class="text-oxide">"engineering-manual"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="flex items-center gap-2 px-2.5 py-1 rounded bg-sunk border border-rule text-dim text-[11px]">
                            <span class="dot dot-ok"></span>
                            <span class="text-paper font-semibold">"v0.1.5"</span>
                            <span class="text-dim">"· stable"</span>
                        </div>
                        <a href="https://github.com/isaacvay/yav.rs" target="_blank" rel="noopener noreferrer" class="hidden sm:inline-flex items-center gap-1.5 text-mute hover:text-paper transition-colors text-[11.5px]">
                            {render_icon("external-link", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            <span>"GitHub"</span>
                        </a>
                    </div>
                </div>
            </div>

            // Standard Documentation Workspace (Sidebar + Content + On-Page TOC)
            <div class="wrap py-8 sm:py-10">
                // Mobile & Tablet Quick Docs Section Pill Bar
                <div class="lg:hidden pb-6">
                    <div class="label pb-2 text-[10.5px] text-dim font-mono flex items-center justify-between">
                        <span>"Documentation sections:"</span>
                        <span class="text-oxide">"5 core chapters"</span>
                    </div>
                    <div class="flex items-center gap-2 overflow-x-auto pb-2 table-scroll text-xs font-mono">
                        <a href="#installation" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0">"1. Getting Started"</a>
                        <a href="#rsx-syntax" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0">"2. Components & RSX"</a>
                        <a href="#filesystem-routing" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0">"3. Routing & Server"</a>
                        <a href="#view-transitions" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0">"4. Interactivity & AI"</a>
                        <a href="#asset-pipeline" class="px-3 py-1.5 rounded plate bg-ink-2 hover:border-oxide text-paper shrink-0">"5. Production & Deploy"</a>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 xl:gap-12 items-start">
                    
                    // Left Sticky Navigation Sidebar (col-span-3, independently scrollable)
                    <DocsSidebar />

                    // Center Documentation Content (col-span-9 on lg, col-span-7 on xl)
                    <article class="lg:col-span-9 xl:col-span-7 prose-doc space-y-20 min-w-0">
                        <DocsGettingStarted />
                        <DocsComponents />
                        <DocsServer />
                        <DocsInteractivity />
                        <DocsProduction />
                    </article>

                    // Right Sticky "On this page" Table of Contents (xl:col-span-2)
                    <DocsToc />
                </div>
            </div>

            // Script for client-side search filtering of docs sidebar
            {Node::raw(r#"<script>
            function filterDocNav(query) {
                var q = query.toLowerCase().trim();
                var groups = document.querySelectorAll('.doc-nav-group');
                groups.forEach(function(group) {
                    var items = group.querySelectorAll('.doc-nav-item');
                    var visibleCount = 0;
                    items.forEach(function(item) {
                        var text = item.textContent.toLowerCase();
                        var match = !q || text.indexOf(q) !== -1;
                        item.style.display = match ? 'block' : 'none';
                        if (match) visibleCount++;
                    });
                    group.style.display = visibleCount > 0 ? 'block' : 'none';
                });
            }
            document.addEventListener('keydown', function(e) {
                if (e.key === 'Escape') {
                    var inp = document.getElementById('doc-search-input');
                    if (inp && inp === document.activeElement) {
                        inp.value = '';
                        filterDocNav('');
                        inp.blur();
                    }
                }
            });
            </script>"#)}
        </AppLayout>
    };

    website_page(
        "Documentation & Engineering Manual — Yav.rs",
        "Complete engineering guide, API reference, routing, component model, forms, SSR, and deployment manual for Yav.",
        "/docs",
        page,
    )
}
