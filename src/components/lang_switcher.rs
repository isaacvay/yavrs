//! Multi-language switcher component for Yav.rs.
//!
//! Provides a dropdown to switch between English (default), French, Spanish, and German.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn LangSwitcher() -> Node {
    rsx! {
        <div class="relative inline-block text-left" id="lang-switcher-container">
            <button
                id="lang-btn"
                onclick="toggleLangMenu()"
                class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg bg-zinc-100 hover:bg-zinc-200/80 dark:bg-zinc-800/80 dark:hover:bg-zinc-700/80 text-xs font-mono text-zinc-700 dark:text-zinc-300 border border-zinc-200 dark:border-zinc-700 transition-all cursor-pointer shadow-xs active:scale-95"
                title="Select Language (Default: English)"
            >
                {render_icon("globe", IconSize::Sm, Some("w-3.5 h-3.5 text-zinc-400"), None)}
                <span id="lang-current-label" class="font-bold">"EN"</span>
                {render_icon("chevron-down", IconSize::Sm, Some("w-3 h-3 opacity-60 ml-0.5"), None)}
            </button>

            // Dropdown Menu
            <div
                id="lang-menu"
                class="hidden absolute right-0 mt-2 w-44 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-xl py-1 text-xs font-sans z-50 overflow-hidden transform opacity-0 scale-95 transition-all"
            >
                <div class="px-3 py-1.5 text-[10px] font-mono uppercase tracking-wider text-zinc-400 font-semibold border-b border-zinc-100 dark:border-zinc-800/80">
                    "Select Language"
                </div>

                <button
                    onclick="setLanguage('en')"
                    class="lang-option w-full flex items-center justify-between px-3 py-2 text-zinc-700 dark:text-zinc-200 hover:bg-zinc-100 dark:hover:bg-zinc-800 cursor-pointer transition-colors text-left font-mono"
                    data-locale="en"
                >
                    <span class="flex items-center gap-2">
                        <span>"🇺🇸"</span>
                        <span class="font-medium">"English"</span>
                    </span>
                    <span class="text-[10px] font-mono text-amber-500 font-bold">"DEFAULT"</span>
                </button>

                <button
                    onclick="setLanguage('fr')"
                    class="lang-option w-full flex items-center justify-between px-3 py-2 text-zinc-700 dark:text-zinc-200 hover:bg-zinc-100 dark:hover:bg-zinc-800 cursor-pointer transition-colors text-left font-mono"
                    data-locale="fr"
                >
                    <span class="flex items-center gap-2">
                        <span>"🇫🇷"</span>
                        <span>"Français"</span>
                    </span>
                    <span class="text-[10px] font-mono text-zinc-400">"FR"</span>
                </button>

                <button
                    onclick="setLanguage('es')"
                    class="lang-option w-full flex items-center justify-between px-3 py-2 text-zinc-700 dark:text-zinc-200 hover:bg-zinc-100 dark:hover:bg-zinc-800 cursor-pointer transition-colors text-left font-mono"
                    data-locale="es"
                >
                    <span class="flex items-center gap-2">
                        <span>"🇪🇸"</span>
                        <span>"Español"</span>
                    </span>
                    <span class="text-[10px] font-mono text-zinc-400">"ES"</span>
                </button>

                <button
                    onclick="setLanguage('de')"
                    class="lang-option w-full flex items-center justify-between px-3 py-2 text-zinc-700 dark:text-zinc-200 hover:bg-zinc-100 dark:hover:bg-zinc-800 cursor-pointer transition-colors text-left font-mono"
                    data-locale="de"
                >
                    <span class="flex items-center gap-2">
                        <span>"🇩🇪"</span>
                        <span>"Deutsch"</span>
                    </span>
                    <span class="text-[10px] font-mono text-zinc-400">"DE"</span>
                </button>
            </div>
        </div>

        <script>
            {r#"
            function toggleLangMenu() {
                const menu = document.getElementById('lang-menu');
                if (menu.classList.contains('hidden')) {
                    menu.classList.remove('hidden');
                    setTimeout(() => {
                        menu.classList.remove('opacity-0', 'scale-95');
                        menu.classList.add('opacity-100', 'scale-100');
                    }, 10);
                } else {
                    closeLangMenu();
                }
            }

            function closeLangMenu() {
                const menu = document.getElementById('lang-menu');
                if (!menu) return;
                menu.classList.remove('opacity-100', 'scale-100');
                menu.classList.add('opacity-0', 'scale-95');
                setTimeout(() => {
                    menu.classList.add('hidden');
                }, 120);
            }

            function setLanguage(lang) {
                localStorage.setItem('yav_lang', lang);
                document.cookie = 'yav_lang=' + lang + '; path=/; max-age=31536000';
                applyLanguage(lang);
                closeLangMenu();
            }

            function applyLanguage(lang) {
                const currentLabel = document.getElementById('lang-current-label');
                if (currentLabel) {
                    currentLabel.innerText = lang.toUpperCase();
                }
                document.documentElement.lang = lang;

                const i18nScript = document.getElementById('yav-i18n-payload');
                if (!i18nScript) return;

                try {
                    const dict = JSON.parse(i18nScript.textContent);
                    const translations = dict[lang] || dict['en'];
                    if (!translations) return;

                    document.querySelectorAll('[data-i18n]').forEach(el => {
                        const key = el.getAttribute('data-i18n');
                        if (translations[key]) {
                            el.innerText = translations[key];
                        }
                    });

                    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
                        const key = el.getAttribute('data-i18n-placeholder');
                        if (translations[key]) {
                            el.placeholder = translations[key];
                        }
                    });
                } catch(e) {
                    console.error('Failed to parse i18n payload', e);
                }
            }

            // Close on click outside
            document.addEventListener('click', (e) => {
                const container = document.getElementById('lang-switcher-container');
                if (container && !container.contains(e.target)) {
                    closeLangMenu();
                }
            });

            // Initialize on load
            document.addEventListener('DOMContentLoaded', () => {
                const savedLang = localStorage.getItem('yav_lang') || 'en';
                applyLanguage(savedLang);
            });
            "#}
        </script>
    }
}
