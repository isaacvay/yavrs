//! Interactive Code Studio & Dynamic Output Panel for Home Page.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn HomeStudio() -> Node {
    let studio_script = Node::raw(r#"<script>
window.originalCodeHtml = {};
window.activeTab = 'rsx';
window.typingTimeout = null;
window.isTyping = false;

window.skipTyping = function() {
    if (window.typingTimeout) {
        clearTimeout(window.typingTimeout);
        window.typingTimeout = null;
    }
    window.isTyping = false;
    var container = document.getElementById('hero-code-' + window.activeTab);
    if (container && window.originalCodeHtml[window.activeTab]) {
        container.innerHTML = window.originalCodeHtml[window.activeTab];
        var cur = document.createElement('span');
        cur.className = 'code-cursor';
        cur.textContent = '▋';
        if (container.lastElementChild) {
            container.lastElementChild.appendChild(cur);
        }
    }
};

window.replayCurrentTab = function() {
    window.typewriteTab(window.activeTab);
};

window.typewriteTab = function(tab) {
    if (window.typingTimeout) {
        clearTimeout(window.typingTimeout);
        window.typingTimeout = null;
    }
    window.activeTab = tab;
    var container = document.getElementById('hero-code-' + tab);
    if (!container) return;

    // Cache original HTML once
    if (!window.originalCodeHtml[tab]) {
        window.originalCodeHtml[tab] = container.innerHTML;
    }
    var fullHtml = window.originalCodeHtml[tab];

    var temp = document.createElement('div');
    temp.innerHTML = fullHtml;
    var originalLines = Array.from(temp.children);
    if (originalLines.length === 0) return;

    container.innerHTML = '';
    window.isTyping = true;

    var cursor = document.createElement('span');
    cursor.className = 'code-cursor';
    cursor.textContent = '▋';

    var lineIndex = 0;

    function processLine() {
        if (!window.isTyping) return;
        if (lineIndex >= originalLines.length) {
            window.isTyping = false;
            if (container.lastElementChild) {
                container.lastElementChild.appendChild(cursor);
            }
            return;
        }

        var sourceLine = originalLines[lineIndex];
        var currentLineElem = document.createElement('div');
        container.appendChild(currentLineElem);

        // Preserve line number span if present
        var lnSpan = sourceLine.querySelector('.ln');
        if (lnSpan) {
            currentLineElem.appendChild(lnSpan.cloneNode(true));
        }

        currentLineElem.appendChild(cursor);

        // Collect characters excluding .ln span
        var chars = [];
        Array.from(sourceLine.childNodes).forEach(function(node) {
            if (node.nodeType === 1 && node.classList.contains('ln')) return;
            if (node.nodeType === 3) {
                var txt = node.textContent;
                for (var i = 0; i < txt.length; i++) {
                    chars.push({ char: txt[i], className: null });
                }
            } else if (node.nodeType === 1) {
                var txt = node.textContent;
                var cls = node.className;
                for (var j = 0; j < txt.length; j++) {
                    chars.push({ char: txt[j], className: cls });
                }
            }
        });

        if (chars.length === 0) {
            lineIndex++;
            window.typingTimeout = setTimeout(processLine, 35);
            return;
        }

        var charIndex = 0;
        var currentSpan = null;

        function typeChar() {
            if (!window.isTyping) return;
            if (charIndex >= chars.length) {
                lineIndex++;
                window.typingTimeout = setTimeout(processLine, 45);
                return;
            }

            var c = chars[charIndex];
            if (c.className) {
                if (currentSpan && currentSpan.className === c.className) {
                    currentSpan.textContent += c.char;
                } else {
                    currentSpan = document.createElement('span');
                    currentSpan.className = c.className;
                    currentSpan.textContent = c.char;
                    currentLineElem.insertBefore(currentSpan, cursor);
                }
            } else {
                currentSpan = null;
                currentLineElem.insertBefore(document.createTextNode(c.char), cursor);
            }

            charIndex++;
            var speed = c.char === ' ' ? 6 : 14;
            window.typingTimeout = setTimeout(typeChar, speed);
        }

        typeChar();
    }

    processLine();
};

window.switchHeroTab = function(tab) {
    var tabs = ['rsx', 'api', 'yavml', 'term'];
    for (var i = 0; i < tabs.length; i++) {
        var t = tabs[i];
        var code = document.getElementById('hero-code-' + t);
        var btn = document.getElementById('btn-file-' + t);
        var output = document.getElementById('hero-output-' + t);
        if (code) {
            if (t === tab) {
                code.classList.remove('hidden');
            } else {
                code.classList.add('hidden');
            }
        }
        if (output) {
            if (t === tab) {
                output.classList.remove('hidden');
            } else {
                output.classList.add('hidden');
            }
        }
        if (btn) {
            if (t === tab) {
                btn.classList.add('active');
            } else {
                btn.classList.remove('active');
            }
        }
    }

    // Trigger typewriter animation for the newly activated tab
    window.typewriteTab(tab);
};

(function initStudio() {
    function start() {
        ['rsx', 'api', 'yavml', 'term'].forEach(function(t) {
            var el = document.getElementById('hero-code-' + t);
            if (el) window.originalCodeHtml[t] = el.innerHTML;
        });

        var hasAnimated = false;
        var rsxBlock = document.getElementById('hero-code-rsx');
        if (rsxBlock && 'IntersectionObserver' in window) {
            var observer = new IntersectionObserver(function(entries) {
                entries.forEach(function(entry) {
                    if (entry.isIntersecting && !hasAnimated) {
                        hasAnimated = true;
                        window.typewriteTab('rsx');
                        observer.disconnect();
                    }
                });
            }, { threshold: 0.25 });
            observer.observe(rsxBlock);
        } else {
            setTimeout(function() { window.typewriteTab('rsx'); }, 200);
        }
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', start);
    } else {
        start();
    }
})();
</script>"#);

    rsx! {
        <section class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"01"</span>
                <span class="t" data-i18n="studio.badge">"The code"</span>
                <span class="sec-rule"></span>
                <span class="r">"src/routes/"</span>
            </div>
            <div class="grid lg:grid-cols-12 gap-10 pt-6">
                <div class="lg:col-span-4 space-y-6 reveal">
                    <h2 class="h-section text-paper">
                        <span data-i18n="studio.title.lead">"Markup the compiler "</span><span class="serif text-oxide" data-i18n="studio.title.accent">"understands."</span>
                    </h2>
                    <p class="lede" data-i18n="studio.lede">
                        "RSX is expanded and checked by rustc. Unclosed tags, wrong prop types and unescaped inputs fail during compilation — never in production."
                    </p>
                    <ul class="space-y-3 pt-2 font-mono text-[13px] text-mute">
                        <li class="flex gap-3"><span class="text-oxide">"→"</span><span data-i18n="studio.bullet.1">"One file, one route"</span></li>
                        <li class="flex gap-3"><span class="text-oxide">"→"</span><span data-i18n="studio.bullet.2">"HTML escaped by default (zero XSS)"</span></li>
                        <li class="flex gap-3"><span class="text-oxide">"→"</span><span data-i18n="studio.bullet.3">"Zero runtime template parsing"</span></li>
                        <li class="flex gap-3"><span class="text-oxide">"→"</span><span data-i18n="studio.bullet.4">"Direct memory buffer appends"</span></li>
                    </ul>
                </div>

                <div class="lg:col-span-8 reveal" style="--d:120ms">
                    <div class="plate">
                        <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                        <div class="flex items-stretch border-b border-[rgba(236,233,227,0.08)] overflow-x-auto bg-[#101012]">
                            <button id="btn-file-rsx" onclick="window.switchHeroTab('rsx')" class="tab-btn active">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span>"index.rs"</span>
                            </button>
                            <button id="btn-file-api" onclick="window.switchHeroTab('api')" class="tab-btn">
                                {render_icon("server", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span>"api/users.rs"</span>
                            </button>
                            <button id="btn-file-yavml" onclick="window.switchHeroTab('yavml')" class="tab-btn">
                                {render_icon("settings", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span>"package.yavml"</span>
                            </button>
                            <button id="btn-file-term" onclick="window.switchHeroTab('term')" class="tab-btn">
                                {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="studio.tab.term">"terminal"</span>
                            </button>
                            <span class="flex-1"></span>
                            <button id="btn-replay-code" onclick="window.replayCurrentTab()" class="hidden sm:flex items-center gap-1.5 px-3 font-mono text-[11px] text-dim hover:text-oxide transition-colors cursor-pointer" title="Replay typing animation" data-i18n-title="studio.replay.title">
                                {render_icon("play", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                                <span data-i18n="studio.replay">"Replay"</span>
                            </button>
                            <span class="hidden sm:flex items-center gap-2 px-4 font-mono text-[11px] text-dim border-l border-[rgba(236,233,227,0.08)]">
                                <span class="dot dot-ok"></span>
                                "127.0.0.1:3000"
                            </span>
                        </div>
                        <div class="grid md:grid-cols-12">
                            // Code block left pane
                            <div class="md:col-span-8 bg-sunk p-6 overflow-x-auto code min-h-[380px] cursor-pointer" onclick="window.skipTyping()" title="Click to show full code immediately" data-i18n-title="studio.skip.title">
                                <div id="hero-code-rsx">
                                    <div><span class="ln">"1"</span><span class="token-keyword">"use"</span>" yav::prelude::*;"</div>
                                    <div><span class="ln">"2"</span></div>
                                    <div><span class="ln">"3"</span><span class="token-comment">"// Discovered by auto_routes!() at compile time"</span></div>
                                    <div><span class="ln">"4"</span><span class="token-keyword">"pub async fn "</span><span class="token-fn">"index"</span>"(_req: Request) -> HtmlPage {"</div>
                                    <div><span class="ln">"5"</span>"    "<span class="token-macro">"rsx!"</span>" {"</div>
                                    <div><span class="ln">"6"</span>"        "<span class="token-tag">"<AppLayout>"</span></div>
                                    <div><span class="ln">"7"</span>"            "<span class="token-tag">"<h1 "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-4xl\""</span><span class="token-tag">">"</span></div>
                                    <div><span class="ln">"8"</span>"                "<span class="token-string">"\"Hello, metal.\""</span></div>
                                    <div><span class="ln">"9"</span>"            "<span class="token-tag">"</h1>"</span></div>
                                    <div><span class="ln">"10"</span>"            "<span class="token-tag">"<Counter "</span><span class="token-attr">"initial="</span><span class="token-val">"{42}"</span><span class="token-tag">" />"</span></div>
                                    <div><span class="ln">"11"</span>"        "<span class="token-tag">"</AppLayout>"</span></div>
                                    <div><span class="ln">"12"</span>"    }"</div>
                                    <div><span class="ln">"13"</span>"}"</div>
                                </div>
                                <div id="hero-code-api" class="hidden">
                                    <div><span class="ln">"1"</span><span class="token-keyword">"use"</span>" axum::extract::Path;"</div>
                                    <div><span class="ln">"2"</span><span class="token-keyword">"use"</span>" axum::Json;"</div>
                                    <div><span class="ln">"3"</span><span class="token-keyword">"use"</span>" serde::Serialize;"</div>
                                    <div><span class="ln">"4"</span></div>
                                    <div><span class="ln">"5"</span><span class="token-keyword">"#[derive(Serialize)]"</span></div>
                                    <div><span class="ln">"6"</span><span class="token-keyword">"pub struct "</span><span class="token-type">"User"</span>" { id: "<span class="token-type">"u64"</span>", name: "<span class="token-type">"String"</span>", role: "<span class="token-type">"String"</span>" }"</div>
                                    <div><span class="ln">"7"</span></div>
                                    <div><span class="ln">"8"</span><span class="token-keyword">"pub async fn "</span><span class="token-fn">"get"</span>"(Path(id): Path<u64>) -> Json<User> {"</div>
                                    <div><span class="ln">"9"</span>"    Json(User {"</div>
                                    <div><span class="ln">"10"</span>"        id,"</div>
                                    <div><span class="ln">"11"</span>"        name: "<span class="token-string">"\"Isaac Yav\""</span>".into(),"</div>
                                    <div><span class="ln">"12"</span>"        role: "<span class="token-string">"\"admin\""</span>".into(),"</div>
                                    <div><span class="ln">"13"</span>"    })"</div>
                                    <div><span class="ln">"14"</span>"}"</div>
                                </div>
                                <div id="hero-code-yavml" class="hidden">
                                    <div><span class="ln">"1"</span><span class="token-keyword">"[server]"</span></div>
                                    <div><span class="ln">"2"</span>"port = "<span class="token-val">"3000"</span></div>
                                    <div><span class="ln">"3"</span>"host = "<span class="token-string">"\"127.0.0.1\""</span></div>
                                    <div><span class="ln">"4"</span></div>
                                    <div><span class="ln">"5"</span><span class="token-keyword">"[tailwind]"</span></div>
                                    <div><span class="ln">"6"</span>"enabled = "<span class="token-val">"true"</span></div>
                                    <div><span class="ln">"7"</span>"version = "<span class="token-string">"\"3.4.17\""</span></div>
                                    <div><span class="ln">"8"</span></div>
                                    <div><span class="ln">"9"</span><span class="token-keyword">"[build]"</span></div>
                                    <div><span class="ln">"10"</span>"minify = "<span class="token-val">"true"</span></div>
                                    <div><span class="ln">"11"</span>"fingerprint = "<span class="token-val">"true"</span></div>
                                    <div><span class="ln">"12"</span>"pre_compress = "<span class="token-val">"[\"br\", \"gzip\"]"</span></div>
                                </div>
                                <div id="hero-code-term" class="hidden">
                                    <div class="text-dim">"$ yav dev"</div>
                                    <div class="pt-2">"  yav dev server  v0.1.5"</div>
                                    <div class="text-dim">"  ─────────────────────────────"</div>
                                    <div>"  tailwind   "<span class="token-string">"compiled"</span><span class="text-dim">"  48ms"</span></div>
                                    <div>"  rustc      "<span class="token-string">"ok"</span><span class="text-dim">"        1.9s"</span></div>
                                    <div>"  reload     "<span class="token-string">"listening"</span><span class="text-dim">" :35729"</span></div>
                                    <div class="pt-2">"  ready on "<span class="text-oxide font-bold">"http://127.0.0.1:3000"</span></div>
                                    <div class="pt-2 blink text-dim">"  watching src/ public/ package.yavml"</div>
                                </div>
                            </div>

                            // Dynamic Output right pane
                            <div class="md:col-span-4 border-t md:border-t-0 md:border-l border-[rgba(236,233,227,0.08)] p-6 flex flex-col justify-between gap-6">
                                <div id="hero-output-rsx" class="space-y-6">
                                    <div class="label flex items-center justify-between">
                                        <span data-i18n="studio.out.label">"Output"</span>
                                        <span class="text-oxide" data-i18n="studio.out.island">"RSX Island"</span>
                                    </div>
                                    <div>
                                        <div class="text-[26px] tracking-[-0.03em] text-paper leading-tight">"Hello, metal."</div>
                                        <div class="flex items-center gap-2 pt-5">
                                            <button id="demo-dec" onclick="var el=document.getElementById('demo-count'); if(el) el.innerText=Math.max(0, parseInt(el.innerText)-1);" class="w-10 h-10 border border-[rgba(236,233,227,0.14)] hover:border-[rgba(236,233,227,0.3)] font-mono text-paper transition-colors cursor-pointer select-none">"−"</button>
                                            <span id="demo-count" class="flex-1 h-10 flex items-center justify-center font-mono text-[20px] text-paper border border-[rgba(236,233,227,0.08)] bg-sunk">"42"</span>
                                            <button id="demo-inc" onclick="var el=document.getElementById('demo-count'); if(el) el.innerText=parseInt(el.innerText)+1;" class="w-10 h-10 bg-[#ece9e3] text-[#0b0b0c] font-mono hover:bg-white transition-colors cursor-pointer select-none">"+"</button>
                                        </div>
                                    </div>
                                    <div class="space-y-2 font-mono text-[11.5px] border-t border-[rgba(236,233,227,0.08)] pt-4">
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.escaping">"escaping"</span><span class="text-paper" data-i18n="studio.out.escaping_val">"auto on (zero XSS)"</span></div>
                                        <div class="flex justify-between"><span class="text-dim">"node_modules"</span><span class="text-paper">"0 B"</span></div>
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.hydration">"hydration JS"</span><span class="text-paper">"0 KB"</span></div>
                                    </div>
                                </div>

                                <div id="hero-output-api" class="hidden space-y-6">
                                    <div class="label flex items-center justify-between">
                                        <span data-i18n="studio.out.response">"Response"</span>
                                        <span class="text-ok">"HTTP 200 OK"</span>
                                    </div>
                                    <div class="bg-sunk p-4 border border-[rgba(236,233,227,0.08)] font-mono text-[12px] text-paper space-y-1">
                                        <div class="text-dim font-bold">"// application/json"</div>
                                        <div>"{"</div>
                                        <div>"  "<span class="text-oxide">"\"id\""</span>": 42,"</div>
                                        <div>"  "<span class="text-oxide">"\"name\""</span>": "<span class="token-string">"\"Isaac Yav\""</span>","</div>
                                        <div>"  "<span class="text-oxide">"\"role\""</span>": "<span class="token-string">"\"admin\""</span></div>
                                        <div>"}"</div>
                                    </div>
                                    <div class="space-y-2 font-mono text-[11.5px] border-t border-[rgba(236,233,227,0.08)] pt-4">
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.extractor">"extractor"</span><span class="text-paper">"Path<u64>"</span></div>
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.latency">"latency"</span><span class="text-paper" data-i18n="studio.out.latency_val">"18 ns (zero-copy)"</span></div>
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.format">"format"</span><span class="text-paper">"serde_json"</span></div>
                                    </div>
                                </div>

                                <div id="hero-output-yavml" class="hidden space-y-6">
                                    <div class="label flex items-center justify-between">
                                        <span data-i18n="studio.out.pipeline">"Pipeline"</span>
                                        <span class="text-paper">"package.yavml"</span>
                                    </div>
                                    <div class="space-y-3 font-mono text-[12px]">
                                        <div class="p-3 bg-sunk border border-[rgba(236,233,227,0.08)] flex justify-between items-center">
                                            <span class="text-dim">"Tailwind CLI"</span>
                                            <span class="chip font-bold text-oxide" data-i18n="studio.out.standalone">"Standalone"</span>
                                        </div>
                                        <div class="p-3 bg-sunk border border-[rgba(236,233,227,0.08)] flex justify-between items-center">
                                            <span class="text-dim" data-i18n="studio.out.compression">"Compression"</span>
                                            <span class="text-paper">"Brotli + Gzip"</span>
                                        </div>
                                        <div class="p-3 bg-sunk border border-[rgba(236,233,227,0.08)] flex justify-between items-center">
                                            <span class="text-dim" data-i18n="studio.out.etags">"ETags Cache"</span>
                                            <span class="text-paper" data-i18n="studio.out.etags_val">"304 Conditional"</span>
                                        </div>
                                    </div>
                                    <div class="space-y-2 font-mono text-[11.5px] border-t border-[rgba(236,233,227,0.08)] pt-4">
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.fingerprint">"fingerprint"</span><span class="text-paper" data-i18n="studio.out.fingerprint_val">"blake3 hash"</span></div>
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.node_req">"node required"</span><span class="text-paper" data-i18n="cmp.val.never">"Never"</span></div>
                                    </div>
                                </div>

                                <div id="hero-output-term" class="hidden space-y-6">
                                    <div class="label flex items-center justify-between">
                                        <span data-i18n="studio.out.dev_server">"Dev Server"</span>
                                        <span class="dot dot-ok"></span>
                                    </div>
                                    <div class="space-y-3 font-mono text-[12px]">
                                        <div class="p-3 bg-sunk border border-[rgba(236,233,227,0.08)] space-y-1">
                                            <div class="text-dim text-[11px]" data-i18n="studio.out.livereload">"LIVE RELOAD LISTENER"</div>
                                            <div class="text-paper">"ws://127.0.0.1:35729"</div>
                                        </div>
                                        <div class="p-3 bg-sunk border border-[rgba(236,233,227,0.08)] flex justify-between items-center">
                                            <span class="text-dim" data-i18n="studio.out.overlay">"Error Overlay"</span>
                                            <span class="text-ok" data-i18n="studio.out.overlay_val">"Dark modal on"</span>
                                        </div>
                                    </div>
                                    <div class="space-y-2 font-mono text-[11.5px] border-t border-[rgba(236,233,227,0.08)] pt-4">
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.rebuild">"rebuild time"</span><span class="text-paper">"48 ms"</span></div>
                                        <div class="flex justify-between"><span class="text-dim" data-i18n="studio.out.watcher">"file watcher"</span><span class="text-paper">"kqueue / inotify"</span></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            {studio_script}
        </section>
    }
}
