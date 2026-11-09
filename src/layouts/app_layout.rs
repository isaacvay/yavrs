//! Application layout — Editorial Blueprint, dark only.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use crate::components::*;
use crate::i18n::get_i18n_json;

#[derive(Clone, Default)]
pub struct AppLayoutProps {
    pub children: Vec<Node>,
}

/// Helper to construct a production-ready `HtmlPage` with full SEO, OpenGraph, Twitter Cards,
/// canonical link, theme colors, and JSON-LD structured data.
pub fn website_page(
    title: impl Into<String>,
    description: impl Into<String>,
    path: impl AsRef<str>,
    body: impl Into<Node>,
) -> yav::server::HtmlPage {
    let title_str = title.into();
    let desc_str = description.into();
    let path_str = path.as_ref();
    let canonical = format!("https://yav.rs{path_str}");

    let mut page = yav::server::HtmlPage::new(body)
        .title(&title_str)
        .favicon("/public/favicon.svg");

    let head_tags = format!(
        r##"<meta name="description" content="{desc_str}" />
<link rel="canonical" href="{canonical}" />
<meta property="og:type" content="website" />
<meta property="og:url" content="{canonical}" />
<meta property="og:title" content="{title_str}" />
<meta property="og:description" content="{desc_str}" />
<meta property="og:image" content="https://yav.rs/public/og-banner.png" />
<meta property="og:site_name" content="Yav.rs" />
<meta name="twitter:card" content="summary_large_image" />
<meta name="twitter:url" content="{canonical}" />
<meta name="twitter:title" content="{title_str}" />
<meta name="twitter:description" content="{desc_str}" />
<meta name="twitter:image" content="https://yav.rs/public/og-banner.png" />
<meta name="theme-color" content="#0b0b0d" />
<meta name="color-scheme" content="dark" />
<script type="application/ld+json">
{{
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "Yav",
  "applicationCategory": "DeveloperApplication",
  "operatingSystem": "Linux, macOS, Windows",
  "offers": {{
    "@type": "Offer",
    "price": "0",
    "priceCurrency": "USD"
  }},
  "url": "https://yav.rs",
  "description": "{desc_str}"
}}
</script>"##
    );
    page.head_extra.push_str(&head_tags);
    page
}

#[component]
pub fn AppLayout(props: AppLayoutProps) -> Node {
    let enforce_dark = Node::raw(r#"<script>
document.documentElement.classList.add('dark');
window.copyText = function (btn, text) {
  var str = text;
  if (!str || typeof str !== 'string') {
    str = btn.getAttribute('data-copy');
  }
  if (!str) {
    var win = btn.closest ? btn.closest('.ide-window, .term-window, .plate') : null;
    if (win) {
      var c = win.querySelector('.code');
      if (c) str = c.innerText || c.textContent;
    }
  }
  if (!str && btn.parentElement) {
    var codeEl = btn.parentElement.querySelector('code');
    if (codeEl) str = codeEl.innerText || codeEl.textContent;
  }
  if (!str) str = '';

  function fallback(t) {
    var ta = document.createElement('textarea');
    ta.value = t;
    ta.style.position = 'fixed';
    ta.style.left = '-9999px';
    ta.style.opacity = '0';
    document.body.appendChild(ta);
    ta.focus();
    ta.select();
    try { document.execCommand('copy'); } catch (e) {}
    document.body.removeChild(ta);
  }

  try {
    if (navigator.clipboard && window.isSecureContext) {
      navigator.clipboard.writeText(str).catch(function () { fallback(str); });
    } else {
      fallback(str);
    }
  } catch (err) {
    fallback(str);
  }

  var el = btn.querySelector ? btn.querySelector('[data-copy-label]') : null;
  if (!el) el = btn;
  var svg = btn.querySelector ? btn.querySelector('svg') : null;
  var prev = el.getAttribute('data-prev-text') || el.textContent;
  el.setAttribute('data-prev-text', prev);

  el.textContent = 'copied ✓';
  el.classList.add('copied');
  btn.classList.add('is-copied');
  el.style.setProperty('color', '#22c55e', 'important');
  el.style.fontWeight = '600';
  btn.style.setProperty('border-color', 'rgba(34, 197, 94, 0.6)', 'important');
  if (svg) {
    svg.style.setProperty('color', '#22c55e', 'important');
    svg.style.setProperty('stroke', '#22c55e', 'important');
  }

  clearTimeout(btn._copyTimer);
  btn._copyTimer = setTimeout(function () { 
    el.textContent = prev; 
    el.classList.remove('copied');
    btn.classList.remove('is-copied');
    el.style.removeProperty('color');
    el.style.fontWeight = '';
    btn.style.removeProperty('border-color');
    if (svg) {
      svg.style.removeProperty('color');
      svg.style.removeProperty('stroke');
    }
  }, 1600);
};
</script>"#);
    let embedded_css = Node::raw(r#"<style>
/* Guaranteed popover positioning right below the 64px header */
#header-search-container, #lang-switcher-container {
  align-self: stretch !important;
  display: flex !important;
  align-items: center !important;
  position: relative !important;
}

#cmd-palette-popover, #lang-menu {
  display: none;
  position: absolute !important;
  top: 100% !important;
  margin-top: 8px !important;
  z-index: 9999 !important;
}

@media (max-width: 639px) {
  #cmd-palette-popover {
    position: fixed !important;
    top: 72px !important;
    left: 12px !important;
    right: 12px !important;
    width: auto !important;
    max-width: none !important;
  }
  #lang-menu {
    position: fixed !important;
    top: 72px !important;
    right: 12px !important;
    width: 220px !important;
    max-width: calc(100vw - 24px) !important;
  }
}

