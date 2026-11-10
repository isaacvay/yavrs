//! 5.9 Command Menu (⌘K) Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentCommandMenu() -> Node {
    rsx! {
        <section id="commandmenu" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.9"</span>
                <span class="t" data-i18n="comp.cmd.sec">"Command Menu (⌘K)"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add command-menu"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.cmd.title">"Spotlight command palette."</h2>
                <button
                    onclick="copyText(this, 'yav add command-menu')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add command-menu"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.cmd.lede">
                "Instant keyboard-driven command bar modeled after Raycast and Spotlight. Scaffolds global keyboard bindings, fuzzy search filtering across compiled routes, and terminal shortcuts in 2 KB total footprint."
            </p>

            // Live Interactive Preview Plate
            <div class="plate max-w-2xl mx-auto overflow-hidden">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.cmd.preview_head">"COMMAND PALETTE PREVIEW"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.cmd.preview_sub">"Press ⌘K anywhere on site"</span>
                </div>

                // Modal Container Preview
                <div class="bg-sunk border-t border-[rgba(236,233,227,0.08)]">
                    // Search Bar Header
                    <div class="flex items-center px-4 border-b border-[rgba(236,233,227,0.08)] bg-ink-2">
                        {render_icon("search", IconSize::Sm, Some("w-4 h-4 text-dim mr-3"), None)}
                        <input
                            type="text"
                            placeholder="Search documentation, routes or run command..."
                            data-i18n-placeholder="comp.cmd.ph"
                            class="w-full h-12 bg-transparent text-xs text-paper placeholder:text-dim focus:outline-none font-mono"
                        />
                        <kbd class="kbd ml-2">"ESC"</kbd>
                    </div>

                    // Quick Navigation Items List
                    <div class="p-2 max-h-72 overflow-y-auto space-y-0.5 text-xs font-mono">
                        <div class="px-3 pt-2 pb-1 label text-[10px] text-oxide" data-i18n="comp.cmd.g1">
                            "01 Navigation Routes"
                        </div>

                        <div class="flex items-center justify-between px-3 py-2 text-paper bg-[rgba(236,233,227,0.05)] rounded cursor-pointer">
                            <div class="flex items-center gap-3">
                                <span class="text-oxide font-bold">"01"</span>
                                <span data-i18n="comp.cmd.r1">"Overview / Home"</span>
                            </div>
                            <span class="text-dim text-[11px]">"/"</span>
                        </div>

                        <div class="flex items-center justify-between px-3 py-2 text-dim hover:text-paper hover:bg-[rgba(236,233,227,0.03)] rounded cursor-pointer transition-colors">
                            <div class="flex items-center gap-3">
                                <span class="text-dim">"02"</span>
                                <span data-i18n="comp.cmd.r2">"Documentation & Guides"</span>
                            </div>
                            <span class="text-dim text-[11px]">"/docs"</span>
                        </div>

                        <div class="flex items-center justify-between px-3 py-2 text-dim hover:text-paper hover:bg-[rgba(236,233,227,0.03)] rounded cursor-pointer transition-colors">
                            <div class="flex items-center gap-3">
                                <span class="text-dim">"03"</span>
                                <span data-i18n="comp.cmd.r3">"Verified Benchmarks"</span>
                            </div>
                            <span class="text-dim text-[11px]">"/benchmarks"</span>
                        </div>

                        <div class="px-3 pt-3 pb-1 label text-[10px] text-oxide" data-i18n="comp.cmd.g2">
                            "02 Developer Actions"
                        </div>

                        <div onclick="copyText(this, 'cargo install yav-cli')" class="flex items-center justify-between px-3 py-2 text-paper hover:bg-[rgba(236,233,227,0.03)] rounded cursor-pointer transition-colors">
                            <div class="flex items-center gap-2.5">
                                <span class="text-oxide">"$"</span>
                                <span>"cargo install yav-cli"</span>
                            </div>
                            <span data-copy-label="" data-i18n="common.copy" class="text-[10px] text-oxide font-medium border border-rule px-1.5 py-0.5 rounded">"copy"</span>
                        </div>

                        <div class="flex items-center justify-between px-3 py-2 text-paper hover:bg-[rgba(236,233,227,0.03)] rounded cursor-pointer transition-colors">
                            <div class="flex items-center gap-2.5">
                                <span class="text-oxide">"$"</span>
                                <span>"yav new my-app"</span>
                            </div>
                            <span class="text-[10px] text-dim border border-rule px-1.5 py-0.5 rounded" data-i18n="comp.cmd.scaffold">"scaffold"</span>
                        </div>
                    </div>

                    // Footer Keyboard Hints
                    <div class="p-2.5 border-t border-[rgba(236,233,227,0.08)] bg-ink-2 flex items-center justify-between text-[11px] font-mono text-dim">
                        <div class="flex items-center gap-3">
                            <span><kbd class="kbd text-[10px]">"↑↓"</kbd><span data-i18n="comp.cmd.nav">" navigate"</span></span>
                            <span><kbd class="kbd text-[10px]">"↵"</kbd><span data-i18n="comp.cmd.select">" select"</span></span>
                            <span><kbd class="kbd text-[10px]">"esc"</kbd><span data-i18n="comp.cmd.close">" close"</span></span>
                        </div>
                        <span class="text-faint" data-i18n="comp.cmd.fuzzy">"Fuzzy search index"</span>
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
                                "src/components/command_menu.rs"
                            </div>
                        </div>
                        <button onclick="copyText(this, 'command_palette(CommandPaletteProps { routes: vec![\"/\", \"/docs\"], actions })')" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Command Palette modal template"</div>
                            <div><span class="token-tag">"<div "</span><span class="token-attr">"id="</span><span class="token-string">"\"cmd-palette-backdrop\""</span>" "<span class="token-attr">"class="</span><span class="token-string">"\"fixed inset-0 z-50 backdrop-blur-md hidden\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"plate bg-sunk max-w-xl mx-auto\""</span><span class="token-tag">">"</span></div>
                            <div>"        "<span class="token-tag">"<input "</span><span class="token-attr">"type="</span><span class="token-string">"\"text\""</span>" "<span class="token-attr">"oninput="</span><span class="token-string">"\"filterCommands(this.value)\""</span><span class="token-tag">" />"</span></div>
                            <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"cmd-list\""</span><span class="token-tag">">"</span>"...items..."<span class="token-tag">"</div>"</span></div>
                            <div>"    "<span class="token-tag">"</div>"</span></div>
                            <div><span class="token-tag">"</div>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="comp.cmd.ide_l">"Global keydown listener (MetaKey / CtrlKey + K)"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="comp.cmd.ide_r">"W3C Dialog"</span></span>
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
                            <span class="text-faint text-[11px] ml-2" data-i18n="comp.common.term_head">"bash — yav-cli scaffold"</span>
                        </div>
                        <button onclick="copyText(this, 'yav add command-menu')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_cmd" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add command-menu"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/command_palette.rs [4.2 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
