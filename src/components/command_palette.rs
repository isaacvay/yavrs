//! Global Command Palette (⌘K) interactive modal.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn CommandPalette() -> Node {
    rsx! {
        <div id="cmd-palette-backdrop" class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs hidden flex items-start justify-center pt-20 px-4 transition-all">
            <div
                id="cmd-palette-modal"
                class="w-full max-w-xl rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-2xl overflow-hidden transition-all transform scale-95 opacity-0"
            >
                // Search Input Header
                <div class="flex items-center px-4 border-b border-zinc-200 dark:border-zinc-800">
                    {render_icon("search", IconSize::Sm, Some("w-4 h-4 text-zinc-400 mr-3"), None)}
                    <input
                        id="cmd-palette-input"
                        type="text"
                        placeholder="Type a command or search documentation..."
                        class="w-full h-12 bg-transparent text-sm text-zinc-900 dark:text-zinc-100 placeholder-zinc-400 focus:outline-hidden font-sans"
                        oninput="filterCommands(this.value)"
                    />
                    <kbd class="hidden sm:inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded text-[10px] font-mono bg-zinc-100 dark:bg-zinc-800 text-zinc-500 border border-zinc-200 dark:border-zinc-700">
                        "ESC"
                    </kbd>
                </div>

                // Quick Navigation Items
                <div class="p-2 max-h-80 overflow-y-auto space-y-1 text-xs" id="cmd-palette-list">
                    <div class="px-3 py-1.5 text-[10px] font-mono uppercase tracking-wider text-zinc-400 font-semibold">
                        "Navigation"
                    </div>

                    <a href="/" class="cmd-item flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors">
                        <div class="flex items-center gap-2.5">
                            <img src="/public/favicon.svg" class="w-4 h-4" alt="Yav logo" />
                            <span class="font-medium">"Home / Overview"</span>
                        </div>
                        <span class="text-[11px] font-mono text-zinc-400">"/"</span>
                    </a>

                    <a href="/docs" class="cmd-item flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors">
                        <div class="flex items-center gap-2.5">
                            {render_icon("book-open", IconSize::Sm, Some("w-4 h-4 text-amber-500"), None)}
                            <span class="font-medium">"Documentation & Quickstart"</span>
                        </div>
                        <span class="text-[11px] font-mono text-zinc-400">"/docs"</span>
                    </a>

                    <a href="/about" class="cmd-item flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors">
                        <div class="flex items-center gap-2.5">
                            {render_icon("layers", IconSize::Sm, Some("w-4 h-4 text-sky-500"), None)}
                            <span class="font-medium">"Architecture & Internals"</span>
                        </div>
                        <span class="text-[11px] font-mono text-zinc-400">"/about"</span>
                    </a>

                    <a href="/benchmarks" class="cmd-item flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors">
                        <div class="flex items-center gap-2.5">
                            {render_icon("activity", IconSize::Sm, Some("w-4 h-4 text-emerald-500"), None)}
                            <span class="font-medium">"Performance Benchmarks"</span>
                        </div>
                        <span class="text-[11px] font-mono text-zinc-400">"/benchmarks"</span>
                    </a>

                    <a href="/components" class="cmd-item flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors">
                        <div class="flex items-center gap-2.5">
                            {render_icon("box", IconSize::Sm, Some("w-4 h-4 text-purple-500"), None)}
                            <span class="font-medium">"UI Components & Icons"</span>
                        </div>
                        <span class="text-[11px] font-mono text-zinc-400">"/components"</span>
                    </a>

                    <div class="px-3 pt-3 pb-1.5 text-[10px] font-mono uppercase tracking-wider text-zinc-400 font-semibold">
                        "Quick Actions"
                    </div>

                    <button
                        onclick="navigator.clipboard.writeText('cargo install yav-cli'); alertNotification('Copied: cargo install yav-cli'); closeCmdPalette();"
                        class="cmd-item w-full flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors text-left"
                    >
                        <div class="flex items-center gap-2.5">
                            {render_icon("terminal", IconSize::Sm, Some("w-4 h-4 text-emerald-500"), None)}
                            <span class="font-mono text-[11px]">"cargo install yav-cli"</span>
                        </div>
                        <span class="text-[10px] font-mono text-zinc-400">"Copy command"</span>
                    </button>

                    <button
                        onclick="navigator.clipboard.writeText('yav new my-app'); alertNotification('Copied: yav new my-app'); closeCmdPalette();"
                        class="cmd-item w-full flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors text-left"
                    >
                        <div class="flex items-center gap-2.5">
                            {render_icon("plus", IconSize::Sm, Some("w-4 h-4 text-amber-500"), None)}
                            <span class="font-mono text-[11px]">"yav new my-app"</span>
                        </div>
                        <span class="text-[10px] font-mono text-zinc-400">"Scaffold project"</span>
                    </button>

                    <a
                        href="https://github.com/isaacvay/yav.rs"
                        target="_blank"
                        rel="noreferrer"
                        class="cmd-item flex items-center justify-between px-3 py-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-200 cursor-pointer transition-colors"
                    >
                        <div class="flex items-center gap-2.5">
                            {render_icon("minimal:github", IconSize::Sm, Some("w-4 h-4 text-zinc-400"), None)}
                            <span class="font-medium">"View GitHub Repository"</span>
                        </div>
                        {render_icon("external-link", IconSize::Sm, Some("w-3 h-3 text-zinc-400"), None)}
                    </a>
                </div>

                // Footer Info
                <div class="px-4 py-2 bg-zinc-50 dark:bg-zinc-950/50 border-t border-zinc-200 dark:border-zinc-800 flex items-center justify-between text-[11px] text-zinc-400">
                    <span class="flex items-center gap-1">
                        <kbd class="px-1 py-0.5 rounded bg-zinc-200/70 dark:bg-zinc-800 font-mono text-[10px]">"↑"</kbd>
                        <kbd class="px-1 py-0.5 rounded bg-zinc-200/70 dark:bg-zinc-800 font-mono text-[10px]">"↓"</kbd>
                        <span class="ml-1">"Navigate"</span>
                    </span>
                    <span>"Press ESC to exit"</span>
                </div>
            </div>
        </div>

        // Notification Toast
        <div id="cmd-notification" class="fixed bottom-6 right-6 z-50 transform translate-y-12 opacity-0 transition-all duration-300 pointer-events-none">
            <div class="px-4 py-2.5 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 text-xs font-mono font-medium shadow-xl flex items-center gap-2 border border-zinc-700 dark:border-zinc-300">
                <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
                <span id="cmd-notification-text">"Action complete"</span>
            </div>
        </div>

        <script>
            {r#"
            function openCmdPalette() {
                const backdrop = document.getElementById('cmd-palette-backdrop');
                const modal = document.getElementById('cmd-palette-modal');
                backdrop.classList.remove('hidden');
                setTimeout(() => {
                    modal.classList.remove('scale-95', 'opacity-0');
                    modal.classList.add('scale-100', 'opacity-100');
                    document.getElementById('cmd-palette-input').focus();
                }, 10);
            }

            function closeCmdPalette() {
                const backdrop = document.getElementById('cmd-palette-backdrop');
                const modal = document.getElementById('cmd-palette-modal');
                modal.classList.remove('scale-100', 'opacity-100');
                modal.classList.add('scale-95', 'opacity-0');
                setTimeout(() => {
                    backdrop.classList.add('hidden');
                }, 150);
            }

            function alertNotification(msg) {
                const toast = document.getElementById('cmd-notification');
                const text = document.getElementById('cmd-notification-text');
                text.innerText = msg;
                toast.classList.remove('translate-y-12', 'opacity-0');
                toast.classList.add('translate-y-0', 'opacity-100');
                setTimeout(() => {
                    toast.classList.remove('translate-y-0', 'opacity-100');
                    toast.classList.add('translate-y-12', 'opacity-0');
                }, 2000);
            }

            function filterCommands(query) {
                const q = query.toLowerCase().trim();
                const items = document.querySelectorAll('#cmd-palette-list .cmd-item');
                items.forEach(el => {
                    const text = el.innerText.toLowerCase();
                    if (!q || text.includes(q)) {
                        el.style.display = 'flex';
                    } else {
                        el.style.display = 'none';
                    }
                });
            }

            document.addEventListener('keydown', (e) => {
                if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
                    e.preventDefault();
                    const backdrop = document.getElementById('cmd-palette-backdrop');
                    if (backdrop.classList.contains('hidden')) {
                        openCmdPalette();
                    } else {
                        closeCmdPalette();
                    }
                }
                if (e.key === 'Escape') {
                    closeCmdPalette();
                }
            });

            document.getElementById('cmd-palette-backdrop')?.addEventListener('click', (e) => {
                if (e.target.id === 'cmd-palette-backdrop') {
                    closeCmdPalette();
                }
            });
            "#}
        </script>
    }
}
