//! Benchmark Full Comparative Matrix Component.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn BenchMatrix() -> Node {
    let script = Node::raw(r#"<script>
    window.filterBenchMatrix = function(cat) {
        document.querySelectorAll('.matrix-cat-btn').forEach(function(btn) {
            btn.classList.toggle('active', btn.getAttribute('data-cat') === cat);
        });
        document.querySelectorAll('.matrix-row').forEach(function(row) {
            var rowCat = row.getAttribute('data-cat');
            var match = (cat === 'all' || rowCat === cat);
            row.style.display = match ? '' : 'none';
        });
    };
    </script>"#);

    rsx! {
        <section class="wrap pt-12 sm:pt-16">
            {script}
            <div class="sec-head">
                <span class="n">"4.3"</span>
                <span class="t" data-i18n="bench.matrix.badge">"Comparative Framework Performance Matrix"</span>
                <span class="sec-rule"></span>
                <span class="r" data-i18n="bench.matrix.meta">"8 vCPU Dedicated · 16 GB RAM"</span>
            </div>

            // Filter Tabs
            <div class="pt-8 flex flex-wrap items-center justify-between gap-4">
                <div class="table-scroll pb-1 w-full sm:w-auto">
                    <div class="btn-group font-mono text-xs min-w-max">
                        <button data-cat="all" onclick="filterBenchMatrix('all')" class="btn tab-btn matrix-cat-btn active" data-i18n="bench.matrix.tab_all">
                            "All Frameworks"
                        </button>
                        <button data-cat="rust" onclick="filterBenchMatrix('rust')" class="btn tab-btn matrix-cat-btn" data-i18n="bench.matrix.tab_rust">
                            "Rust Ecosystem"
                        </button>
                        <button data-cat="go" onclick="filterBenchMatrix('go')" class="btn tab-btn matrix-cat-btn">
                            "Go"
                        </button>
                        <button data-cat="node" onclick="filterBenchMatrix('node')" class="btn tab-btn matrix-cat-btn">
                            "Node.js / Full-Stack"
                        </button>
                        <button data-cat="python" onclick="filterBenchMatrix('python')" class="btn tab-btn matrix-cat-btn">
                            "Python & Ruby"
                        </button>
                    </div>
                </div>

                <div class="text-xs font-mono text-dim" data-i18n="bench.matrix.load_note">
                    "Under identical load: 200 conns, 30s duration"
                </div>
            </div>

            // Matrix Table Plate
            <div class="plate bg-ink-2 mt-6 overflow-hidden">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="table-scroll">
                    <table class="dtable min-w-[840px]">
                        <thead>
                            <tr>
                                <th data-i18n="bench.matrix.col1">"Framework"</th>
                                <th data-i18n="bench.matrix.col2">"Language / Runtime"</th>
                                <th data-i18n="bench.matrix.col3">"Architecture"</th>
                                <th class="num" data-i18n="bench.matrix.col4">"Throughput"</th>
                                <th class="num">"p50 TTFB"</th>
                                <th class="num" data-i18n="bench.matrix.col6">"p99 Tail"</th>
                                <th class="num" data-i18n="bench.matrix.col7">"RAM (Idle → Load)"</th>
                                <th class="num" data-i18n="bench.matrix.col8">"Hydration Tax"</th>
                            </tr>
                        </thead>
                        <tbody>
                            // YAV.RS (US ROW)
                            <tr class="us matrix-row" data-cat="rust">
                                <td class="font-bold text-paper flex items-center gap-2">
                                    <span class="dot dot-ok pulse-dot"></span>
                                    "yav.rs 0.1.5"
                                </td>
                                <td class="text-paper">"Rust · Tokio 1.40"</td>
                                <td class="text-dim">"Compiled RSX · Radix O(k)"</td>
                                <td class="num font-bold text-oxide">"124,500 req/s"</td>
                                <td class="num text-oxide font-medium">"0.28 ms"</td>
                                <td class="num text-paper font-medium">"1.12 ms"</td>
                                <td class="num text-ok">"8 MB → 24 MB"</td>
                                <td class="num text-ok font-semibold" data-i18n="bench.matrix.tax_none">"0 kB (None)"</td>
                            </tr>

                            // ACTIX-WEB
                            <tr class="matrix-row" data-cat="rust">
                                <td class="font-medium text-paper">"actix-web 4.9"</td>
                                <td>"Rust · Tokio"</td>
                                <td class="text-dim" data-i18n="bench.matrix.arch_actix">"Raw API Handler (No HTML)"</td>
                                <td class="num text-paper">"128,000 req/s"</td>
                                <td class="num">"0.25 ms"</td>
                                <td class="num">"1.05 ms"</td>
                                <td class="num text-ok">"7 MB → 22 MB"</td>
                                <td class="num text-dim" data-i18n="bench.matrix.tax_na">"N/A (Raw API)"</td>
                            </tr>

                            // AXUM
                            <tr class="matrix-row" data-cat="rust">
                                <td class="font-medium text-paper">"axum 0.8"</td>
                                <td>"Rust · Tokio"</td>
                                <td class="text-dim" data-i18n="bench.matrix.arch_axum">"Raw Router (No Template)"</td>
                                <td class="num text-paper">"126,200 req/s"</td>
                                <td class="num">"0.27 ms"</td>
                                <td class="num">"1.08 ms"</td>
                                <td class="num text-ok">"8 MB → 23 MB"</td>
                                <td class="num text-dim" data-i18n="bench.matrix.tax_na">"N/A (Raw API)"</td>
                            </tr>

                            // GIN
                            <tr class="matrix-row" data-cat="go">
                                <td class="font-medium text-paper">"gin 1.10"</td>
                                <td>"Go 1.22 · Goroutines"</td>
                                <td class="text-dim">"SSR html/template"</td>
                                <td class="num text-paper">"76,200 req/s"</td>
                                <td class="num">"0.85 ms"</td>
                                <td class="num">"3.20 ms"</td>
                                <td class="num">"18 MB → 65 MB"</td>
                                <td class="num text-ok" data-i18n="bench.matrix.tax_none">"0 kB (None)"</td>
                            </tr>

                            // FIBER
                            <tr class="matrix-row" data-cat="go">
                                <td class="font-medium text-paper">"fiber 2.52"</td>
                                <td>"Go 1.22 · Fasthttp"</td>
                                <td class="text-dim">"Fasthttp SSR"</td>
                                <td class="num text-paper">"82,400 req/s"</td>
                                <td class="num">"0.78 ms"</td>
                                <td class="num">"2.90 ms"</td>
                                <td class="num">"22 MB → 72 MB"</td>
                                <td class="num text-ok" data-i18n="bench.matrix.tax_none">"0 kB (None)"</td>
                            </tr>

                            // FASTIFY
                            <tr class="matrix-row" data-cat="node">
                                <td class="font-medium text-paper">"fastify 4.28"</td>
                                <td>"Node.js 20 · V8 JIT"</td>
                                <td class="text-dim">"Event Loop Single-Thread"</td>
                                <td class="num text-paper">"34,100 req/s"</td>
                                <td class="num">"2.10 ms"</td>
                                <td class="num">"7.80 ms"</td>
                                <td class="num text-bad">"45 MB → 180 MB"</td>
                                <td class="num text-dim" data-i18n="bench.matrix.tax_varies">"Varies"</td>
                            </tr>

                            // NEXT.JS 15
                            <tr class="matrix-row" data-cat="node">
                                <td class="font-medium text-paper">"next.js 15.1"</td>
                                <td>"Node.js 20 · React 19"</td>
                                <td class="text-dim">"Server Components + SSR"</td>
                                <td class="num text-bad font-semibold">"8,900 req/s"</td>
                                <td class="num text-bad">"8.40 ms"</td>
                                <td class="num text-bad font-semibold">"28.50 ms"</td>
                                <td class="num text-bad">"120 MB → 420 MB"</td>
                                <td class="num text-bad font-semibold">"480 kB JS"</td>
                            </tr>

                            // SVELTEKIT
                            <tr class="matrix-row" data-cat="node">
                                <td class="font-medium text-paper">"sveltekit 2.5"</td>
                                <td>"Node.js 20 · Node Adapter"</td>
                                <td class="text-dim">"Compiled Hydration SSR"</td>
                                <td class="num text-paper">"14,200 req/s"</td>
                                <td class="num">"5.20 ms"</td>
                                <td class="num">"18.40 ms"</td>
                                <td class="num text-bad">"90 MB → 280 MB"</td>
                                <td class="num text-dim">"85 kB JS"</td>
                            </tr>

                            // FASTAPI
                            <tr class="matrix-row" data-cat="python">
                                <td class="font-medium text-paper">"fastapi 0.111"</td>
                                <td>"Python 3.12 · Uvicorn"</td>
                                <td class="text-dim">"Asyncio Event Loop"</td>
                                <td class="num text-bad">"6,400 req/s"</td>
                                <td class="num text-bad">"12.10 ms"</td>
                                <td class="num text-bad">"45.20 ms"</td>
                                <td class="num">"38 MB → 110 MB"</td>
                                <td class="num text-ok">"0 kB"</td>
                            </tr>

                            // RAILS
                            <tr class="matrix-row" data-cat="python">
                                <td class="font-medium text-paper">"ruby on rails 7.2"</td>
                                <td>"Ruby 3.3 · Puma 6"</td>
                                <td class="text-dim">"Multi-process SSR"</td>
                                <td class="num text-bad">"4,100 req/s"</td>
                                <td class="num text-bad">"18.50 ms"</td>
                                <td class="num text-bad">"68.40 ms"</td>
                                <td class="num text-bad">"85 MB → 310 MB"</td>
                                <td class="num text-dim">"120 kB Turbo"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            // Comparative Advantage Scorecard Cards
            <div class="grid md:grid-cols-3 gap-6 pt-8">
                <div class="plate p-6 bg-ink-2 space-y-2 border-l-2 border-l-oxide">
                    <div class="text-oxide font-bold font-mono text-[13px] flex items-center gap-2">
                        {render_icon("zap", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                        <span data-i18n="bench.matrix.sc1.title">"14.0× Faster than Next.js SSR"</span>
                    </div>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="bench.matrix.sc1.desc">
                        "While Next.js serializes JSON props and constructs dynamic Virtual DOM trees on Node.js, Yav streams compiled byte buffers directly from Rust thread memory with zero serialization penalty."
                    </p>
                </div>

                <div class="plate p-6 bg-ink-2 space-y-2 border-l-2 border-l-ok">
                    <div class="text-ok font-bold font-mono text-[13px] flex items-center gap-2">
                        {render_icon("database", IconSize::Sm, Some("w-4 h-4 text-ok"), None)}
                        <span data-i18n="bench.matrix.sc2.title">"17.5× Less Memory RSS"</span>
                    </div>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="bench.matrix.sc2.desc">
                        "Node.js and V8 garbage collectors hold hundreds of megabytes of object trees under load. Yav's native RAII memory management drops memory instantly the moment a socket closes."
                    </p>
                </div>

                <div class="plate p-6 bg-ink-2 space-y-2 border-l-2 border-l-paper">
                    <div class="text-paper font-bold font-mono text-[13px] flex items-center gap-2">
                        {render_icon("shield-check", IconSize::Sm, Some("w-4 h-4 text-paper"), None)}
                        <span data-i18n="bench.matrix.sc3.title">"97.3% of Raw Actix-Web"</span>
                    </div>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="bench.matrix.sc3.desc">
                        "You get full file routing, layouts, typed component props, and cryptographic HMAC sessions with less than a 2.7% performance delta compared to hand-rolled raw Actix-web code."
                    </p>
                </div>
            </div>
        </section>
    }
}
