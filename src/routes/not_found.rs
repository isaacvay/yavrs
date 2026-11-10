//! 404 Not Found — Editorial Blueprint fallback handler.

use yav::prelude::*;
use axum::extract::Request;
use axum::http::StatusCode;
use crate::layouts::*;

pub async fn not_found(_req: Request) -> (StatusCode, yav::server::HtmlPage) {
    let page = rsx! {
        <AppLayout>
            <div class="wrap py-16 sm:py-24 max-w-4xl min-h-[70vh] flex flex-col justify-center">
                <div class="space-y-8">
                    // Sub-badge
                    <div class="inline-flex items-center gap-2 px-3 py-1 bg-ink-2 border border-rule font-mono text-[11.5px] text-oxide">
                        <span class="w-2 h-2 rounded-full bg-oxide animate-pulse"></span>
                        <span data-i18n="err404.badge">"HTTP 404 · RADIX TREE ROUTE MISS"</span>
                    </div>

                    // Hero heading
                    <div class="space-y-3">
                        <h1 class="text-[42px] sm:text-[60px] font-bold text-paper tracking-[-0.035em] leading-[1.05]" data-i18n="err404.title">
                            "Route not found in binary."
                        </h1>
                        <p class="text-[17px] text-mute max-w-xl leading-relaxed">
                            <span data-i18n="err404.lede.a">"The requested path does not map to any active file handler in "</span>
                            <code class="inline">"src/routes/"</code>
                            <span data-i18n="err404.lede.b">" or static buffer in "</span>
                            <code class="inline">"public/"</code>
                            "."
                        </p>
                    </div>

                    // Diagnostic Terminal
                    <div class="term-window max-w-2xl">
                        <div class="term-head">
                            <div class="flex items-center gap-2">
                                <div class="flex items-center gap-1.5">
                                    <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                    <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                    <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                                </div>
                                <span class="text-faint text-[11px] ml-2" data-i18n="err404.term_head">"diagnostic trace"</span>
                            </div>
                            <span class="text-[10px] text-dim font-mono" data-i18n="err404.term_status">"404 Not Found"</span>
                        </div>
                        <div class="p-5 code text-[12.5px] space-y-1 bg-[#070709]">
                            <div class="text-dim">"$ curl -I http://127.0.0.1:3000/unmatched-route"</div>
                            <div class="text-oxide">"HTTP/1.1 404 Not Found"</div>
                            <div>"server: yav-server/0.1.5 (hyper/1.0 tokio/1.0)"</div>
                            <div>"x-router: RadixTreeLookup::Miss"</div>
                            <div>"content-type: text/html; charset=utf-8"</div>
                            <div class="pt-2 token-comment" data-i18n="err404.term_hint">"# Recommended action: jump to an active section below"</div>
                        </div>
                    </div>

                    // Quick navigation cards
                    <div class="space-y-3 pt-4">
                        <div class="text-dim font-mono text-[11px] uppercase tracking-wider" data-i18n="err404.endpoints_title">
                            "Registered Top-Level Endpoints"
                        </div>
                        <div class="grid sm:grid-cols-2 md:grid-cols-3 gap-3">
                            <a href="/" class="plate p-4 bg-ink-2 hover:border-oxide/60 transition-colors group block space-y-1">
                                <div class="text-paper font-semibold text-[14px] flex items-center justify-between">
                                    <span data-i18n="err404.c1.t">"01. Overview"</span>
                                    <span class="text-dim group-hover:text-oxide transition-colors">"→"</span>
                                </div>
                                <p class="text-[12px] text-mute" data-i18n="err404.c1.d">"Framework flagship showcase & features."</p>
                            </a>
                            <a href="/docs" class="plate p-4 bg-ink-2 hover:border-oxide/60 transition-colors group block space-y-1">
                                <div class="text-paper font-semibold text-[14px] flex items-center justify-between">
                                    <span data-i18n="err404.c2.t">"02. Documentation"</span>
                                    <span class="text-dim group-hover:text-oxide transition-colors">"→"</span>
                                </div>
                                <p class="text-[12px] text-mute" data-i18n="err404.c2.d">"Complete engineering manual & guides."</p>
                            </a>
                            <a href="/components" class="plate p-4 bg-ink-2 hover:border-oxide/60 transition-colors group block space-y-1">
                                <div class="text-paper font-semibold text-[14px] flex items-center justify-between">
                                    <span data-i18n="err404.c3.t">"03. Components"</span>
                                    <span class="text-dim group-hover:text-oxide transition-colors">"→"</span>
                                </div>
                                <p class="text-[12px] text-mute" data-i18n="err404.c3.d">"Scaffolded RSX UI library & primitives."</p>
                            </a>
                            <a href="/about" class="plate p-4 bg-ink-2 hover:border-oxide/60 transition-colors group block space-y-1">
                                <div class="text-paper font-semibold text-[14px] flex items-center justify-between">
                                    <span data-i18n="err404.c4.t">"04. Architecture"</span>
                                    <span class="text-dim group-hover:text-oxide transition-colors">"→"</span>
                                </div>
                                <p class="text-[12px] text-mute" data-i18n="err404.c4.d">"Tokio thread pool & memory layout."</p>
                            </a>
                            <a href="/benchmarks" class="plate p-4 bg-ink-2 hover:border-oxide/60 transition-colors group block space-y-1">
                                <div class="text-paper font-semibold text-[14px] flex items-center justify-between">
                                    <span data-i18n="err404.c5.t">"05. Benchmarks"</span>
                                    <span class="text-dim group-hover:text-oxide transition-colors">"→"</span>
                                </div>
                                <p class="text-[12px] text-mute" data-i18n="err404.c5.d">"124,500 req/s verified harness."</p>
                            </a>
                            <button onclick="openCmdPalette()" class="plate p-4 bg-ink-2 hover:border-oxide/60 transition-colors group block space-y-1 text-left cursor-pointer w-full">
                                <div class="text-paper font-semibold text-[14px] flex items-center justify-between">
                                    <span data-i18n="err404.c6.t">"Search Palettes"</span>
                                    <kbd class="kbd text-[10px]">"Ctrl K"</kbd>
                                </div>
                                <p class="text-[12px] text-mute" data-i18n="err404.c6.d">"Quick jump to any document or symbol."</p>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </AppLayout>
    };

    (
        StatusCode::NOT_FOUND,
        website_page(
            "404 Not Found — Yav.rs",
            "The requested route does not exist in the Yav binary.",
            "/404",
            page,
        ),
    )
}
