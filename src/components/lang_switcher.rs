//! Multi-language switcher component — Editorial Blueprint.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn LangSwitcher() -> Node {
    rsx! {
        <div class="relative self-stretch flex items-center text-left" id="lang-switcher-container">
            <button
                id="lang-btn"
                onclick="window.toggleLangMenu()"
                class="inline-flex items-center gap-1.5 h-[34px] px-2.5 border border-rule hover:border-oxide/50 text-[12px] font-mono text-paper-2 hover:text-paper transition-colors cursor-pointer bg-ink-2"
                title="Select Language"
                aria-expanded="false"
                aria-haspopup="true"
            >
                {render_icon("globe", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                <span id="lang-current-label" class="font-bold">"EN"</span>
                {render_icon("chevron-down", IconSize::Sm, Some("w-3 h-3 text-dim"), None)}
            </button>

            // Dropdown Menu — strictly opens right below the 64px navbar
            <div
                id="lang-menu"
                class="popover-dropdown lang-popover plate bg-ink-2 shadow-[0_20px_50px_rgba(0,0,0,0.95)] border border-rule overflow-hidden"
                onclick="event.stopPropagation()"
            >
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="px-3 py-2 label text-[10px] border-b border-rule bg-sunk flex items-center justify-between">
                    <span>"Select Locale"</span>
                    <span class="font-mono text-[9px] text-oxide">"i18n"</span>
                </div>

                <div class="p-1 space-y-0.5">
                    <button
                        onclick="setLanguage('en')"
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded"
                        data-locale="en"
                    >
                        <span class="flex items-center gap-2.5">
                            <span class="text-xs font-semibold text-paper">"US"</span>
                            <span>"English"</span>
                        </span>
                        <span class="text-[10px] font-mono text-oxide font-bold">"DEFAULT"</span>
                    </button>

                    <button
                        onclick="setLanguage('fr')"
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded"
                        data-locale="fr"
                    >
                        <span class="flex items-center gap-2.5">
                            <span class="text-xs font-semibold text-paper">"FR"</span>
                            <span>"Français"</span>
                        </span>
                        <span class="text-[10px] font-mono text-dim">"fr"</span>
                    </button>

                    <button
                        onclick="setLanguage('es')"
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded"
                        data-locale="es"
                    >
                        <span class="flex items-center gap-2.5">
                            <span class="text-xs font-semibold text-paper">"ES"</span>
                            <span>"Español"</span>
                        </span>
                        <span class="text-[10px] font-mono text-dim">"es"</span>
                    </button>

                    <button
                        onclick="setLanguage('de')"
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded"
                        data-locale="de"
                    >
                        <span class="flex items-center gap-2.5">
                            <span class="text-xs font-semibold text-paper">"DE"</span>
                            <span>"Deutsch"</span>
                        </span>
                        <span class="text-[10px] font-mono text-dim">"de"</span>
                    </button>
                </div>
            </div>

            <script>
                {r#"
                function setLanguage(lang) {
                    localStorage.setItem('yav_lang', lang);
                    document.cookie = 'yav_lang=' + lang + '; path=/; max-age=31536000';
                    applyLanguage(lang);
                    if (window.closeLangMenu) window.closeLangMenu();
                }

                function applyLanguage(lang) {
                    var currentLabel = document.getElementById('lang-current-label');
                    if (currentLabel) {
                        currentLabel.innerText = lang.toUpperCase();
                    }
                    document.documentElement.lang = lang;

                    var i18nScript = document.getElementById('yav-i18n-payload');
                    if (!i18nScript) return;

                    try {
                        var dict = JSON.parse(i18nScript.textContent);
                        var translations = dict[lang] || dict['en'];
                        if (!translations) return;

                        document.querySelectorAll('[data-i18n]').forEach(function(el) {
                            var key = el.getAttribute('data-i18n');
                            if (translations[key]) {
                                el.innerText = translations[key];
                            }
                        });

                        document.querySelectorAll('[data-i18n-placeholder]').forEach(function(el) {
                            var key = el.getAttribute('data-i18n-placeholder');
                            if (translations[key]) {
                                el.placeholder = translations[key];
                            }
                        });
                    } catch(e) {
                        console.error('Failed to parse i18n payload', e);
                    }
                }

                (function() {
                    function init() {
                        var savedLang = localStorage.getItem('yav_lang') || 'en';
                        applyLanguage(savedLang);
                    }
                    if (document.readyState === 'loading') {
                        document.addEventListener('DOMContentLoaded', init);
                    } else {
                        init();
                    }
                })();
                "#}
            </script>
        </div>
    }
}
