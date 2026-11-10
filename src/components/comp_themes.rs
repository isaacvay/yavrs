//! 5.12 Multi-Theme Design System (8 Themes) Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentThemes() -> Node {
    let script = Node::raw(r#"<script>
window.previewTheme = function(themeName, bg, fg, accent) {
    var preview = document.getElementById('theme-preview-canvas');
    var badge = document.getElementById('theme-active-name');
    var varBg = document.getElementById('theme-var-bg');
    var varFg = document.getElementById('theme-var-fg');
    var varAccent = document.getElementById('theme-var-accent');
    if (preview) {
        preview.style.backgroundColor = bg;
        preview.style.color = fg;
    }
    if (badge) badge.textContent = themeName;
    if (varBg) varBg.textContent = bg;
    if (varFg) varFg.textContent = fg;
    if (varAccent) {
        varAccent.textContent = accent;
        varAccent.style.color = accent;
    }
};
</script>"#);

    rsx! {
        <section id="themes" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.12"</span>
                <span class="t" data-i18n="comp.themes.sec">"Multi-Theme System"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add theme-selector"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.themes.title">"8 built-in themes with zero-FOUC persistence."</h2>
                <button
                    onclick="copyText(this, 'yav add theme-selector')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add theme-selector"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.themes.lede">
                "Complete zero-dependency theme engine with 8 calibrated design systems, client-side persistence in localStorage and cookies, and an inline script that executes before HTML body parsing to guarantee zero flash of unstyled content (FOUC)."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        <span data-i18n="comp.themes.preview_head">"LIVE PREVIEW — 8 THEME PRESETS"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.themes.preview_sub">"click to test palette"</span>
                </div>
                <div class="p-6 sm:p-8 space-y-6 bg-ink-2">
                    // Theme selector chips
                    <div class="flex flex-wrap items-center gap-2 text-xs font-mono">
                        <button onclick="previewTheme('Editorial Blueprint', '#0b0b0c', '#ece9e3', '#e0592f')" class="px-3 py-1.5 bg-sunk border border-oxide text-paper rounded flex items-center gap-2 hover:border-oxide cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]"></span>
                            "Editorial Blueprint"
                        </button>
                        <button onclick="previewTheme('Obsidian Minimal', '#09090b', '#f4f4f5', '#6366f1')" class="px-3 py-1.5 bg-sunk border border-rule text-dim hover:text-paper rounded flex items-center gap-2 cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#6366f1]"></span>
                            "Obsidian Minimal"
                        </button>
                        <button onclick="previewTheme('Nordic Paper', '#f7f5f0', '#1c1b18', '#c85a32')" class="px-3 py-1.5 bg-sunk border border-rule text-dim hover:text-paper rounded flex items-center gap-2 cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#c85a32]"></span>
                            "Nordic Paper"
                        </button>
                        <button onclick="previewTheme('Cyber Terminal', '#050805', '#00ff66', '#ffb000')" class="px-3 py-1.5 bg-sunk border border-rule text-dim hover:text-paper rounded flex items-center gap-2 cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#00ff66]"></span>
                            "Cyber Terminal"
                        </button>
                        <button onclick="previewTheme('Solarized Lab', '#fdf6e3', '#002b36', '#2aa198')" class="px-3 py-1.5 bg-sunk border border-rule text-dim hover:text-paper rounded flex items-center gap-2 cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#2aa198]"></span>
                            "Solarized Lab"
                        </button>
                        <button onclick="previewTheme('Glass Aurora', '#070913', '#f1f5f9', '#8b5cf6')" class="px-3 py-1.5 bg-sunk border border-rule text-dim hover:text-paper rounded flex items-center gap-2 cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#8b5cf6]"></span>
                            "Glass Aurora"
                        </button>
                        <button onclick="previewTheme('Classic Light', '#ffffff', '#0f172a', '#2563eb')" class="px-3 py-1.5 bg-sunk border border-rule text-dim hover:text-paper rounded flex items-center gap-2 cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#2563eb]"></span>
                            "Classic Light"
                        </button>
                        <button onclick="previewTheme('Slate Dark', '#0f172a', '#f8fafc', '#38bdf8')" class="px-3 py-1.5 bg-sunk border border-rule text-dim hover:text-paper rounded flex items-center gap-2 cursor-pointer">
                            <span class="w-2.5 h-2.5 rounded-full bg-[#38bdf8]"></span>
                            "Slate Dark"
                        </button>
                    </div>

                    // Interactive theme preview canvas
                    <div id="theme-preview-canvas" class="p-6 rounded-xl border border-rule transition-all duration-300 space-y-4" style="background-color:#0b0b0c; color:#ece9e3;">
                        <div class="flex flex-wrap items-center justify-between gap-2 border-b border-rule/50 pb-3">
                            <div class="flex items-center gap-2 font-mono text-xs">
                                <span class="font-bold text-oxide" id="theme-active-name">"Editorial Blueprint"</span>
                                <span class="text-dim" data-i18n="comp.themes.active_lbl">"· active theme"</span>
                            </div>
                            <div class="flex flex-wrap items-center gap-3 font-mono text-[11px] text-dim">
                                <span>"--bg: "<span id="theme-var-bg" class="text-paper">"#0b0b0c"</span></span>
                                <span>"--fg: "<span id="theme-var-fg" class="text-paper">"#ece9e3"</span></span>
                                <span>"--accent: "<span id="theme-var-accent" class="text-oxide font-bold">"#e0592f"</span></span>
                            </div>
                        </div>

                        <div class="grid sm:grid-cols-3 gap-4 font-mono text-xs">
                            <div class="p-4 rounded-lg border border-rule/60 bg-sunk/50 space-y-2">
                                <span class="text-dim text-[11px]" data-i18n="comp.themes.card1_lbl">"Primary Card"</span>
                                <div class="font-bold text-sm">"124,500 req/s"</div>
                                <p class="text-[11px] text-dim" data-i18n="comp.themes.card1_sub">"High-concurrency Axum 0.8 engine"</p>
                            </div>
                            <div class="p-4 rounded-lg border border-rule/60 bg-sunk/50 space-y-2">
                                <span class="text-dim text-[11px]" data-i18n="comp.themes.card2_lbl">"Session Security"</span>
                                <div class="font-bold text-sm">"HMAC-SHA256"</div>
                                <p class="text-[11px] text-dim" data-i18n="comp.themes.card2_sub">"Constant-time tamper-proof cookies"</p>
                            </div>
                            <div class="p-4 rounded-lg border border-rule/60 bg-sunk/50 space-y-2">
                                <span class="text-dim text-[11px]" data-i18n="comp.themes.card3_lbl">"Client Payload"</span>
                                <div class="font-bold text-sm text-oxide" data-i18n="comp.themes.card3_val">"0 KB JavaScript"</div>
                                <p class="text-[11px] text-dim" data-i18n="comp.themes.card3_sub">"Zero hydration overhead"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // IDE Code Window
            <div class="ide-window">
                <div class="ide-head">
                    <div class="flex items-center h-full">
                        <div class="ide-tab">
                            {render_icon("moon", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/ThemeSelector.rs"
                        </div>
                    </div>
                    <button data-copy="use yav::ui::{Theme, ThemeSelector};\n\nrsx! {\n    <head>\n        {Theme::inject_all_themes_css()}\n    </head>\n    <ThemeSelector default_theme=&quot;editorial-blueprint&quot; />\n}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                    </button>
                </div>
                <div class="flex bg-[#0b0b0d]">
                    <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                        "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"
                    </div>
                    <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                        <div class="token-comment">"// Multi-Theme System with Zero-FOUC Injection"</div>
                        <div><span class="token-keyword">"use "</span>"yav::ui::{"<span class="token-type">"Theme"</span>", "<span class="token-type">"ThemeSelector"</span>"};"</div>
                        <div class="pt-1"><span class="token-tag">"<head>"</span></div>
                        <div>"    {"<span class="token-type">"Theme"</span>"::"<span class="token-fn">"inject_all_themes_css"</span>"()}"</div>
                        <div>"    "<span class="token-tag">"<link "</span><span class="token-attr">"rel="</span><span class="token-string">"\"stylesheet\" "</span><span class="token-attr">"href="</span><span class="token-string">"\"/public/styles.css\" "</span><span class="token-tag">"/>"</span></div>
                        <div><span class="token-tag">"</head>"</span></div>
                        <div class="pt-1"><span class="token-tag">"<ThemeSelector "</span><span class="token-attr">"default_theme="</span><span class="token-string">"\"editorial-blueprint\" "</span><span class="token-tag">"/>"</span></div>
                    </div>
                </div>
                <div class="ide-status">
                    <span data-i18n="comp.themes.ide_l">"8 themes · localStorage & cookie sync"</span>
                    <span class="text-ok" data-i18n="comp.themes.ide_r">"Zero Flash of Unstyled Content"</span>
                </div>
            </div>

            {script}
        </section>
    }
}
