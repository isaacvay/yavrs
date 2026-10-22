//! Modern application layout with precision engineering navbar, command palette, i18n switcher, and footer.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use crate::components::*;
use crate::i18n::get_i18n_json;

#[derive(Clone, Default)]
pub struct AppLayoutProps {
    pub children: Vec<Node>,
}

#[component]
pub fn AppLayout(props: AppLayoutProps) -> Node {
    let anti_fouc = anti_fouc_script();
    let i18n_payload = get_i18n_json();

    rsx! {
        <div class="min-h-screen bg-white dark:bg-[#050507] text-zinc-900 dark:text-zinc-100 antialiased selection:bg-amber-500 selection:text-white flex flex-col justify-between font-sans relative bg-mesh-grid">
            {anti_fouc}
            <ScrollProgressBar class="fixed top-0 left-0 right-0 h-[2.5px] bg-gradient-to-r from-amber-500 via-orange-500 to-amber-400 z-50 origin-left" />
            <script id="yav-i18n-payload" type="application/json">
                {i18n_payload}
            </script>
            <CommandPalette />

            // Top Announcement Bar
            <div class="w-full bg-zinc-950 text-zinc-200 text-[11px] font-mono py-1.5 px-4 border-b border-zinc-800/80 flex items-center justify-center gap-2">
                <span class="inline-flex items-center gap-1.5 font-semibold text-amber-400">
                    <span class="w-1.5 h-1.5 rounded-full bg-amber-400 animate-pulse"></span>
                    <span data-i18n="banner.released">"v0.1.3 RELEASED"</span>
                </span>
                <span class="text-zinc-600">"|"</span>
                <span class="hidden sm:inline text-zinc-400" data-i18n="banner.text">"Production-grade Axum 0.8, RSX compile-time rendering, zero JS hydration."</span>
                <a href="/docs" class="text-amber-400 hover:text-amber-300 underline underline-offset-2 flex items-center gap-0.5 ml-1">
                    <span data-i18n="banner.action">"Get Started"</span>
                    {render_icon("arrow-right", IconSize::Sm, Some("w-3 h-3"), None)}
                </a>
            </div>

            // Sticky High-Precision Header (Next.js & Shuttle Frosted Glass)
            <header class="sticky top-0 z-40 w-full border-b border-zinc-200/80 bg-white/80 dark:border-zinc-800/80 dark:bg-[#050507]/80 backdrop-blur-xl transition-colors">
                <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-8">
                    // Left: Logo & Main Navigation
                    <div class="flex items-center gap-8">
                        <a href="/" class="flex items-center gap-2.5 hover:opacity-90 transition-opacity group">
                            <img src="/public/favicon.svg" class="w-8 h-8 rounded-lg group-hover:scale-105 transition-transform" alt="Yav logo" />
                            <div class="flex items-baseline gap-1.5">
                                <span class="font-bold tracking-tight text-zinc-900 dark:text-white text-lg">"yav"</span>
                                <span class="font-mono text-zinc-400 dark:text-zinc-500 text-xs">".rs"</span>
                            </div>
                            <span class="rounded-full bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 px-2 py-0.5 text-[10px] font-mono font-medium border border-zinc-200 dark:border-zinc-700/80">
                                "v0.1.3"
                            </span>
                        </a>

                        <nav class="hidden md:flex items-center gap-6 text-sm text-zinc-600 dark:text-zinc-400 font-medium">
                            <a href="/" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.overview">
                                "Overview"
                            </a>
                            <a href="/about" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.architecture">
                                "Architecture"
                            </a>
                            <a href="/docs" class="hover:text-zinc-900 dark:hover:text-white transition-colors flex items-center gap-1">
                                <span data-i18n="nav.docs">"Docs"</span>
                            </a>
                            <a href="/benchmarks" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.benchmarks">
                                "Benchmarks"
                            </a>
                            <a href="/components" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.components">
                                "Components"
                            </a>
                        </nav>
                    </div>

                    // Right: Actions (Search, Quick Copy, Language Switcher, Theme, GitHub)
                    <div class="flex items-center gap-3">
                        // Search Button (Trigger ⌘K)
                        <button
                            onclick="openCmdPalette()"
                            class="hidden lg:flex items-center gap-3 px-3 py-1.5 rounded-lg bg-zinc-100 hover:bg-zinc-200/70 dark:bg-zinc-800/60 dark:hover:bg-zinc-800 text-xs text-zinc-500 dark:text-zinc-400 border border-zinc-200/80 dark:border-zinc-700/60 transition-all cursor-pointer shadow-xs"
                            title="Open Search Palette (⌘K)"
                        >
                            <span class="flex items-center gap-2">
                                {render_icon("search", IconSize::Sm, Some("w-3.5 h-3.5 text-zinc-400"), None)}
                                <span data-i18n="nav.search">"Quick search..."</span>
                            </span>
                            <kbd class="flex items-center gap-0.5 px-1.5 py-0.5 rounded text-[10px] font-mono bg-white dark:bg-zinc-900 text-zinc-500 dark:text-zinc-400 border border-zinc-200 dark:border-zinc-700">
                                "⌘K"
                            </kbd>
                        </button>

                        // Quick Copy `cargo add yav`
                        <button
                            onclick="navigator.clipboard.writeText('cargo add yav'); const el=this.querySelector('.copy-txt'); el.innerText='copied!'; setTimeout(()=>el.innerText='cargo add yav', 1500);"
                            class="hidden sm:inline-flex items-center gap-2 px-3 py-1.5 rounded-lg bg-zinc-100 hover:bg-zinc-200/80 dark:bg-zinc-800/80 dark:hover:bg-zinc-700/80 text-xs font-mono text-zinc-700 dark:text-zinc-300 transition-all cursor-pointer border border-zinc-200 dark:border-zinc-700 shadow-xs active:scale-95"
                            title="Copy dependency command"
                        >
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                            <span class="copy-txt">"cargo add yav"</span>
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 opacity-60 ml-0.5"), None)}
                        </button>

                        // Language Switcher (EN / FR / ES / DE)
                        <LangSwitcher />

                        // GitHub Link
                        <a
                            href="https://github.com/isaacvay/yav.rs"
                            target="_blank"
                            rel="noreferrer"
                            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-100 text-xs font-medium transition-all shadow-xs active:scale-95"
                        >
                            {render_icon("minimal:github", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                            <span class="hidden sm:inline font-mono">"GitHub"</span>
                            <span class="text-[10px] font-mono opacity-80 border-l border-zinc-700 dark:border-zinc-300 pl-1.5 ml-0.5">"★"</span>
                        </a>

                        // Theme Toggle
                        <div class="border-l border-zinc-200 dark:border-zinc-800 pl-1 sm:pl-2">
                            <ThemeToggle />
                        </div>
                    </div>
                </div>
            </header>

            // Main Content Area
            <main class="flex-grow w-full">
                {props.children}
            </main>

            // High-Craft Engineering Footer (Next.js & Shuttle Style)
            <footer class="border-t border-zinc-200 dark:border-zinc-800/80 bg-zinc-50/80 dark:bg-[#030305] py-14 px-4 sm:px-8 text-xs text-zinc-500 dark:text-zinc-400 mt-20 transition-colors">
                <div class="mx-auto max-w-7xl">
                    <div class="grid grid-cols-1 md:grid-cols-5 gap-10 pb-12 border-b border-zinc-200 dark:border-zinc-800/60">
                        // Col 1: Brand & Philosophy (Span 2)
                        <div class="md:col-span-2 space-y-4">
                            <div class="flex items-center gap-2.5">
                                <img src="/public/favicon.svg" class="w-6 h-6" alt="Yav logo" />
                                <span class="font-bold text-zinc-900 dark:text-white tracking-tight text-sm">"yav.rs"</span>
                                <span class="text-[11px] font-mono text-zinc-400">"v0.1.3"</span>
                            </div>
                            <p class="text-xs text-zinc-600 dark:text-zinc-400 max-w-sm leading-relaxed font-sans" data-i18n="footer.brand.desc">
                                "The High-Velocity Full-Stack Web Framework for Rust. Built on Axum 0.8, Hyper 1.x, Tokio, and compiled RSX templates. Designed for extreme performance, type safety, and zero client JS hydration overhead."
                            </p>
                            <div class="flex items-center gap-3 pt-1">
                                <div class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-200 dark:border-emerald-800/60 text-[10px] font-mono text-emerald-700 dark:text-emerald-400">
                                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                                    <span data-i18n="footer.status">"All Systems Operational"</span>
                                </div>
                                <span class="text-[11px] font-mono text-zinc-400">"0.32ms p99"</span>
                            </div>
                        </div>

                        // Col 2: Framework
                        <div class="space-y-3">
                            <h4 class="text-xs font-mono uppercase tracking-wider text-zinc-900 dark:text-zinc-100 font-semibold" data-i18n="footer.col.framework">
                                "Framework"
                            </h4>
                            <ul class="space-y-2 text-zinc-600 dark:text-zinc-400">
                                <li><a href="/" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.overview">"Overview"</a></li>
                                <li><a href="/about" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.architecture">"Architecture"</a></li>
                                <li><a href="/docs" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.docs">"Documentation"</a></li>
                                <li><a href="/benchmarks" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.benchmarks">"Benchmarks"</a></li>
                                <li><a href="/components" class="hover:text-zinc-900 dark:hover:text-white transition-colors" data-i18n="nav.components">"Components"</a></li>
                            </ul>
                        </div>

                        // Col 3: Ecosystem Crates
                        <div class="space-y-3">
                            <h4 class="text-xs font-mono uppercase tracking-wider text-zinc-900 dark:text-zinc-100 font-semibold" data-i18n="footer.col.ecosystem">
                                "Ecosystem"
                            </h4>
                            <ul class="space-y-2 text-zinc-600 dark:text-zinc-400 font-mono text-[11px]">
                                <li><a href="https://crates.io/crates/yav" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors">"yav (metapackage)"</a></li>
                                <li><a href="https://crates.io/crates/yav-core" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors">"yav-core"</a></li>
                                <li><a href="https://crates.io/crates/yav-macros" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors">"yav-macros (RSX)"</a></li>
                                <li><a href="https://crates.io/crates/yav-server" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors">"yav-server (Axum 0.8)"</a></li>
                                <li><a href="https://crates.io/crates/yav-cli" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors">"yav-cli toolchain"</a></li>
                            </ul>
                        </div>

                        // Col 4: Community & Links
                        <div class="space-y-3">
                            <h4 class="text-xs font-mono uppercase tracking-wider text-zinc-900 dark:text-zinc-100 font-semibold" data-i18n="footer.col.community">
                                "Community"
                            </h4>
                            <ul class="space-y-2 text-zinc-600 dark:text-zinc-400">
                                <li>
                                    <a href="https://github.com/isaacvay/yav.rs" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors flex items-center gap-1.5">
                                        {render_icon("minimal:github", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                        <span>"GitHub"</span>
                                    </a>
                                </li>
                                <li>
                                    <a href="https://docs.rs/yav" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors flex items-center gap-1.5">
                                        {render_icon("book-open", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                        <span>"docs.rs/yav"</span>
                                    </a>
                                </li>
                                <li>
                                    <a href="https://crates.io/crates/yav" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors flex items-center gap-1.5">
                                        {render_icon("box", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                        <span>"crates.io"</span>
                                    </a>
                                </li>
                                <li>
                                    <a href="https://github.com/isaacvay/yav.rs/issues" target="_blank" rel="noreferrer" class="hover:text-zinc-900 dark:hover:text-white transition-colors flex items-center gap-1.5">
                                        {render_icon("circle", IconSize::Sm, Some("w-3 h-3 text-amber-500"), None)}
                                        <span>"Issue Tracker"</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>

                    // Bottom Bar
                    <div class="pt-8 flex flex-col sm:flex-row items-center justify-between gap-4 font-mono text-[11px] text-zinc-500">
                        <div class="flex items-center gap-2">
                            <span>"© 2026 Isaac Yav &amp; Contributors."</span>
                            <span>"•"</span>
                            <span data-i18n="footer.copyright">"Dual-licensed under MIT or Apache-2.0."</span>
                        </div>
                        <div class="flex items-center gap-4">
                            <span class="flex items-center gap-1">
                                {render_icon("cpu", IconSize::Sm, Some("w-3 h-3 text-zinc-400"), None)}
                                <span>"Built with Rust 1.75+"</span>
                            </span>
                            <span>"•"</span>
                            <span class="text-zinc-400" data-i18n="footer.rendered">"Rendered with RSX in 0.12ms"</span>
                        </div>
                    </div>
                </div>
            </footer>
            {motion_script()}
        </div>
    }
}