@media (min-width: 640px) {
  #cmd-palette-popover {
    position: absolute !important;
    top: 100% !important;
    right: 0 !important;
    margin-top: 8px !important;
    width: 520px !important;
  }
  #lang-menu {
    position: absolute !important;
    top: 100% !important;
    right: 0 !important;
    margin-top: 8px !important;
    width: 208px !important;
  }
}

/* Guaranteed Vivid Emerald Green on copied state */
[data-copy-label] {
  transition: all 0.2s cubic-bezier(0.2, 0.8, 0.2, 1);
}
[data-copy-label].copied,
button:hover [data-copy-label].copied,
button [data-copy-label].copied,
button.is-copied [data-copy-label],
button.is-copied:hover [data-copy-label] {
  color: #22c55e !important;
  font-weight: 600 !important;
}
button.is-copied,
button.is-copied:hover {
  border-color: rgba(34, 197, 94, 0.6) !important;
  background-color: rgba(34, 197, 94, 0.08) !important;
  color: #22c55e !important;
}
button.is-copied svg,
button.is-copied:hover svg {
  color: #22c55e !important;
  stroke: #22c55e !important;
}
</style>"#);
    let i18n_payload = get_i18n_json();

    let ui_script = Node::raw(r##"<script>
(function () {
  var path = location.pathname.replace(/\/$/, '') || '/';
  document.querySelectorAll('[data-nav]').forEach(function (a) {
    if (a.getAttribute('href') === path) a.setAttribute('aria-current', 'page');
  });

  var header = document.getElementById('site-header');
  var bar = document.getElementById('scroll-progress');
  function onScroll() {
    var y = window.scrollY || 0;
    var max = document.documentElement.scrollHeight - window.innerHeight;
    if (header) {
      if (y > 8) {
        header.classList.add('is-scrolled');
      } else {
        header.classList.remove('is-scrolled');
      }
    }
    if (bar && max > 0) {
      bar.style.setProperty('--progress', Math.min(y / max, 1));
    }
  }
  window.addEventListener('scroll', onScroll, { passive: true });
  onScroll();

  // Scroll reveal observer
  if ('IntersectionObserver' in window) {
    var revObs = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (e.isIntersecting) {
          e.target.classList.add('is-in');
        }
      });
    }, { threshold: 0.1 });
    document.querySelectorAll('.reveal').forEach(function (el) {
      revObs.observe(el);
    });
  } else {
    document.querySelectorAll('.reveal').forEach(function (el) {
      el.classList.add('is-in');
    });
  }

  // Mobile Nav & Dropdown Menu Controller
  var mBtn = document.getElementById('mobile-menu-btn');
  var mNav = document.getElementById('mobile-nav');
  var mOpenIcon = document.getElementById('menu-icon-open');
  var mCloseIcon = document.getElementById('menu-icon-close');

  window.toggleMobileNav = function (e) {
    if (e && e.stopPropagation) e.stopPropagation();
    if (!mNav) mNav = document.getElementById('mobile-nav');
    if (!mBtn) mBtn = document.getElementById('mobile-menu-btn');
    if (!mOpenIcon) mOpenIcon = document.getElementById('menu-icon-open');
    if (!mCloseIcon) mCloseIcon = document.getElementById('menu-icon-close');
    if (!mNav) return;

    if (window.closeCmdPalette) window.closeCmdPalette();
    if (window.closeLangMenu) window.closeLangMenu();

    var isHidden = mNav.classList.contains('hidden');
    if (isHidden) {
      mNav.classList.remove('hidden');
      document.body.classList.add('mobile-nav-open');
      if (mBtn) mBtn.setAttribute('aria-expanded', 'true');
      if (mOpenIcon) { mOpenIcon.classList.add('hidden'); mOpenIcon.classList.remove('inline-flex'); }
      if (mCloseIcon) { mCloseIcon.classList.remove('hidden'); mCloseIcon.classList.add('inline-flex'); }
      requestAnimationFrame(function () {
        mNav.classList.remove('opacity-0', '-translate-y-2');
        mNav.classList.add('opacity-100', 'translate-y-0');
      });
    } else {
      window.closeMobileNav();
    }
  };

  window.closeMobileNav = function () {
    if (!mNav) mNav = document.getElementById('mobile-nav');
    if (!mBtn) mBtn = document.getElementById('mobile-menu-btn');
    if (!mOpenIcon) mOpenIcon = document.getElementById('menu-icon-open');
    if (!mCloseIcon) mCloseIcon = document.getElementById('menu-icon-close');
    if (!mNav || mNav.classList.contains('hidden')) return;

    document.body.classList.remove('mobile-nav-open');
    mNav.classList.remove('opacity-100', 'translate-y-0');
    mNav.classList.add('opacity-0', '-translate-y-2');
    if (mBtn) mBtn.setAttribute('aria-expanded', 'false');
    if (mOpenIcon) { mOpenIcon.classList.remove('hidden'); mOpenIcon.classList.add('inline-flex'); }
    if (mCloseIcon) { mCloseIcon.classList.add('hidden'); mCloseIcon.classList.remove('inline-flex'); }
    setTimeout(function () {
      mNav.classList.add('hidden');
    }, 180);
  };

  if (mBtn) {
    mBtn.addEventListener('click', window.toggleMobileNav);
  }

  // Command Palette Popover
  window.toggleCmdPalette = function (e) {
    if (e && e.stopPropagation) e.stopPropagation();
    var p = document.getElementById('cmd-palette-popover');
    if (!p) return;
    if (p.style.display === 'block' || p.classList.contains('is-open')) {
      window.closeCmdPalette();
    } else {
      window.openCmdPalette();
    }
  };

  window.openCmdPalette = function () {
    var p = document.getElementById('cmd-palette-popover');
    var btn = document.getElementById('header-search-btn');
    if (!p) return;
    window.closeLangMenu();
    p.classList.add('is-open');
    p.style.display = 'block';
    p.style.opacity = '1';
    p.style.visibility = 'visible';
    p.style.pointerEvents = 'auto';
    if (btn) btn.setAttribute('aria-expanded', 'true');
    var input = document.getElementById('cmd-palette-input');
    if (input) {
      setTimeout(function () { input.focus(); input.select(); }, 40);
    }
  };

  window.closeCmdPalette = function () {
    var p = document.getElementById('cmd-palette-popover');
    var btn = document.getElementById('header-search-btn');
    if (!p) return;
    p.classList.remove('is-open');
    p.style.display = 'none';
    p.style.opacity = '0';
    p.style.visibility = 'hidden';
    p.style.pointerEvents = 'none';
    if (btn) btn.setAttribute('aria-expanded', 'false');
  };

  // Language Menu Popover
  window.toggleLangMenu = function (e) {
    if (e && e.stopPropagation) e.stopPropagation();
    var m = document.getElementById('lang-menu');
    if (!m) return;
    if (m.style.display === 'block' || m.classList.contains('is-open')) {
      window.closeLangMenu();
    } else {
      window.openLangMenu();
    }
  };

  window.openLangMenu = function () {
    var m = document.getElementById('lang-menu');
    var btn = document.getElementById('lang-btn');
    if (!m) return;
    window.closeCmdPalette();
    m.classList.add('is-open');
    m.style.display = 'block';
    m.style.opacity = '1';
    m.style.visibility = 'visible';
    m.style.pointerEvents = 'auto';
    if (btn) btn.setAttribute('aria-expanded', 'true');
  };

  window.closeLangMenu = function () {
    var m = document.getElementById('lang-menu');
    var btn = document.getElementById('lang-btn');
    if (!m) return;
    m.classList.remove('is-open');
    m.style.display = 'none';
    m.style.opacity = '0';
    m.style.visibility = 'hidden';
    m.style.pointerEvents = 'none';
    if (btn) btn.setAttribute('aria-expanded', 'false');
  };

  // Close menus on click-outside
  document.addEventListener('click', function (e) {
    var sBtn = document.getElementById('header-search-btn');
    var sPop = document.getElementById('cmd-palette-popover');
    if (sPop && (sPop.style.display === 'block' || sPop.classList.contains('is-open'))) {
      var insideSearch = (sBtn && (sBtn === e.target || sBtn.contains(e.target))) || (sPop && sPop.contains(e.target));
      if (!insideSearch) {
        window.closeCmdPalette();
      }
    }

    var lBtn = document.getElementById('lang-btn');
    var lPop = document.getElementById('lang-menu');
    if (lPop && (lPop.style.display === 'block' || lPop.classList.contains('is-open'))) {
      var insideLang = (lBtn && (lBtn === e.target || lBtn.contains(e.target))) || (lPop && lPop.contains(e.target));
      if (!insideLang) {
        window.closeLangMenu();
      }
    }

    var curNav = document.getElementById('mobile-nav');
    var curBtn = document.getElementById('mobile-menu-btn');
    if (curNav && !curNav.classList.contains('hidden')) {
      var insideNavBtn = curBtn && (curBtn === e.target || curBtn.contains(e.target));
      var insideNav = curNav && curNav.contains(e.target);
      if (!insideNavBtn && !insideNav) {
        window.closeMobileNav();
      }
    }
  });

  // Close menus on Escape key & Ctrl+K shortcut
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape') {
      window.closeCmdPalette();
      window.closeLangMenu();
      window.closeMobileNav();
    }
    if ((e.metaKey || e.ctrlKey) && e.key && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      window.toggleCmdPalette();
    }
  });

  // Reset mobile menu on resize to desktop
  window.addEventListener('resize', function () {
    if (window.innerWidth >= 1024) {
      window.closeMobileNav();
    }
  });

  // Count-up numbers with IntersectionObserver
  function runCountUp(el) {
    if (el._counted) return;
    el._counted = true;
    var raw = el.dataset.count;
    var to = parseFloat(raw);
    if (isNaN(to)) return;
    var parts = raw.split('.');
    var dec = parts[1] ? parts[1].length : 0;
    var t0 = performance.now();
    var duration = 1400;
    function step(t) {
      var p = Math.min((t - t0) / duration, 1);
      var k = 1 - Math.pow(1 - p, 3);
      el.textContent = (to * k).toLocaleString('en-US', { minimumFractionDigits: dec, maximumFractionDigits: dec });
      if (p < 1) {
        requestAnimationFrame(step);
      } else {
        el.textContent = to.toLocaleString('en-US', { minimumFractionDigits: dec, maximumFractionDigits: dec });
      }
    }
    requestAnimationFrame(step);
  }

  if ('IntersectionObserver' in window) {
    var countObs = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (e.isIntersecting) {
          runCountUp(e.target);
          countObs.unobserve(e.target);
        }
      });
    }, { threshold: 0.15 });
    document.querySelectorAll('[data-count]').forEach(function (el) {
      countObs.observe(el);
    });
  } else {
    document.querySelectorAll('[data-count]').forEach(runCountUp);
  }

  // Scroll-spy for .toc
  var toc = document.querySelector('.toc');
  if (toc) {
    var links = {};
    toc.querySelectorAll("a[href^='#']").forEach(function (a) {
      var href = a.getAttribute('href');
      if (href) links[href.slice(1)] = a;
    });
    if ('IntersectionObserver' in window) {
      var sio = new IntersectionObserver(function (es) {
        es.forEach(function (e) {
          if (e.isIntersecting) {
            Object.keys(links).forEach(function (k) {
              links[k].classList.toggle('is-active', k === e.target.id);
            });
          }
        });
      }, { rootMargin: '-20% 0px -60% 0px' });
      Object.keys(links).forEach(function (id) {
        var s = document.getElementById(id);
        if (s) sio.observe(s);
      });
    }
  }
})();

