//! Global Command Palette & Search Popover — Editorial Blueprint.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn CommandPalette() -> Node {
    rsx! {
        <div class="relative self-stretch flex items-center" id="header-search-container">
            // Trigger Button
            <button
                id="header-search-btn"
                onclick="window.toggleCmdPalette(event)"
                class="hidden md:flex items-center gap-2 h-[34px] px-2.5 xl:pl-3 xl:pr-2.5 border border-rule hover:border-oxide/50 text-[13px] text-dim hover:text-paper transition-colors cursor-pointer bg-ink-2 shrink-0"
                title="Search documentation (Ctrl+K)"
                aria-expanded="false"
                aria-haspopup="true"
            >
                {render_icon("search", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                <span class="hidden xl:inline" data-i18n="nav.search">"Search"</span>
                <kbd class="kbd hidden xl:inline-flex text-[10px]">"Ctrl K"</kbd>
            </button>

            // Popover Dropdown — opens strictly directly below the 64px navbar
            <div
                id="cmd-palette-popover"
                class="popover-dropdown search-popover-mobile search-popover-desktop plate bg-ink-2 shadow-[0_24px_70px_rgba(0,0,0,0.95)] border border-rule overflow-hidden"
                onclick="event.stopPropagation()"
            >
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>

                // Search Input Header
                <div class="flex items-center px-3.5 border-b border-rule bg-sunk">
                    {render_icon("search", IconSize::Sm, Some("w-4 h-4 text-oxide mr-2.5 shrink-0"), None)}
                    <input
                        id="cmd-palette-input"
                        type="text"
                        placeholder="Search documentation, routes, commands..."
                        class="w-full h-11 bg-transparent text-sm text-paper placeholder-dim focus:outline-none font-sans"
                        oninput="filterCommands(this.value)"
                        autocomplete="off"
                    />
                    <button
                        onclick="window.closeCmdPalette()"
                        class="text-dim hover:text-paper ml-2 text-xs font-mono p-1 cursor-pointer"
                        title="Close (ESC)"
                    >
                        <kbd class="kbd text-[10px]">"ESC"</kbd>
                    </button>
                </div>

                // Navigation & Action Items
                <div class="p-1.5 max-h-[340px] overflow-y-auto space-y-0.5 text-xs font-sans" id="cmd-palette-list">
                    <div class="px-2.5 pt-2 pb-1 label text-[10px] text-dim font-mono">
                        "01 Index Navigation"
                    </div>

                    <a href="/" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"01"</span>
                            <span class="font-medium">"Overview / Homepage"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/"</span>
                    </a>

                    <a href="/about" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"02"</span>
                            <span class="font-medium">"Architecture & Tokio Engine"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/about"</span>
                    </a>

                    <a href="/docs" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"03"</span>
                            <span class="font-medium">"Documentation & Quickstart"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/docs"</span>
                    </a>

                    <a href="/benchmarks" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"04"</span>
                            <span class="font-medium">"Verified Benchmarks"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/benchmarks"</span>
                    </a>

                    <a href="/components" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"05"</span>
                            <span class="font-medium">"UI Components & Icons"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/components"</span>
                    </a>

                    <a href="/showcase" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"06"</span>
                            <span class="font-medium">"Production Showcase"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/showcase"</span>
                    </a>

                    <a href="/faq" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"07"</span>
                            <span class="font-medium">"Why Yav? / FAQ"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/faq"</span>
                    </a>

                    <a href="/releases" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"08"</span>
                            <span class="font-medium">"Releases & Changelog"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/releases"</span>
                    </a>

                    <a href="/privacy" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="font-mono text-[11px] text-oxide group-hover:underline">"09"</span>
                            <span class="font-medium">"Privacy & Security Model"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/privacy"</span>
                    </a>

                    <div class="px-2.5 pt-3 pb-1 label text-[10px] text-dim font-mono">
                        "02 Enterprise Architecture & Protocols"
                    </div>

                    <a href="/docs#ai-mcp" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-[11px]">"✦"</span>
                            <span class="font-medium">"AI SDK & Model Context Protocol (MCP)"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/docs#ai-mcp"</span>
                    </a>

                    <a href="/docs#passkeys-rebac" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-[11px]">"✦"</span>
                            <span class="font-medium">"WebAuthn Passkeys & ReBAC Zanzibar"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/docs#passkeys-rebac"</span>
                    </a>

                    <a href="/docs#resilience" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-[11px]">"✦"</span>
                            <span class="font-medium">"Resilience: Circuit Breaker & XFetch"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/docs#resilience"</span>
                    </a>

                    <a href="/docs#modern-protocols" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-[11px]">"✦"</span>
                            <span class="font-medium">"GraphQL, TurboStream & Tus Resumable"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/docs#modern-protocols"</span>
                    </a>

                    <a href="/components#charts" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="text-ok font-mono text-[11px]">"⬡"</span>
                            <span class="font-medium">"Enterprise SVG Charts & Heatmaps"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/components#charts"</span>
                    </a>

                    <a href="/components#enterprise" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="text-ok font-mono text-[11px]">"⬡"</span>
                            <span class="font-medium">"DataGrid, Kanban (LexoRank) & FlowCanvas"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/components#enterprise"</span>
                    </a>

                    <a href="/components#visual-widgets" onclick="window.closeCmdPalette()" class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors group rounded">
                        <div class="flex items-center gap-2.5">
                            <span class="text-ok font-mono text-[11px]">"⬡"</span>
                            <span class="font-medium">"3D Mesh Viewer & Reactive Spreadsheet"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"/components#visual-widgets"</span>
                    </a>

                    <div class="px-2.5 pt-3 pb-1 label text-[10px] text-dim font-mono">
                        "03 Developer Actions"
                    </div>

                    <button
                        onclick="navigator.clipboard.writeText('cargo install yav-cli'); alertNotification('Copied: cargo install yav-cli'); window.closeCmdPalette();"
                        class="cmd-item w-full flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono rounded"
                    >
                        <div class="flex items-center gap-2.5">
                            <span class="text-oxide">"$"</span>
                            <span class="text-[12px] text-paper">"cargo install yav-cli"</span>
                        </div>
                        <span class="text-[10px] text-oxide font-medium">"copy"</span>
                    </button>

                    <button
                        onclick="navigator.clipboard.writeText('yav new my-app'); alertNotification('Copied: yav new my-app'); window.closeCmdPalette();"
                        class="cmd-item w-full flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono rounded"
                    >
                        <div class="flex items-center gap-2.5">
                            <span class="text-oxide">"$"</span>
                            <span class="text-[12px] text-paper">"yav new my-app"</span>
                        </div>
                        <span class="text-[10px] text-dim">"scaffold"</span>
                    </button>

                    <a
                        href="https://github.com/isaacvay/yav.rs"
                        target="_blank"
                        rel="noreferrer"
                        class="cmd-item flex items-center justify-between px-2.5 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors rounded"
                    >
                        <div class="flex items-center gap-2.5">
                            {render_icon("minimal:github", IconSize::Sm, Some("w-4 h-4 text-dim"), None)}
                            <span class="font-medium">"GitHub Repository"</span>
                        </div>
                        <span class="font-mono text-[11px] text-dim">"↗"</span>
                    </a>

                    <div id="cmd-no-results" class="hidden p-6 text-center text-dim font-mono text-xs">
                        "No matching routes or commands found."
                    </div>
                </div>

                // Footer Info
                <div class="px-3.5 py-1.5 bg-sunk border-t border-rule flex items-center justify-between text-[11px] text-dim font-mono">
                    <span class="flex items-center gap-1.5">
                        <span class="text-oxide">"✦"</span>
                        <span class="text-mute text-[10px]">"Editorial Command Bar"</span>
                    </span>
                    <span class="text-mute text-[10px]">"ESC to close"</span>
                </div>
            </div>

            // Notification Toast
            <div id="cmd-notification" class="fixed bottom-6 right-6 z-50 transform translate-y-12 opacity-0 transition-all duration-300 pointer-events-none">
                <div class="px-4 py-2.5 plate bg-sunk text-paper text-xs font-mono flex items-center gap-2.5 border border-rule shadow-xl">
                    <span class="dot dot-ok"></span>
                    <span id="cmd-notification-text">"Action complete"</span>
                </div>
            </div>

            <script>
                {r#"
                function alertNotification(msg) {
                    var toast = document.getElementById('cmd-notification');
                    var text = document.getElementById('cmd-notification-text');
                    if (!toast || !text) return;
                    text.innerText = msg;
                    toast.classList.remove('translate-y-12', 'opacity-0');
                    toast.classList.add('translate-y-0', 'opacity-100');
                    setTimeout(function() {
                        toast.classList.remove('translate-y-0', 'opacity-100');
                        toast.classList.add('translate-y-12', 'opacity-0');
                    }, 2000);
                }

                function filterCommands(query) {
                    var q = (query || '').toLowerCase().trim();
                    var items = document.querySelectorAll('#cmd-palette-list .cmd-item');
                    var visible = 0;
                    items.forEach(function(el) {
                        var text = (el.innerText || '').toLowerCase();
                        if (!q || text.indexOf(q) !== -1) {
                            el.style.display = 'flex';
                            visible++;
                        } else {
                            el.style.display = 'none';
                        }
                    });
                    var empty = document.getElementById('cmd-no-results');
                    if (empty) {
                        if (visible === 0) {
                            empty.classList.remove('hidden');
                        } else {
                            empty.classList.add('hidden');
                        }
                    }
                }
                "#}
            </script>
        </div>
    }
}
