//! 5.10 Vector Icon Explorer Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentIcons() -> Node {
    rsx! {
        <section id="icons" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.10"</span>
                <span class="t">"Vector Icon Explorer"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav::icons"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"Compiled vector catalog."</h2>
                <button
                    onclick="copyText(this, 'cargo add yav-icons')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"cargo add yav-icons"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Over 13,000 clean vector icons compiled directly into your native binary via Lucide, MynaUI, and Phosphor. Zero network roundtrips, zero external SVG layout shifts. Click any icon to inspect and copy its exact RSX invocation snippet."
            </p>

            // Live Interactive Icon Explorer Plate
            <div class="plate overflow-hidden">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        "VECTOR ICON EXPLORER (36 OF 13,000+)"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"Click any icon to inspect"</span>
                </div>

                // Search & Filter Category Bar
                <div class="p-4 bg-ink-2 border-b border-[rgba(236,233,227,0.08)] space-y-3">
                    <div class="flex flex-col sm:flex-row items-center justify-between gap-3">
                        <div class="relative w-full sm:w-80">
                            <div class="absolute left-3 top-2.5 pointer-events-none text-dim">
                                {render_icon("search", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                            </div>
                            <input
                                type="text"
                                id="icon-search-field"
                                placeholder="Filter icons (e.g. terminal, server, shield)..."
                                oninput="filterIconCatalog(this.value)"
                                class="w-full bg-sunk border border-rule rounded pl-9 pr-3 py-1.5 text-xs font-mono text-paper placeholder:text-dim outline-none focus:border-oxide transition-colors"
                            />
                        </div>

                        // Category Pills
                        <div class="flex flex-wrap items-center gap-1.5 w-full sm:w-auto text-[11px] font-mono">
                            <button class="px-2.5 py-1 bg-ink border border-oxide text-paper font-medium rounded" onclick="filterIconCategory('all')">"All (36)"</button>
                            <button class="px-2.5 py-1 bg-sunk border border-rule text-dim hover:text-paper rounded" onclick="filterIconCategory('compute')">"Compute"</button>
                            <button class="px-2.5 py-1 bg-sunk border border-rule text-dim hover:text-paper rounded" onclick="filterIconCategory('security')">"Security"</button>
                            <button class="px-2.5 py-1 bg-sunk border border-rule text-dim hover:text-paper rounded" onclick="filterIconCategory('dev')">"Dev & Git"</button>
                            <button class="px-2.5 py-1 bg-sunk border border-rule text-dim hover:text-paper rounded" onclick="filterIconCategory('ui')">"UI Controls"</button>
                        </div>
                    </div>
                </div>

                // Live Inspector Bar (Updates dynamically on icon click!)
                <div id="icon-inspect-bar" class="p-4 bg-[#09090b] border-b border-[rgba(236,233,227,0.08)] flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 font-mono text-xs">
                    <div class="flex items-center gap-4">
                        <div class="w-10 h-10 rounded bg-ink-2 border border-rule flex items-center justify-center text-oxide" id="inspect-icon-preview">
                            {render_icon("terminal", IconSize::Md, Some("w-6 h-6 text-oxide"), None)}
                        </div>
                        <div class="space-y-0.5">
                            <div class="flex items-center gap-2">
                                <span class="text-paper font-semibold text-[13px]" id="inspect-icon-name">"terminal"</span>
                                <span class="text-[10px] px-1.5 py-0.5 bg-ink-2 border border-rule text-dim rounded" id="inspect-icon-cat">"compute"</span>
                            </div>
                            <div class="text-[11px] text-faint break-all" id="inspect-icon-syntax">
                                "{render_icon(\"terminal\", IconSize::Sm, Some(\"w-4 h-4 text-oxide\"), None)}"
                            </div>
                        </div>
                    </div>
                    <button
                        id="inspect-copy-btn"
                        data-copy="{render_icon(&quot;terminal&quot;, IconSize::Sm, Some(&quot;w-4 h-4 text-oxide&quot;), None)}"
                        onclick="copyText(this)"
                        class="btn btn-sm btn-line self-stretch sm:self-auto justify-center flex items-center gap-2 text-[11px] hover:border-oxide/40"
                    >
                        {render_icon("copy", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        <span data-copy-label="" class="text-oxide font-medium">"Copy RSX Snippet"</span>
                    </button>
                </div>

                // 36 Core Icons Grid
                <div class="p-6 bg-ink-2" id="icon-catalog-grid">
                    <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 gap-3">
                        // Compute & System
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('terminal', 'compute')" data-name="terminal" data-cat="compute">
                            {render_icon("terminal", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"terminal"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('server', 'compute')" data-name="server" data-cat="compute">
                            {render_icon("server", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"server"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('cpu', 'compute')" data-name="cpu" data-cat="compute">
                            {render_icon("cpu", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"cpu"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('database', 'compute')" data-name="database" data-cat="compute">
                            {render_icon("database", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"database"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('box', 'compute')" data-name="box" data-cat="compute">
                            {render_icon("box", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"box"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('activity', 'compute')" data-name="activity" data-cat="compute">
                            {render_icon("activity", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"activity"</span>
                        </div>

                        // Security & Infra
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('shield', 'security')" data-name="shield" data-cat="security">
                            {render_icon("shield", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"shield"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('lock', 'security')" data-name="lock" data-cat="security">
                            {render_icon("lock", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"lock"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('zap', 'security')" data-name="zap" data-cat="security">
                            {render_icon("zap", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"zap"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('bell', 'security')" data-name="bell" data-cat="security">
                            {render_icon("bell", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"bell"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('globe', 'security')" data-name="globe" data-cat="security">
                            {render_icon("globe", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"globe"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('layers', 'security')" data-name="layers" data-cat="security">
                            {render_icon("layers", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"layers"</span>
                        </div>

                        // Dev & Git
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('code', 'dev')" data-name="code" data-cat="dev">
                            {render_icon("code", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"code"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('git-branch', 'dev')" data-name="git-branch" data-cat="dev">
                            {render_icon("git-branch", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"git-branch"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('folder', 'dev')" data-name="folder" data-cat="dev">
                            {render_icon("folder", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"folder"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('file-text', 'dev')" data-name="file-text" data-cat="dev">
                            {render_icon("file-text", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"file-text"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('settings', 'dev')" data-name="settings" data-cat="dev">
                            {render_icon("settings", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"settings"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('copy', 'dev')" data-name="copy" data-cat="dev">
                            {render_icon("copy", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"copy"</span>
                        </div>

                        // UI Controls & Navigation
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('search', 'ui')" data-name="search" data-cat="ui">
                            {render_icon("search", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"search"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('arrow-right', 'ui')" data-name="arrow-right" data-cat="ui">
                            {render_icon("arrow-right", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"arrow-right"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('arrow-left', 'ui')" data-name="arrow-left" data-cat="ui">
                            {render_icon("arrow-left", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"arrow-left"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('check', 'ui')" data-name="check" data-cat="ui">
                            {render_icon("check", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"check"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('x', 'ui')" data-name="x" data-cat="ui">
                            {render_icon("x", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"x"</span>
                        </div>
                        <div class="icon-item p-4 border border-[rgba(236,233,227,0.08)] hover:border-oxide flex flex-col items-center justify-center gap-2.5 transition-colors bg-sunk text-center group cursor-pointer" onclick="selectIcon('menu', 'ui')" data-name="menu" data-cat="ui">
                            {render_icon("menu", IconSize::Md, Some("w-5 h-5 text-paper group-hover:text-oxide transition-colors"), None)}
                            <span class="font-mono text-[11px] text-dim group-hover:text-paper">"menu"</span>
                        </div>
                    </div>
                </div>
            </div>

            // Code Preview Windows: Tabbed IDE + Terminal
            <div class="space-y-3 pt-2">
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/icon_example.rs"
                            </div>
                        </div>
                        <button data-copy="use yav::icons::{render_icon, IconSize};\n{render_icon(&quot;terminal&quot;, IconSize::Sm, Some(&quot;w-4 h-4 text-oxide&quot;), None)}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Vector Icon Rendering Macro"</div>
                            <div><span class="token-keyword">"use"</span>" yav::icons::{"<span class="token-fn">"render_icon"</span>", "<span class="token-type">"IconSize"</span>"};"</div>
                            <div>
                                "{render_icon("
                            </div>
                            <div>
                                "    "<span class="token-string">"\"terminal\""</span>","
                            </div>
                            <div>
                                "    "<span class="token-type">"IconSize::Md"</span>","
                            </div>
                            <div>
                                "    "<span class="token-type">"Some"</span>"("<span class="token-string">"\"w-5 h-5 text-oxide\""</span>"),"
                            </div>
                            <div>
                                "    "<span class="token-type">"None"</span>
                            </div>
                            <div>
                                ")}"
                            </div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"Zero network requests · Inlined SVG output · 13,000+ icons"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"Fast Compilation"</span>
                    </div>
                </div>

                // Terminal Scaffolding Command
                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-1.5">
                                <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                            </div>
                            <span class="text-faint text-[11px] ml-2">"bash — yav icon search"</span>
                        </div>
                        <button onclick="copyText(this, 'yav icon search terminal')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav icon search terminal"</div>
                        <div class="text-faint text-[11px]">"Found 4 matches: terminal, terminal-square, command, console"</div>
                    </div>
                </div>
            </div>

            // Script for Interactive Icon Selection and Filtering
            <script>
                {r#"
                function selectIcon(name, cat) {
                    var nameEl = document.getElementById('inspect-icon-name');
                    var catEl = document.getElementById('inspect-icon-cat');
                    var syntaxEl = document.getElementById('inspect-icon-syntax');
                    var copyBtn = document.getElementById('inspect-copy-btn');
                    if (nameEl) nameEl.textContent = name;
                    if (catEl) catEl.textContent = cat;
                    var code = '{render_icon("' + name + '", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}';
                    if (syntaxEl) syntaxEl.textContent = code;
                    if (copyBtn) {
                        copyBtn.setAttribute('data-copy', code);
                        copyBtn.setAttribute('onclick', 'copyText(this)');
                    }
                }
                function filterIconCatalog(query) {
                    var q = (query || '').toLowerCase().trim();
                    var items = document.querySelectorAll('#icon-catalog-grid .icon-item');
                    items.forEach(function(item) {
                        var name = item.getAttribute('data-name') || '';
                        if (!q || name.indexOf(q) !== -1) {
                            item.style.display = '';
                        } else {
                            item.style.display = 'none';
                        }
                    });
                }
                function filterIconCategory(cat) {
                    var items = document.querySelectorAll('#icon-catalog-grid .icon-item');
                    items.forEach(function(item) {
                        var itemCat = item.getAttribute('data-cat') || '';
                        if (cat === 'all' || itemCat === cat) {
                            item.style.display = '';
                        } else {
                            item.style.display = 'none';
                        }
                    });
                }
                "#}
            </script>
        </section>
    }
}