window.copyText = function (btn, text) {
  var str = text;
  if (!str || typeof str !== 'string') {
    str = btn.getAttribute('data-copy');
  }
  if (!str) {
    var win = btn.closest ? btn.closest('.ide-window, .term-window, .plate') : null;
    if (win) {
      var c = win.querySelector('.code');
      if (c) str = c.innerText || c.textContent;
    }
  }
  if (!str && btn.parentElement) {
    var codeEl = btn.parentElement.querySelector('code');
    if (codeEl) str = codeEl.innerText || codeEl.textContent;
  }
  if (!str) str = '';

  function fallback(t) {
    var ta = document.createElement('textarea');
    ta.value = t;
    ta.style.position = 'fixed';
    ta.style.left = '-9999px';
    ta.style.opacity = '0';
    document.body.appendChild(ta);
    ta.focus();
    ta.select();
    try { document.execCommand('copy'); } catch (e) {}
    document.body.removeChild(ta);
  }

  try {
    if (navigator.clipboard && window.isSecureContext) {
      navigator.clipboard.writeText(str).catch(function () { fallback(str); });
    } else {
      fallback(str);
    }
  } catch (err) {
    fallback(str);
  }

  var el = btn.querySelector ? btn.querySelector('[data-copy-label]') : null;
  if (!el) el = btn;
  var svg = btn.querySelector ? btn.querySelector('svg') : null;
  var prev = el.getAttribute('data-prev-text') || el.textContent;
  el.setAttribute('data-prev-text', prev);

  el.textContent = 'copied ✓';
  el.classList.add('copied');
  btn.classList.add('is-copied');
  el.style.setProperty('color', '#22c55e', 'important');
  el.style.fontWeight = '600';
  btn.style.setProperty('border-color', 'rgba(34, 197, 94, 0.6)', 'important');
  if (svg) {
    svg.style.setProperty('color', '#22c55e', 'important');
    svg.style.setProperty('stroke', '#22c55e', 'important');
  }

  clearTimeout(btn._copyTimer);
  btn._copyTimer = setTimeout(function () { 
    el.textContent = prev; 
    el.classList.remove('copied');
    btn.classList.remove('is-copied');
    el.style.removeProperty('color');
    el.style.fontWeight = '';
    btn.style.removeProperty('border-color');
    if (svg) {
      svg.style.removeProperty('color');
      svg.style.removeProperty('stroke');
    }
  }, 1600);
};
</script>"##);

    rsx! {
        <div class="page-frame min-h-screen flex flex-col font-sans antialiased">
            {enforce_dark}
            {embedded_css}
            <script id="yav-i18n-payload" type="application/json">
                {i18n_payload}
            </script>
            <div class="rails" aria-hidden="true"></div>

            <header id="site-header" class="site-header sticky top-0 z-40 w-full">
                <div class="wrap flex h-16 items-center justify-between gap-2 sm:gap-6">
                    <a href="/" class="flex items-center gap-2 sm:gap-2.5 shrink-0" aria-label="Yav home">
                        <img src="/public/favicon.svg" class="w-6 h-6 shrink-0" alt="" />
                        <span class="text-[15px] sm:text-[16px] font-semibold tracking-[-0.03em] text-paper">"yav"<span class="text-dim font-normal">".rs"</span></span>
                        <span class="hidden sm:inline-flex items-center px-1.5 py-0.5 text-[10px] font-mono font-medium text-oxide bg-oxide/10 border border-oxide/20 rounded">"v0.1.5"</span>
                    </a>

                    <nav class="hidden lg:flex items-center gap-0.5 xl:gap-2" aria-label="Primary">
                        <a href="/" data-nav="" class="nav-link"><span class="idx">"01"</span><span data-i18n="nav.overview">"Overview"</span></a>
                        <a href="/about" data-nav="" class="nav-link"><span class="idx">"02"</span><span data-i18n="nav.architecture">"Architecture"</span></a>
                        <a href="/docs" data-nav="" class="nav-link"><span class="idx">"03"</span><span data-i18n="nav.docs">"Docs"</span></a>
                        <a href="/benchmarks" data-nav="" class="nav-link"><span class="idx">"04"</span><span data-i18n="nav.benchmarks">"Benchmarks"</span></a>
                        <a href="/components" data-nav="" class="nav-link"><span class="idx">"05"</span><span data-i18n="nav.components">"Components"</span></a>
                    </nav>

                    <div class="self-stretch flex items-center gap-1.5 sm:gap-2">
                        <CommandPalette />
                        <LangSwitcher />
                        <a
                            id="header-github-link"
                            href="https://github.com/isaacvay/yav.rs"
                            target="_blank"
                            rel="noreferrer"
                            class="hidden sm:inline-flex items-center justify-center w-[34px] h-[34px] text-mute hover:text-paper transition-colors shrink-0"
                            title="GitHub"
                        >
                            {render_icon("minimal:github", IconSize::Sm, Some("w-4 h-4"), None)}
                        </a>
                        <a id="header-cta" href="/docs" class="btn btn-solid btn-sm hidden md:inline-flex shrink-0">"Get started"<span class="arr">"→"</span></a>
                        <button
                            id="mobile-menu-btn"
                            onclick="window.toggleMobileNav(event)"
                            class="lg:hidden inline-flex items-center justify-center w-[36px] h-[36px] text-paper border border-[rgba(236,233,227,0.14)] bg-ink-2 hover:border-oxide/50 hover:bg-ink-3 transition-colors cursor-pointer shrink-0"
                            aria-label="Toggle navigation menu"
                            aria-expanded="false"
                        >
                            <span id="menu-icon-open" class="inline-flex items-center justify-center">{render_icon("menu", IconSize::Sm, Some("w-4 h-4"), None)}</span>
                            <span id="menu-icon-close" class="hidden items-center justify-center">{render_icon("x", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}</span>
                        </button>
                    </div>
                </div>
                <nav
                    id="mobile-nav"
                    class="hidden lg:hidden fixed inset-x-0 top-16 bottom-0 z-50 bg-[#0b0b0d]/98 backdrop-blur-2xl border-t border-rule overflow-y-auto transition-all duration-200 opacity-0 -translate-y-2"
                    aria-label="Mobile Navigation"
                >
                    <div class="wrap py-6 space-y-6 max-h-full">
                        // Mobile Search Trigger
                        <button
                            onclick="window.closeMobileNav(); window.openCmdPalette();"
                            class="w-full flex items-center justify-between h-11 px-3.5 bg-ink-2 border border-rule text-dim text-xs font-mono cursor-pointer hover:border-oxide/50 transition-colors"
                        >
                            <span class="flex items-center gap-2.5">
                                {render_icon("search", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                                <span class="text-mute">"Search documentation & symbols..."</span>
                            </span>
                            <kbd class="kbd text-[10px]">"Ctrl K"</kbd>
                        </button>

                        // Group 1: Core Framework
                        <div class="space-y-2">
                            <div class="text-[10px] font-mono uppercase tracking-wider text-dim px-2">
                                "01 · Framework Architecture"
                            </div>
                            <div class="grid gap-1.5">
                                <a href="/" onclick="window.closeMobileNav()" data-nav="" class="flex items-center justify-between px-3.5 py-3 rounded plate bg-ink-2 hover:border-oxide/50 text-[14px] text-paper transition-colors group">
                                    <span class="flex items-center gap-3">
                                        <span class="font-mono text-[11px] text-oxide">"01"</span>
                                        <span class="font-medium" data-i18n="nav.overview">"Overview"</span>
                                    </span>
                                    <span class="text-dim text-xs group-hover:text-oxide transition-colors">"→"</span>
                                </a>
                                <a href="/about" onclick="window.closeMobileNav()" data-nav="" class="flex items-center justify-between px-3.5 py-3 rounded plate bg-ink-2 hover:border-oxide/50 text-[14px] text-paper transition-colors group">
                                    <span class="flex items-center gap-3">
                                        <span class="font-mono text-[11px] text-oxide">"02"</span>
                                        <span class="font-medium" data-i18n="nav.architecture">"Architecture & Tokio Engine"</span>
                                    </span>
                                    <span class="text-dim text-xs group-hover:text-oxide transition-colors">"→"</span>
                                </a>
                                <a href="/docs" onclick="window.closeMobileNav()" data-nav="" class="flex items-center justify-between px-3.5 py-3 rounded plate bg-ink-2 hover:border-oxide/50 text-[14px] text-paper transition-colors group">
                                    <span class="flex items-center gap-3">
                                        <span class="font-mono text-[11px] text-oxide">"03"</span>
                                        <span class="font-medium" data-i18n="nav.docs">"Engineering Documentation"</span>
                                    </span>
                                    <span class="text-dim text-xs group-hover:text-oxide transition-colors">"→"</span>
                                </a>
                                <a href="/benchmarks" onclick="window.closeMobileNav()" data-nav="" class="flex items-center justify-between px-3.5 py-3 rounded plate bg-ink-2 hover:border-oxide/50 text-[14px] text-paper transition-colors group">
                                    <span class="flex items-center gap-3">
                                        <span class="font-mono text-[11px] text-oxide">"04"</span>
                                        <span class="font-medium" data-i18n="nav.benchmarks">"Verified Benchmarks (124k req/s)"</span>
                                    </span>
                                    <span class="text-dim text-xs group-hover:text-oxide transition-colors">"→"</span>
                                </a>
                                <a href="/components" onclick="window.closeMobileNav()" data-nav="" class="flex items-center justify-between px-3.5 py-3 rounded plate bg-ink-2 hover:border-oxide/50 text-[14px] text-paper transition-colors group">
                                    <span class="flex items-center gap-3">
                                        <span class="font-mono text-[11px] text-oxide">"05"</span>
                                        <span class="font-medium" data-i18n="nav.components">"UI Components & Icons"</span>
                                    </span>
                                    <span class="text-dim text-xs group-hover:text-oxide transition-colors">"→"</span>
                                </a>
                            </div>
                        </div>

                        // Group 2: Ecosystem & Resources
                        <div class="space-y-2">
                            <div class="text-[10px] font-mono uppercase tracking-wider text-dim px-2">
                                "02 · Ecosystem & Resources"
                            </div>
                            <div class="grid grid-cols-2 gap-2 text-xs">
                                <a href="/showcase" onclick="window.closeMobileNav()" class="p-3 plate bg-ink-2 hover:border-oxide/50 block space-y-1 transition-colors group">
                                    <div class="flex items-center gap-2 text-paper font-semibold group-hover:text-oxide transition-colors">
                                        {render_icon("award", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                        <span>"Showcase"</span>
                                    </div>
                                    <p class="text-[11px] text-mute">"Production systems"</p>
                                </a>
                                <a href="/faq" onclick="window.closeMobileNav()" class="p-3 plate bg-ink-2 hover:border-oxide/50 block space-y-1 transition-colors group">
                                    <div class="flex items-center gap-2 text-paper font-semibold group-hover:text-oxide transition-colors">
                                        {render_icon("help-circle", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                        <span>"Why Yav?"</span>
                                    </div>
                                    <p class="text-[11px] text-mute">"Matrix & FAQs"</p>
                                </a>
                                <a href="/releases" onclick="window.closeMobileNav()" class="p-3 plate bg-ink-2 hover:border-oxide/50 block space-y-1 transition-colors group">
                                    <div class="flex items-center gap-2 text-paper font-semibold group-hover:text-ok transition-colors">
                                        {render_icon("git-branch", IconSize::Sm, Some("w-3.5 h-3.5 text-ok"), None)}
                                        <span>"Changelog"</span>
                                    </div>
                                    <p class="text-[11px] text-mute">"v0.1.5 stable"</p>
                                </a>
                                <a href="/privacy" onclick="window.closeMobileNav()" class="p-3 plate bg-ink-2 hover:border-oxide/50 block space-y-1 transition-colors group">
                                    <div class="flex items-center gap-2 text-paper font-semibold group-hover:text-paper transition-colors">
                                        {render_icon("shield-check", IconSize::Sm, Some("w-3.5 h-3.5 text-dim"), None)}
                                        <span>"Security"</span>
                                    </div>
                                    <p class="text-[11px] text-mute">"Zero tracking"</p>
                                </a>
                            </div>
                        </div>

                        // Group 3: Language Selector in Mobile Menu
                        <div class="space-y-2">
                            <div class="text-[10px] font-mono uppercase tracking-wider text-dim px-2">
                                "03 · Language / Locale"
                            </div>
                            <div class="grid grid-cols-4 gap-2 text-xs font-mono">
                                <button
                                    onclick="setLanguage('en'); window.closeMobileNav();"
                                    class="py-2.5 px-2 plate bg-ink-2 hover:border-oxide/50 text-paper text-center transition-colors cursor-pointer flex flex-col items-center gap-0.5"
                                    data-locale="en"
                                >
                                    <span class="font-bold text-[13px]">"EN"</span>
                                    <span class="text-[9px] text-dim">"English"</span>
                                </button>
                                <button
                                    onclick="setLanguage('fr'); window.closeMobileNav();"
                                    class="py-2.5 px-2 plate bg-ink-2 hover:border-oxide/50 text-paper text-center transition-colors cursor-pointer flex flex-col items-center gap-0.5"
                                    data-locale="fr"
                                >
                                    <span class="font-bold text-[13px]">"FR"</span>
                                    <span class="text-[9px] text-dim">"Français"</span>
                                </button>
                                <button
                                    onclick="setLanguage('es'); window.closeMobileNav();"
                                    class="py-2.5 px-2 plate bg-ink-2 hover:border-oxide/50 text-paper text-center transition-colors cursor-pointer flex flex-col items-center gap-0.5"
                                    data-locale="es"
                                >
                                    <span class="font-bold text-[13px]">"ES"</span>
                                    <span class="text-[9px] text-dim">"Español"</span>
                                </button>
                                <button
                                    onclick="setLanguage('de'); window.closeMobileNav();"
                                    class="py-2.5 px-2 plate bg-ink-2 hover:border-oxide/50 text-paper text-center transition-colors cursor-pointer flex flex-col items-center gap-0.5"
                                    data-locale="de"
                                >
                                    <span class="font-bold text-[13px]">"DE"</span>
                                    <span class="text-[9px] text-dim">"Deutsch"</span>
                                </button>
                            </div>
                        </div>

                        // Group 4: CTA & CLI Install
                        <div class="space-y-2.5 pt-1">
                            <a href="/docs" onclick="window.closeMobileNav()" class="btn btn-solid w-full justify-center h-11 text-sm font-semibold">
                                <span>"Get started with Yav"</span>
                                <span class="arr">"→"</span>
                            </a>

                            <div class="p-3.5 plate bg-ink-2 flex items-center justify-between text-xs font-mono">
                                <div class="flex items-center gap-2 overflow-hidden truncate">
                                    <span class="text-oxide font-bold shrink-0">"$"</span>
                                    <span class="text-paper truncate">"cargo install yav-cli"</span>
                                </div>
                                <button
                                    onclick="copyText(this, 'cargo install yav-cli')"
                                    class="px-2.5 py-1 text-[11px] text-oxide hover:text-oxide-hi bg-sunk border border-rule hover:border-oxide/40 rounded cursor-pointer transition-colors shrink-0"
                                >
                                    <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                                </button>
                            </div>
                        </div>

                        // Group 5: Mobile Bottom Links
                        <div class="pt-3 border-t border-rule/50 flex items-center justify-between text-xs font-mono text-dim">
                            <a href="https://github.com/isaacvay/yav.rs" target="_blank" rel="noreferrer" class="flex items-center gap-1.5 hover:text-paper transition-colors">
                                {render_icon("minimal:github", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                <span>"GitHub"</span>
                            </a>
                            <a href="https://crates.io/crates/yav" target="_blank" rel="noreferrer" class="hover:text-paper transition-colors">
                                "crates.io"
                            </a>
                            <span class="flex items-center gap-1.5 text-ok">
                                <span class="dot dot-ok"></span>
                                <span>"Operational"</span>
                            </span>
                        </div>
                    </div>
                </nav>
                <div id="scroll-progress" class="scroll-progress" aria-hidden="true"></div>
            </header>

            <main id="main-content" class="flex-grow w-full">
                {props.children}
            </main>

            <footer class="mt-12 border-t border-[rgba(236,233,227,0.08)] overflow-hidden">
                <div class="wrap">
                    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-12 gap-8 py-8">
                        <div class="col-span-1 sm:col-span-2 md:col-span-5 space-y-6">
                            <p class="text-[20px] sm:text-[22px] leading-[1.3] tracking-[-0.025em] text-paper max-w-sm">
                                "A web framework for people who read the "<span class="serif text-[1.1em]">"assembly"</span>"."
                            </p>
                            <button
                                id="footer-copy-install"
                                onclick="copyText(this, 'cargo install yav-cli')"
                                class="group inline-flex items-center gap-3 sm:gap-4 h-11 px-3.5 sm:px-4 max-w-full border border-[rgba(236,233,227,0.14)] hover:border-[rgba(236,233,227,0.24)] font-mono text-[12px] sm:text-[13px] text-paper transition-colors cursor-pointer overflow-hidden"
                            >
                                <span class="text-oxide">"$"</span>
                                <span class="truncate">"cargo install yav-cli"</span>
                                <span data-copy-label="" class="text-oxide font-medium text-[11px] pl-2.5 sm:pl-3 border-l border-[rgba(236,233,227,0.14)] shrink-0">"copy"</span>
                            </button>
                        </div>
                        <div class="sm:col-span-1 md:col-span-2 md:col-start-7 space-y-4">
                            <div class="label">"Framework"</div>
                            <ul class="space-y-2.5 text-[14px]">
                                <li><a href="/" class="ulink" data-i18n="nav.overview">"Overview"</a></li>
                                <li><a href="/about" class="ulink" data-i18n="nav.architecture">"Architecture"</a></li>
                                <li><a href="/docs" class="ulink" data-i18n="nav.docs">"Docs"</a></li>
                                <li><a href="/benchmarks" class="ulink" data-i18n="nav.benchmarks">"Benchmarks"</a></li>
                                <li><a href="/components" class="ulink" data-i18n="nav.components">"Components"</a></li>
                                <li><a href="/showcase" class="ulink">"Showcase"</a></li>
                                <li><a href="/faq" class="ulink">"Why Yav / FAQ"</a></li>
                                <li><a href="/releases" class="ulink">"Changelog"</a></li>
                            </ul>
                        </div>
                        <div class="sm:col-span-1 md:col-span-2 space-y-4">
                            <div class="label">"Crates"</div>
                            <ul class="space-y-2.5 font-mono text-[13px]">
                                <li><a href="https://crates.io/crates/yav" target="_blank" rel="noreferrer" class="ulink">"yav"</a></li>
                                <li><a href="https://crates.io/crates/yav-core" target="_blank" rel="noreferrer" class="ulink">"yav-core"</a></li>
                                <li><a href="https://crates.io/crates/yav-macros" target="_blank" rel="noreferrer" class="ulink">"yav-macros"</a></li>
                                <li><a href="https://crates.io/crates/yav-server" target="_blank" rel="noreferrer" class="ulink">"yav-server"</a></li>
                                <li><a href="https://crates.io/crates/yav-cli" target="_blank" rel="noreferrer" class="ulink">"yav-cli"</a></li>
                            </ul>
                        </div>
                        <div class="sm:col-span-1 md:col-span-2 space-y-4">
                            <div class="label">"Elsewhere"</div>
                            <ul class="space-y-2.5 text-[14px]">
                                <li><a href="https://github.com/isaacvay/yav.rs" target="_blank" rel="noreferrer" class="ulink">"GitHub"</a></li>
                                <li><a href="https://docs.rs/yav" target="_blank" rel="noreferrer" class="ulink">"docs.rs"</a></li>
                                <li><a href="https://github.com/isaacvay/yav.rs/issues" target="_blank" rel="noreferrer" class="ulink">"Issues"</a></li>
                            </ul>
                        </div>
                    </div>
                </div>
                <div class="wrap overflow-hidden">
                    <div class="wordmark" aria-hidden="true">"yav.rs"</div>
                </div>
                <div class="border-t border-[rgba(236,233,227,0.08)]">
                    <div class="wrap min-h-14 py-3 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2 sm:gap-4 font-mono text-[11.5px] text-dim">
                        <span>
                            "© 2026 Isaac Yav & contributors — "
                            <a href="/privacy" class="hover:text-paper transition-colors underline underline-offset-2">"Privacy & Security"</a>
                            " — MIT / Apache-2.0"
                        </span>
                        <span class="flex items-center gap-2 shrink-0"><span class="dot dot-ok"></span><span data-i18n="footer.status">"All systems operational"</span></span>
                    </div>
                </div>
            </footer>
            {motion_script()}
            {ui_script}
        </div>
    }
}
