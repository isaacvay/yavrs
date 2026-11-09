//! Hero Section & Performance Spec Strip for Home Page.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn HomeHero() -> Node {
    let hero_spec_script = Node::raw(r#"<script>
window.runHeroSpecAnimation = function() {
    if (window._heroSpecAnimFrame) {
        cancelAnimationFrame(window._heroSpecAnimFrame);
        window._heroSpecAnimFrame = null;
    }

    var elThroughput = document.getElementById('hero-stat-throughput');
    var elTtfb       = document.getElementById('hero-stat-ttfb');
    var elMemory     = document.getElementById('hero-stat-memory');

    if (!elThroughput || !elTtfb || !elMemory) return;

    var targetThroughput = 124500;
    var targetTtfb       = 0.28;
    var targetMemory     = 8.2;

    elThroughput.textContent = '0';
    elTtfb.textContent = '0.00';
    elMemory.textContent = '0.0';

    var startTime = performance.now();
    var duration = 1400; // 1.4s smooth easeOutCubic

    function tick(now) {
        var elapsed = now - startTime;
        var progress = Math.min(elapsed / duration, 1);
        var ease = 1 - Math.pow(1 - progress, 3);

        var curThroughput = Math.round(targetThroughput * ease);
        elThroughput.textContent = curThroughput.toLocaleString('en-US');

        var curTtfb = (targetTtfb * ease).toFixed(2);
        elTtfb.textContent = curTtfb;

        var curMemory = (targetMemory * ease).toFixed(1);
        elMemory.textContent = curMemory;

        if (progress < 1) {
            window._heroSpecAnimFrame = requestAnimationFrame(tick);
        } else {
            elThroughput.textContent = targetThroughput.toLocaleString('en-US');
            elTtfb.textContent = targetTtfb.toFixed(2);
            elMemory.textContent = targetMemory.toFixed(1);
            window._heroSpecAnimFrame = null;
        }
    }

    window._heroSpecAnimFrame = requestAnimationFrame(tick);
};

(function initHeroSpecObserver() {
    function setup() {
        var strip = document.getElementById('hero-spec-strip');
        if (!strip) return;

        var elThroughput = document.getElementById('hero-stat-throughput');
        var elTtfb       = document.getElementById('hero-stat-ttfb');
        var elMemory     = document.getElementById('hero-stat-memory');
        if (elThroughput) elThroughput.textContent = '0';
        if (elTtfb) elTtfb.textContent = '0.00';
        if (elMemory) elMemory.textContent = '0.0';

        var triggered = false;
        if ('IntersectionObserver' in window) {
            var observer = new IntersectionObserver(function(entries) {
                entries.forEach(function(entry) {
                    if (entry.isIntersecting && !triggered) {
                        triggered = true;
                        window.runHeroSpecAnimation();
                        observer.disconnect();
                    }
                });
            }, { threshold: 0.15 });
            observer.observe(strip);
        } else {
            window.runHeroSpecAnimation();
        }

        strip.addEventListener('click', function() {
            window.runHeroSpecAnimation();
        });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', setup);
    } else {
        setup();
    }
})();
</script>"#);

    rsx! {
        <section class="relative overflow-hidden">
            <div class="absolute inset-0 bp-grid opacity-60 [mask-image:linear-gradient(180deg,#000_0%,transparent_85%)]" aria-hidden="true"></div>
            <div class="wrap relative pt-8 sm:pt-12 pb-10">
                <div class="fade flex flex-wrap items-center justify-between gap-4 pb-6 sm:pb-8" style="--d:0ms">
                    <div class="flex items-center gap-6 label">
                        <span class="label-oxide">"Fig. 00"</span>
                        <span>"A web framework for Rust"</span>
                    </div>
                    <a href="/docs" class="chip hover:border-[rgba(236,233,227,0.24)] transition-colors">
                        <span class="dot pulse-dot text-[#e0592f]"></span>
                        <span data-i18n="hero.pill.engine">"v0.1.4 — Axum 0.8 · 60+ UI Primitives · AI SDK & MCP"</span>
                    </a>
                </div>

                <div class="relative">
                    <div class="hero-glow" aria-hidden="true"></div>
                    <h1 class="display">
                        <span class="line-mask block">
                            <span style="--d:100ms" class="hero-title-main" data-i18n="hero.title.line1">
                                "Fast by nature."
                            </span>
                        </span>
                        <span class="line-mask block mt-1 sm:mt-2">
                            <span style="--d:220ms" class="serif hero-title-accent" data-i18n="hero.title.line2">
                                "Safe by design."
                            </span>
                        </span>
                    </h1>
                </div>

                <div class="grid lg:grid-cols-12 gap-10 pt-8 sm:pt-12 items-start">
                    <div class="lg:col-span-5 space-y-8 fade" style="--d:450ms">
                        <p class="lede" data-i18n="hero.subtitle">
                            <strong>"Compiled RSX templates, 60+ UI primitives, AI SDK & MCP, and Tailwind — in one native binary."</strong>
                            " No Node. No npm. No hydration tax. Just Axum 0.8, Tokio and the Rust compiler doing what they do best."
                        </p>
                        <div class="flex flex-wrap items-center gap-3">
                            <a id="hero-cta-start" href="/docs" class="btn btn-oxide">
                                <span data-i18n="hero.cta.start">"Start building"</span>
                                {render_icon("arrow-right", IconSize::Sm, Some("w-4 h-4 ml-1"), None)}
                            </a>
                            <button id="hero-copy-install" onclick="copyText(this, 'cargo install yav-cli')" class="btn btn-line font-mono !text-[13px]">
                                <span class="text-oxide">"$"</span>
                                <span>"cargo install yav-cli"</span>
                                <span data-copy-label="" class="text-oxide font-medium text-[11px] pl-3 border-l border-[rgba(236,233,227,0.14)]">"copy"</span>
                            </button>
                        </div>
                    </div>

                    // Schematic: request lifecycle (FIG. 01) with vector labels
                    <figure class="lg:col-span-7 fade" style="--d:600ms">
                        <div class="plate bg-sunk">
                            <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                            <div class="plate-head">
                                <span>"FIG. 01 — REQUEST LIFECYCLE"</span>
                                <span class="text-oxide font-mono">"t = 0.28 ms p50"</span>
                            </div>
                            <div class="p-4 sm:p-6 overflow-hidden">
                                <svg viewBox="0 0 640 250" class="w-full h-auto block" fill="none" aria-label="Request lifecycle schematic diagram">
                                    <g stroke="rgba(236,233,227,0.12)" stroke-width="1">
                                        // Box 1: Ingress
                                        <rect x="20" y="80" width="124" height="72" fill="rgba(236,233,227,0.03)" rx="3" stroke="rgba(236,233,227,0.24)" />
                                        // Box 2: Matchit Router
                                        <rect x="180" y="80" width="124" height="72" fill="rgba(236,233,227,0.03)" rx="3" stroke="rgba(236,233,227,0.24)" />
                                        // Box 3: RSX Renderer
                                        <rect x="340" y="80" width="124" height="72" fill="rgba(236,233,227,0.03)" rx="3" stroke="rgba(236,233,227,0.24)" />
                                        // Box 4: Wire Response
                                        <rect x="500" y="80" width="120" height="72" fill="rgba(236,233,227,0.03)" rx="3" stroke="rgba(236,233,227,0.24)" />

                                        // Flow lines
                                        <line x1="144" y1="116" x2="180" y2="116" stroke="rgba(236,233,227,0.4)" />
                                        <polygon points="176,113 182,116 176,119" fill="rgba(236,233,227,0.4)" />
                                        <line x1="304" y1="116" x2="340" y2="116" stroke="rgba(236,233,227,0.4)" />
                                        <polygon points="336,113 342,116 336,119" fill="rgba(236,233,227,0.4)" />
                                        <line x1="464" y1="116" x2="500" y2="116" stroke="rgba(236,233,227,0.4)" />
                                        <polygon points="496,113 502,116 496,119" fill="rgba(236,233,227,0.4)" />

                                        // Bottom timeline
                                        <line x1="20" y1="200" x2="620" y2="200" stroke-dasharray="3 4" stroke="rgba(236,233,227,0.2)" />
                                        <line x1="20" y1="194" x2="20" y2="206" stroke="rgba(236,233,227,0.4)" />
                                        <line x1="180" y1="196" x2="180" y2="204" stroke="rgba(236,233,227,0.3)" />
                                        <line x1="340" y1="196" x2="340" y2="204" stroke="rgba(236,233,227,0.3)" />
                                        <line x1="620" y1="194" x2="620" y2="206" stroke="#e0592f" stroke-width="1.5" />

                                        // Top route bypass arc
                                        <path d="M242 80 L242 45 L402 45 L402 80" stroke="#e0592f" stroke-width="1.2" stroke-dasharray="4 3" />
                                        <circle cx="242" cy="45" r="3" fill="#e0592f" />
                                    </g>

                                    // Vector text labels
                                    <g font-family="'Geist Mono', monospace" font-size="11" fill="rgba(236,233,227,0.6)">
                                        <text x="82" y="108" text-anchor="middle" fill="#ece9e3" font-weight="500">"hyper 1.0"</text>
                                        <text x="82" y="128" text-anchor="middle" font-size="10" fill="rgba(236,233,227,0.45)">"tcp :3000"</text>

                                        <text x="242" y="108" text-anchor="middle" fill="#ece9e3" font-weight="500">"matchit"</text>
                                        <text x="242" y="128" text-anchor="middle" font-size="10" fill="rgba(236,233,227,0.45)">"radix O(k)"</text>

                                        <text x="402" y="108" text-anchor="middle" fill="#ece9e3" font-weight="500">"rsx! -> buf"</text>
                                        <text x="402" y="128" text-anchor="middle" font-size="10" fill="#e0592f">"0 alloc · 0 vdom"</text>

                                        <text x="560" y="108" text-anchor="middle" fill="#ece9e3" font-weight="500">"HTTP 200"</text>
                                        <text x="560" y="128" text-anchor="middle" font-size="10" fill="rgba(236,233,227,0.45)">"chunked wire"</text>

                                        <text x="20" y="222" text-anchor="start" font-size="10" fill="rgba(236,233,227,0.4)">"0.00 ms (ingress)"</text>
                                        <text x="242" y="222" text-anchor="middle" font-size="10" fill="rgba(236,233,227,0.4)">"0.04 ms (routed)"</text>
                                        <text x="402" y="222" text-anchor="middle" font-size="10" fill="rgba(236,233,227,0.4)">"0.18 ms (rendered)"</text>
                                        <text x="620" y="222" text-anchor="end" font-size="10" fill="#e0592f" font-weight="500">"0.28 ms p50 TTFB"</text>

                                        <text x="322" y="38" text-anchor="middle" font-size="9.5" fill="#e0592f" letter-spacing="0.5">"COMPILED DISPATCH — ZERO RUNTIME TAX"</text>
                                    </g>

                                    // Native vector animated pulse
                                    <circle cx="20" cy="116" r="3.5" fill="#e0592f">
                                        <animate attributeName="cx" values="20;144;180;304;340;464;500;620" dur="2.8s" repeatCount="indefinite" />
                                        <animate attributeName="opacity" values="0;1;1;1;1;1;1;0" dur="2.8s" repeatCount="indefinite" />
                                    </circle>
                                </svg>
                            </div>
                        </div>
                    </figure>
                </div>
            </div>

            // Spec strip
            <div id="hero-spec-strip" class="border-y border-[rgba(236,233,227,0.08)] bg-[rgba(236,233,227,0.015)] select-none hover:bg-[rgba(236,233,227,0.03)] transition-colors cursor-pointer" title="Click to replay spec metrics animation">
                <div class="wrap grid grid-cols-2 md:grid-cols-4">
                    <div class="py-8 pr-6 md:border-r border-[rgba(236,233,227,0.08)] group">
                        <div class="label pb-3" data-i18n="metrics.throughput">"Throughput"</div>
                        <div class="text-[34px] tracking-[-0.04em] text-paper font-mono"><span id="hero-stat-throughput">"0"</span></div>
                        <div class="text-[13px] text-dim pt-1" data-i18n="metrics.throughput.desc">"req/s · 8 cores"</div>
                    </div>
                    <div class="py-8 md:px-6 md:border-r border-[rgba(236,233,227,0.08)] group">
                        <div class="label pb-3" data-i18n="metrics.latency">"Median TTFB"</div>
                        <div class="text-[34px] tracking-[-0.04em] text-paper font-mono"><span id="hero-stat-ttfb">"0.00"</span><span class="text-dim text-[18px]">" ms"</span></div>
                        <div class="text-[13px] text-dim pt-1" data-i18n="metrics.latency.desc">"p99 1.12 ms"</div>
                    </div>
                    <div class="py-8 pr-6 md:px-6 md:border-r border-[rgba(236,233,227,0.08)] group">
                        <div class="label pb-3" data-i18n="metrics.hydration">"Client JS"</div>
                        <div class="text-[34px] tracking-[-0.04em] text-paper font-mono"><span id="hero-stat-js" class="text-oxide font-semibold">"0"</span><span class="text-dim text-[18px]">" KB"</span></div>
                        <div class="text-[13px] text-dim pt-1" data-i18n="metrics.hydration.desc">"no hydration overhead"</div>
                    </div>
                    <div class="py-8 md:pl-6 group">
                        <div class="label pb-3" data-i18n="metrics.memory">"Idle memory"</div>
                        <div class="text-[34px] tracking-[-0.04em] text-paper font-mono"><span id="hero-stat-memory">"0.0"</span><span class="text-dim text-[18px]">" MB"</span></div>
                        <div class="text-[13px] text-dim pt-1" data-i18n="metrics.memory.desc">"resident memory RSS"</div>
                    </div>
                </div>
            </div>
            {hero_spec_script}
        </section>
    }
}
