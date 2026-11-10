//! Multi-language switcher component — Editorial Blueprint.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn LangSwitcher() -> Node {
    rsx! {
        <div class="hidden lg:flex relative self-stretch items-center text-left" id="lang-switcher-container">
            <button
                id="lang-btn"
                onclick="window.toggleLangMenu(event)"
                class="hidden lg:inline-flex items-center gap-1 sm:gap-1.5 h-[34px] px-2 sm:px-2.5 border border-rule hover:border-oxide/50 text-[12px] font-mono text-paper-2 hover:text-paper transition-colors cursor-pointer bg-ink-2 shrink-0"
                title="Select Language"
                data-i18n-title="lang.select_title"
                aria-label="Select Language"
                data-i18n-aria-label="lang.select_title"
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
                    <span data-i18n="lang.menu_title">"Select Locale"</span>
                    <span class="font-mono text-[9px] text-oxide">"i18n"</span>
                </div>

                <div class="p-1 space-y-0.5">
                    <button
                        onclick="setLanguage('en')"
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded border border-transparent"
                        data-locale="en"
                    >
                        <span class="flex items-center gap-2.5">
                            <span class="text-xs font-semibold text-paper">"US"</span>
                            <span>"English"</span>
                        </span>
                        <span class="text-[10px] font-mono text-oxide font-bold" data-i18n="lang.default_badge">"DEFAULT"</span>
                    </button>

                    <button
                        onclick="setLanguage('fr')"
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded border border-transparent"
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
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded border border-transparent"
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
                        class="lang-option w-full flex items-center justify-between px-3 py-2 text-paper-2 hover:bg-[rgba(236,233,227,0.06)] hover:text-paper cursor-pointer transition-colors text-left font-mono text-xs rounded border border-transparent"
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
                {r##"
                window.__yavSupportedLangs = ['en', 'fr', 'es', 'de'];

                function getI18nDict() {
                    if (window.__yavI18nDict) return window.__yavI18nDict;
                    var i18nScript = document.getElementById('yav-i18n-payload');
                    if (!i18nScript) return null;
                    try {
                        window.__yavI18nDict = JSON.parse(i18nScript.textContent || '{}');
                        return window.__yavI18nDict;
                    } catch (e) {
                        console.error('Failed to parse i18n payload', e);
                        return null;
                    }
                }

                function normalizeLang(lang) {
                    if (window.__yavSupportedLangs.indexOf(lang) !== -1) return lang;
                    return 'en';
                }

                window.yavT = function(key, fallback) {
                    var lang = normalizeLang(window.yavCurrentLang || localStorage.getItem('yav_lang') || 'en');
                    var dict = getI18nDict();
                    if (dict) {
                        var tr = dict[lang];
                        if (tr && Object.prototype.hasOwnProperty.call(tr, key)) return tr[key];
                        var en = dict['en'];
                        if (en && Object.prototype.hasOwnProperty.call(en, key)) return en[key];
                    }
                    return fallback !== undefined ? fallback : key;
                };

                function routeTitleKey() {
                    var p = (location.pathname || '/').replace(/\/index\.html$/, '/').replace(/\.html$/, '').replace(/\/$/, '') || '/';
                    var map = {
                        '/': 'meta.title.home',
                        '/about': 'meta.title.about',
                        '/docs': 'meta.title.docs',
                        '/benchmarks': 'meta.title.benchmarks',
                        '/components': 'meta.title.components',
                        '/showcase': 'meta.title.showcase',
                        '/faq': 'meta.title.faq',
                        '/releases': 'meta.title.releases',
                        '/privacy': 'meta.title.privacy',
                        '/404': 'meta.title.not_found'
                    };
                    return map[p] || null;
                }

                function syncLocaleButtons(lang) {
                    document.querySelectorAll('#lang-menu button[data-locale], #mobile-nav button[data-locale]').forEach(function(btn) {
                        var active = btn.getAttribute('data-locale') === lang;
                        btn.setAttribute('aria-pressed', active ? 'true' : 'false');
                        btn.classList.toggle('border-oxide', active);
                        btn.classList.toggle('bg-sunk', active);
                        btn.classList.toggle('text-paper', active);
                    });
                }

                function setLanguage(lang) {
                    lang = normalizeLang(lang);
                    try {
                        localStorage.setItem('yav_lang', lang);
                        document.cookie = 'yav_lang=' + lang + '; path=/; max-age=31536000';
                    } catch (_) {}
                    applyLanguage(lang, false);
                    if (window.closeLangMenu) window.closeLangMenu();
                }
                window.setLanguage = setLanguage;
                window.applyLanguage = applyLanguage;

                function applyLanguage(lang, isInitial) {
                    lang = normalizeLang(lang);
                    window.yavCurrentLang = lang;

                    var currentLabel = document.getElementById('lang-current-label');
                    if (currentLabel) currentLabel.textContent = lang.toUpperCase();
                    var mobileLabel = document.getElementById('mobile-lang-current');
                    if (mobileLabel) mobileLabel.textContent = lang.toUpperCase();
                    document.documentElement.lang = lang;
                    syncLocaleButtons(lang);

                    if (isInitial && lang === 'en') {
                        window.dispatchEvent(new CustomEvent('yav:langchange', { detail: { lang: lang } }));
                        return;
                    }

                    var dict = getI18nDict();
                    if (!dict) return;
                    var translations = dict[lang] || dict['en'];
                    if (!translations) return;

                    document.querySelectorAll('[data-i18n]').forEach(function(el) {
                        var key = el.getAttribute('data-i18n');
                        if (!Object.prototype.hasOwnProperty.call(el, '_yavI18nOrig')) {
                            el._yavI18nOrig = el.textContent;
                        }
                        if (Object.prototype.hasOwnProperty.call(translations, key)) {
                            el.textContent = translations[key];
                        } else if (lang === 'en') {
                            el.textContent = el._yavI18nOrig;
                        }
                        if (el.hasAttribute('data-copy-label')) {
                            el.removeAttribute('data-prev-text');
                        }
                    });

                    document.querySelectorAll('[data-i18n-placeholder]').forEach(function(el) {
                        var key = el.getAttribute('data-i18n-placeholder');
                        if (!Object.prototype.hasOwnProperty.call(el, '_yavI18nOrigPh')) {
                            el._yavI18nOrigPh = el.placeholder || '';
                        }
                        if (Object.prototype.hasOwnProperty.call(translations, key)) {
                            el.placeholder = translations[key];
                        } else if (lang === 'en') {
                            el.placeholder = el._yavI18nOrigPh;
                        }
                    });

                    document.querySelectorAll('[data-i18n-title]').forEach(function(el) {
                        var key = el.getAttribute('data-i18n-title');
                        if (!Object.prototype.hasOwnProperty.call(el, '_yavI18nOrigTitle')) {
                            el._yavI18nOrigTitle = el.getAttribute('title') || '';
                        }
                        if (Object.prototype.hasOwnProperty.call(translations, key)) {
                            el.setAttribute('title', translations[key]);
                        } else if (lang === 'en') {
                            el.setAttribute('title', el._yavI18nOrigTitle);
                        }
                    });

                    document.querySelectorAll('[data-i18n-aria-label]').forEach(function(el) {
                        var key = el.getAttribute('data-i18n-aria-label');
                        if (!Object.prototype.hasOwnProperty.call(el, '_yavI18nOrigAria')) {
                            el._yavI18nOrigAria = el.getAttribute('aria-label') || '';
                        }
                        if (Object.prototype.hasOwnProperty.call(translations, key)) {
                            el.setAttribute('aria-label', translations[key]);
                        } else if (lang === 'en') {
                            el.setAttribute('aria-label', el._yavI18nOrigAria);
                        }
                    });

                    var tKey = routeTitleKey();
                    if (tKey && Object.prototype.hasOwnProperty.call(translations, tKey)) {
                        document.title = translations[tKey];
                    }

                    window.dispatchEvent(new CustomEvent('yav:langchange', { detail: { lang: lang } }));
                }

                (function() {
                    function init() {
                        var savedLang = 'en';
                        try {
                            savedLang = normalizeLang(localStorage.getItem('yav_lang') || 'en');
                        } catch (_) {}
                        applyLanguage(savedLang, true);
                    }
                    if (document.readyState === 'loading') {
                        document.addEventListener('DOMContentLoaded', init);
                    } else {
                        init();
                    }
                })();
                "##}
            </script>
        </div>
    }
}
