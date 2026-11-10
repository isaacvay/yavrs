//! Sticky "On this page" TOC navigation for documentation.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn DocsToc() -> Node {
    rsx! {
        <aside class="hidden xl:block xl:col-span-2 select-none">
            <div class="sticky top-28 space-y-4 text-xs font-mono">
                <div class="text-paper font-semibold uppercase tracking-wider text-[11px] flex items-center gap-1.5 pb-1 border-b border-rule">
                    {render_icon("file-text", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                    <span data-i18n="docs.toc.title">"On this page"</span>
                </div>
                <nav class="space-y-1 text-mute text-[11.5px]" aria-label="On this page navigation" data-i18n-aria-label="docs.toc.aria">
                    <a href="#installation" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.prereq">"Prerequisites"</a>
                    <a href="#quickstart" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.quickstart">"Quickstart"</a>
                    <a href="#manifest-reference" class="block hover:text-paper transition-colors py-0.5">"package.yavml"</a>
                    <a href="#cli-toolbelt" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.cli">"CLI Toolbelt (20 cmds)"</a>
                    <a href="#rsx-syntax" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.rsx">"RSX Syntax & Loops"</a>
                    <a href="#component-patterns" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.props">"Component Props"</a>
                    <a href="#static-folding" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.folding">"Static VDOM Folding"</a>
                    <a href="#native-control-flow" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.flow">"Native if & for Loops"</a>
                    <a href="#filesystem-routing" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.radix">"Radix Routing"</a>
                    <a href="#server-architecture" class="block hover:text-paper transition-colors py-0.5">"Axum & Tokio"</a>
                    <a href="#security-sessions" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.sessions">"HMAC Sessions"</a>
                    <a href="#ssr-caching" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.caching">"In-Memory Caching"</a>
                    <a href="#view-transitions" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.vt">"View Transitions"</a>
                    <a href="#ai-mcp" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.mcp">"AI SDK & MCP Server"</a>
                    <a href="#resilience" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.resilience">"Resilience & XFetch"</a>
                    <a href="#passkeys-rebac" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.passkeys">"Passkeys & ReBAC"</a>
                    <a href="#modern-protocols" class="block hover:text-paper transition-colors py-0.5">"GraphQL, Turbo & Tus"</a>
                    <a href="#zero-js-microactions" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.micro">"Zero-JS Micro-Actions"</a>
                    <a href="#cursor-tracking" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.cursor">"Cursor & 3D Tilt"</a>
                    <a href="#motion-springs" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.spring">"Spring Motion"</a>
                    <a href="#micro-signals" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.signals">"Reactive Signals"</a>
                    <a href="#theme-system" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.themes">"8 Design Themes"</a>
                    <a href="#tree-shaking" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.shake">"SSR Tree-Shaking"</a>
                    <a href="#htmx-interactivity" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.htmx">"HTMX Partials"</a>
                    <a href="#icons-catalog" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.icons">"13,000+ Icons"</a>
                    <a href="#ui-scaffolding" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.ui">"yav add (60+ UI)"</a>
                    <a href="#asset-pipeline" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.brotli">"Brotli Pipeline"</a>
                    <a href="#ssg-export" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.ssg">"SSG Static Export"</a>
                    <a href="#docker-deployment" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.docker">"Docker & Multi-Cloud"</a>
                    <a href="#developer-diagnostics" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.diag">"Diagnostics & Routes"</a>
                    <a href="#database-migrations" class="block hover:text-paper transition-colors py-0.5" data-i18n="docs.toc.db">"DB Migrations (yav db)"</a>
                </nav>

                <div class="pt-6 border-t border-rule space-y-2">
                    <a href="#top" class="text-dim hover:text-paper transition-colors text-[11px] flex items-center gap-1.5">
                        {render_icon("arrow-up", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        <span data-i18n="docs.toc.top">"Back to top"</span>
                    </a>
                    <a href="https://github.com/isaacvay/yav.rs/issues/new" target="_blank" rel="noopener noreferrer" class="text-dim hover:text-paper transition-colors text-[11px] flex items-center gap-1.5">
                        {render_icon("edit", IconSize::Sm, Some("w-3.5 h-3.5 text-dim"), None)}
                        <span data-i18n="docs.toc.edit">"Suggest edit"</span>
                    </a>
                </div>
            </div>
        </aside>
    }
}
