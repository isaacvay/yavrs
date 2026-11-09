//! Performance Benchmarks & Framework Comparison for Home Page.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn HomeBenchmarks() -> Node {
    let bench_script = Node::raw(r#"<script>
window.runBenchmarkAnimation = function() {
    if (window._benchAnimFrame) {
        cancelAnimationFrame(window._benchAnimFrame);
        window._benchAnimFrame = null;
    }

    var items = [
        { numId: 'bench-val-actix',   barId: 'bench-bar-actix',   targetVal: 128000, targetPct: 100.0 },
        { numId: 'bench-val-yav',     barId: 'bench-bar-yav',     targetVal: 124500, targetPct: 97.27, pctId: 'bench-pct-yav', targetRatio: 97.3 },
        { numId: 'bench-val-gin',     barId: 'bench-bar-gin',     targetVal: 76200,  targetPct: 59.53 },
        { numId: 'bench-val-fastify', barId: 'bench-bar-fastify', targetVal: 34100,  targetPct: 26.64 },
        { numId: 'bench-val-next',    barId: 'bench-bar-next',    targetVal: 8900,   targetPct: 6.95 }
    ];

    var container = document.getElementById('benchmarks-bars-container');
    if (!container) return;

    var lines = container.querySelectorAll('.bench-line');
    var ruler = container.querySelector('.ruler');
    var dimBar = container.querySelector('.dim-line .bar');
    var secRule = document.querySelector('.sec-head .sec-rule');

    // Immediate visual reset to 0
    items.forEach(function(item) {
        var bar = document.getElementById(item.barId);
        if (bar) bar.style.width = '0%';
        var num = document.getElementById(item.numId);
        if (num) num.textContent = '0';
        if (item.pctId) {
            var pct = document.getElementById(item.pctId);
            if (pct) pct.textContent = '(0.0%)';
        }
    });
    lines.forEach(function(l) { l.style.width = '0%'; });
    if (ruler) ruler.style.width = '0%';
    if (dimBar) dimBar.style.transform = 'scaleX(0)';
    if (secRule) secRule.style.transform = 'scaleX(0)';

    var startTime = performance.now();
    var duration = 1400; // 1.4s smooth easeOutCubic

    function step(now) {
        var elapsed = now - startTime;
        var progress = Math.min(elapsed / duration, 1);
        var ease = 1 - Math.pow(1 - progress, 3); // cubic ease-out

        // Frame-by-frame lockstep update of bars & numbers
        items.forEach(function(item) {
            var bar = document.getElementById(item.barId);
            if (bar) {
                bar.style.width = (item.targetPct * ease).toFixed(2) + '%';
            }
            var num = document.getElementById(item.numId);
            if (num) {
                var currentVal = Math.round(item.targetVal * ease);
                num.textContent = currentVal.toLocaleString('en-US');
            }
            if (item.pctId) {
                var pct = document.getElementById(item.pctId);
                if (pct) {
                    var currentRatio = (item.targetRatio * ease).toFixed(1);
                    pct.textContent = '(' + currentRatio + '%)';
                }
            }
        });

        // Frame-by-frame animation of divider lines & ruler scale
        var linePct = (100 * ease).toFixed(1) + '%';
        lines.forEach(function(l) { l.style.width = linePct; });
        if (ruler) ruler.style.width = linePct;
        if (dimBar) dimBar.style.transform = 'scaleX(' + ease.toFixed(3) + ')';
        if (secRule) secRule.style.transform = 'scaleX(' + ease.toFixed(3) + ')';

        if (progress < 1) {
            window._benchAnimFrame = requestAnimationFrame(step);
        } else {
            // Guarantee final target values
            items.forEach(function(item) {
                var bar = document.getElementById(item.barId);
                if (bar) bar.style.width = item.targetPct + '%';
                var num = document.getElementById(item.numId);
                if (num) num.textContent = item.targetVal.toLocaleString('en-US');
                if (item.pctId) {
                    var pct = document.getElementById(item.pctId);
                    if (pct) pct.textContent = '(' + item.targetRatio + '%)';
                }
            });
            lines.forEach(function(l) { l.style.width = '100%'; });
            if (ruler) ruler.style.width = '100%';
            if (dimBar) dimBar.style.transform = 'scaleX(1)';
            if (secRule) secRule.style.transform = 'scaleX(1)';
            window._benchAnimFrame = null;
        }
    }

    window._benchAnimFrame = requestAnimationFrame(step);
};

(function initBenchmarkObserver() {
    function setup() {
        var container = document.getElementById('benchmarks-bars-container');
        if (!container) return;

        // Initialize elements to 0% so they do not flash before animation starts
        var bars = container.querySelectorAll('.hbar > i');
        bars.forEach(function(b) {
            b.style.display = 'block';
            b.style.width = '0%';
        });
        var lines = container.querySelectorAll('.bench-line');
        lines.forEach(function(l) { l.style.width = '0%'; });
        var ruler = container.querySelector('.ruler');
        if (ruler) ruler.style.width = '0%';

        var triggered = false;
        if ('IntersectionObserver' in window) {
            var observer = new IntersectionObserver(function(entries) {
                entries.forEach(function(entry) {
                    if (entry.isIntersecting && !triggered) {
                        triggered = true;
                        window.runBenchmarkAnimation();
                        observer.disconnect();
                    }
                });
            }, { threshold: 0.1 });
            observer.observe(container);
        } else {
            window.runBenchmarkAnimation();
        }
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', setup);
    } else {
        setup();
    }
})();
</script>"#);

    rsx! {
        // =====================================================
        // 04 — MEASURED BENCHMARKS
        // =====================================================
        <section class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"04"</span>
                <span class="t" data-i18n="bench.badge">"Measured"</span>
                <span class="sec-rule"></span>
                <span class="r">"wrk -t8 -c200 -d30s"</span>
            </div>
            <div class="grid lg:grid-cols-12 gap-12 pt-6">
                <div class="lg:col-span-4 space-y-6 reveal">
                    <h2 class="h-section text-paper" data-i18n="bench.title">
                        "97% of raw metal, "<span class="serif text-oxide">"with a framework on top."</span>
                    </h2>
                    <p class="text-[14.5px] text-mute leading-relaxed" data-i18n="bench.subtitle">
                        "Tested on identical 8 vCPU instances running wrk across 200 concurrent connections rendering dynamic templates with session validation."
                    </p>
                    <div class="pt-2 flex flex-col gap-3 font-mono text-[13px]">
                        <a href="/benchmarks" class="ulink inline-block">"Full methodology & reproducible steps →"</a>
                        <button id="btn-replay-bench" onclick="window.runBenchmarkAnimation()" class="inline-flex items-center gap-2 text-dim hover:text-oxide transition-colors cursor-pointer w-fit" title="Rejouer la simulation des benchmarks">
                            {render_icon("refresh-cw", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            <span>"Rerun benchmark simulation"</span>
                        </button>
                    </div>
                </div>
                <div class="lg:col-span-8 space-y-6 reveal" style="--d:120ms" id="benchmarks-bars-container">
                    // Top dimension line
                    <div class="dim-line pb-1">
                        <span>"DELTA RATIO"</span>
                        <div class="bar"></div>
                        <span class="text-oxide font-mono text-[12px] font-semibold">"only 2.7% overhead vs raw metal"</span>
                    </div>

                    // Row 1: Actix
                    <div class="space-y-2">
                        <div class="flex flex-wrap sm:flex-nowrap items-baseline justify-between gap-1 font-mono text-[12px] sm:text-[13px]">
                            <span class="text-dim">"actix-web — raw metal, no HTML"</span>
                            <span class="text-mute shrink-0"><span id="bench-val-actix" data-val="128000">"128,000"</span>" req/s"</span>
                        </div>
                        <div class="hbar" style="--w:100%"><i id="bench-bar-actix" style="display:block;width:0%"></i></div>
                        <div class="bench-line" style="width:0%"></div>
                    </div>

                    // Row 2: Yav.rs
                    <div class="space-y-2">
                        <div class="flex flex-wrap sm:flex-nowrap items-baseline justify-between gap-1 font-mono text-[12px] sm:text-[13px]">
                            <span class="text-paper font-semibold flex items-center gap-2">
                                <span class="dot dot-ok pulse-dot"></span>
                                "yav.rs — compiled RSX + HMAC"
                            </span>
                            <span class="text-oxide font-bold shrink-0">
                                <span id="bench-val-yav" data-val="124500">"124,500"</span>" req/s"
                                <span id="bench-pct-yav" class="text-[#f07448] font-mono text-[11px] sm:text-[11.5px] ml-1 font-normal">"(97.3%)"</span>
                            </span>
                        </div>
                        <div class="hbar us" style="--w:97.3%"><i id="bench-bar-yav" style="display:block;width:0%"></i></div>
                        <div class="bench-line" style="width:0%"></div>
                    </div>

                    // Row 3: Go Gin
                    <div class="space-y-2">
                        <div class="flex flex-wrap sm:flex-nowrap items-baseline justify-between gap-1 font-mono text-[12px] sm:text-[13px]">
                            <span class="text-dim">"go gin — SSR templates"</span>
                            <span class="text-mute shrink-0"><span id="bench-val-gin" data-val="76200">"76,200"</span>" req/s"</span>
                        </div>
                        <div class="hbar" style="--w:59.5%"><i id="bench-bar-gin" style="display:block;width:0%"></i></div>
                        <div class="bench-line" style="width:0%"></div>
                    </div>

                    // Row 4: Fastify
                    <div class="space-y-2">
                        <div class="flex flex-wrap sm:flex-nowrap items-baseline justify-between gap-1 font-mono text-[12px] sm:text-[13px]">
                            <span class="text-dim">"fastify — node 20"</span>
                            <span class="text-mute shrink-0"><span id="bench-val-fastify" data-val="34100">"34,100"</span>" req/s"</span>
                        </div>
                        <div class="hbar" style="--w:26.6%"><i id="bench-bar-fastify" style="display:block;width:0%"></i></div>
                        <div class="bench-line" style="width:0%"></div>
                    </div>

                    // Row 5: Next.js 15
                    <div class="space-y-2">
                        <div class="flex flex-wrap sm:flex-nowrap items-baseline justify-between gap-1 font-mono text-[12px] sm:text-[13px]">
                            <span class="text-dim">"next.js 15 — node SSR"</span>
                            <span class="text-mute shrink-0"><span id="bench-val-next" data-val="8900">"8,900"</span>" req/s"</span>
                        </div>
                        <div class="hbar" style="--w:7%"><i id="bench-bar-next" style="display:block;width:0%"></i></div>
                    </div>

                    // Ruler graduation line & scale
                    <div class="pt-2">
                        <div class="ruler opacity-80" style="width:0%"></div>
                        <div class="flex justify-between font-mono text-[10.5px] text-dim pt-2 select-none">
                            <span>"0 req/s"</span>
                            <span class="hidden sm:inline">"32k"</span>
                            <span>"64k"</span>
                            <span class="hidden sm:inline">"96k"</span>
                            <span class="text-oxide font-semibold">"124.5k (Yav)"</span>
                            <span>"128k (Max)"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // =====================================================
        // 05 — COMPARED MATRIX
        // =====================================================
        <section class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"05"</span>
                <span class="t">"Compared"</span>
                <span class="sec-rule"></span>
                <span class="r">"same hardware"</span>
            </div>
            <div class="pt-6 table-scroll reveal">
                <table class="dtable min-w-[760px]">
                    <thead>
                        <tr>
                            <th>"Metric / Capability"</th>
                            <th class="col-us">"yav.rs"</th>
                            <th>"next.js 15"</th>
                            <th>"leptos"</th>
                            <th>"bare axum"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="text-paper">"Throughput (req / s)"</td>
                            <td class="col-us">"124,500"</td>
                            <td>"8,900"</td>
                            <td>"45,000"</td>
                            <td>"130,000*"</td>
                        </tr>
                        <tr>
                            <td class="text-paper">"Client JS runtime"</td>
                            <td class="col-us">"0 KB"</td>
                            <td>"350–800 KB"</td>
                            <td>"250–500 KB wasm"</td>
                            <td>"0 KB"</td>
                        </tr>
                        <tr>
                            <td class="text-paper">"Median TTFB (p50)"</td>
                            <td class="col-us">"0.28 ms"</td>
                            <td>"8.40 ms"</td>
                            <td>"1.20 ms"</td>
                            <td>"0.24 ms"</td>
                        </tr>
                        <tr>
                            <td class="text-paper">"Idle memory (RSS)"</td>
                            <td class="col-us">"8.2 MB"</td>
                            <td>"180–320 MB"</td>
                            <td>"24 MB"</td>
                            <td>"6.5 MB"</td>
                        </tr>
                        <tr>
                            <td class="text-paper">"Node.js required"</td>
                            <td class="col-us">"Never"</td>
                            <td>"Yes"</td>
                            <td>"Optional"</td>
                            <td>"No"</td>
                        </tr>
                        <tr>
                            <td class="text-paper">"Template engine"</td>
                            <td class="col-us">"rsx! (checked)"</td>
                            <td>"JSX"</td>
                            <td>"view!"</td>
                            <td>"format!()"</td>
                        </tr>
                        <tr>
                            <td class="text-paper">"Built-in icons"</td>
                            <td class="col-us">"13,000+ native"</td>
                            <td>"npm package"</td>
                            <td>"crate"</td>
                            <td>"none"</td>
                        </tr>
                        <tr>
                            <td class="text-paper">"Asset pre-compression"</td>
                            <td class="col-us">"Brotli + Gzip"</td>
                            <td>"plugin / CDN"</td>
                            <td>"manual"</td>
                            <td>"Tower layer"</td>
                        </tr>
                    </tbody>
                </table>
                <p class="font-mono text-[11px] text-dim pt-4">"* Bare Axum measured without template HTML rendering (JSON only)."</p>
            </div>
        </section>

        {bench_script}
    }
}
