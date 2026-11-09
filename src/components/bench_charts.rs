//! Benchmark Interactive Charts Component: Multi-metric toggles, animated bars & delta analysis.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn BenchCharts() -> Node {
    let script = Node::raw(r#"<script>
    var currentMetric = 'throughput';
    var metricData = {
        throughput: {
            title: "Throughput (req/s)",
            note: "Higher is better · 200 persistent connections",
            dimension: "97.3% OF RAW ACTIX-WEB (ONLY 2.7% FRAMEWORK OVERHEAD)",
            max: 128000,
            reverse: false,
            unit: " req/s",
            items: [
                { name: "actix-web 4.9 · raw metal", val: 128000, display: "128,000", pct: 100, isUs: false },
                { name: "yav.rs 0.1.3 · compiled RSX", val: 124500, display: "124,500", pct: 97.3, isUs: true, ratio: "97.3%" },
                { name: "go gin 1.10 · SSR", val: 76200, display: "76,200", pct: 59.5, isUs: false },
                { name: "fastify 4.28 · node 20", val: 34100, display: "34,100", pct: 26.6, isUs: false },
                { name: "next.js 15 · node 20 SSR", val: 8900, display: "8,900", pct: 7.0, isUs: false },
                { name: "fastapi 0.111 · python 3.12", val: 6400, display: "6,400", pct: 5.0, isUs: false },
                { name: "ruby on rails 7.2 · puma 6", val: 4100, display: "4,100", pct: 3.2, isUs: false }
            ]
        },
        latency: {
            title: "p99 Tail Latency (ms)",
            note: "Lower is better · 99th percentile under sustained load",
            dimension: "25.4× LOWER TAIL LATENCY THAN NEXT.JS",
            max: 68.4,
            reverse: true,
            unit: " ms",
            items: [
                { name: "actix-web 4.9 · raw metal", val: 1.05, display: "1.05", pct: 1.5, isUs: false },
                { name: "yav.rs 0.1.3 · compiled RSX", val: 1.12, display: "1.12", pct: 1.6, isUs: true, ratio: "1.12 ms" },
                { name: "go gin 1.10 · SSR", val: 3.20, display: "3.20", pct: 4.7, isUs: false },
                { name: "fastify 4.28 · node 20", val: 7.80, display: "7.80", pct: 11.4, isUs: false },
                { name: "next.js 15 · node 20 SSR", val: 28.50, display: "28.50", pct: 41.7, isUs: false },
                { name: "fastapi 0.111 · python 3.12", val: 45.20, display: "45.20", pct: 66.1, isUs: false },
                { name: "ruby on rails 7.2 · puma 6", val: 68.40, display: "68.40", pct: 100.0, isUs: false }
            ]
        },
        memory: {
            title: "Memory Footprint under 1k conns (MB)",
            note: "Lower is better · RSS resident memory footprint",
            dimension: "17.5× LIGHTER MEMORY CONSUMPTION THAN NEXT.JS",
            max: 420,
            reverse: true,
            unit: " MB",
            items: [
                { name: "actix-web 4.9 · raw metal", val: 22, display: "22", pct: 5.2, isUs: false },
                { name: "yav.rs 0.1.3 · compiled RSX", val: 24, display: "24", pct: 5.7, isUs: true, ratio: "24 MB" },
                { name: "go gin 1.10 · SSR", val: 65, display: "65", pct: 15.5, isUs: false },
                { name: "fastify 4.28 · node 20", val: 180, display: "180", pct: 42.8, isUs: false },
                { name: "fastapi 0.111 · python 3.12", val: 110, display: "110", pct: 26.2, isUs: false },
                { name: "ruby on rails 7.2 · puma 6", val: 310, display: "310", pct: 73.8, isUs: false },
                { name: "next.js 15 · node 20 SSR", val: 420, display: "420", pct: 100.0, isUs: false }
            ]
        },
        efficiency: {
            title: "Core Efficiency (req/s per CPU core)",
            note: "Higher is better · Throughput divided by 8 vCPUs",
            dimension: "15,562 REQ/S PER HARDWARE CORE",
            max: 16000,
            reverse: false,
            unit: " req/s/core",
            items: [
                { name: "actix-web 4.9 · raw metal", val: 16000, display: "16,000", pct: 100.0, isUs: false },
                { name: "yav.rs 0.1.3 · compiled RSX", val: 15562, display: "15,562", pct: 97.3, isUs: true, ratio: "15,562" },
                { name: "go gin 1.10 · SSR", val: 9525, display: "9,525", pct: 59.5, isUs: false },
                { name: "fastify 4.28 · node 20", val: 4262, display: "4,262", pct: 26.6, isUs: false },
                { name: "next.js 15 · node 20 SSR", val: 1112, display: "1,112", pct: 7.0, isUs: false },
                { name: "fastapi 0.111 · python 3.12", val: 800, display: "800", pct: 5.0, isUs: false },
                { name: "ruby on rails 7.2 · puma 6", val: 512, display: "512", pct: 3.2, isUs: false }
            ]
        }
    };

    window.switchBenchMetric = function(m) {
        currentMetric = m;
        document.querySelectorAll('.bench-metric-tab').forEach(function(btn) {
            btn.classList.toggle('active', btn.getAttribute('data-metric') === m);
        });
        renderBenchChart();
    };

    function renderBenchChart() {
        var data = metricData[currentMetric];
        if (!data) return;

        var dimElem = document.getElementById('bench-chart-dimension');
        if (dimElem) dimElem.textContent = data.dimension;
        var noteElem = document.getElementById('bench-chart-note');
        if (noteElem) noteElem.textContent = data.note;

        var container = document.getElementById('bench-chart-rows');
        if (!container) return;

        var html = '';
        data.items.forEach(function(item, idx) {
            var isUs = item.isUs;
            var labelClass = isUs ? 'text-paper font-semibold' : 'text-dim';
            var valClass = isUs ? 'text-oxide font-bold' : 'text-mute';
            var barClass = isUs ? 'hbar us' : 'hbar';
            var dot = isUs ? '<span class=\"dot dot-ok pulse-dot mr-2\"></span>' : '';
            var ratioBadge = item.ratio ? '<span class=\"text-[#f07448] font-mono text-[11px] ml-1.5 font-normal\">(' + item.ratio + ')</span>' : '';

            // For metrics where lower is better (latency, memory), we visually invert the bar or display exact width
            var barWidth = item.pct + '%';

            html += '<div class=\"space-y-2\">' +
                '<div class=\"flex justify-between font-mono text-[13px]\">' +
                    '<span class=\"' + labelClass + ' flex items-center\">' + dot + item.name + '</span>' +
                    '<span class=\"' + valClass + '\">' + item.display + data.unit + ratioBadge + '</span>' +
                '</div>' +
                '<div class=\"' + barClass + '\"><i style=\"display:block;width:' + barWidth + ';transition:width 0.6s cubic-bezier(0.2, 0.8, 0.2, 1);\"></i></div>' +
                '<div class=\"bench-line\" style=\"width:100%\"></div>' +
            '</div>';
        });

        container.innerHTML = html;
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() { renderBenchChart(); });
    } else {
        renderBenchChart();
    }
    </script>"#);

    rsx! {
        <section class="wrap pt-12 sm:pt-16">
            {script}
            <div class="sec-head">
                <span class="n">"4.1"</span>
                <span class="t">"Multi-Metric Comparative Benchmarks"</span>
                <span class="sec-rule"></span>
                <span class="r">"wrk 4.2.0 · 8 vCPU"</span>
            </div>

            // Metric Selector Tabs
            <div class="pt-8 flex flex-wrap items-center justify-between gap-4">
                <div class="btn-group font-mono text-xs">
                    <button data-metric="throughput" onclick="switchBenchMetric('throughput')" class="btn tab-btn bench-metric-tab active">
                        "Throughput (req/s)"
                    </button>
                    <button data-metric="latency" onclick="switchBenchMetric('latency')" class="btn tab-btn bench-metric-tab">
                        "p99 Latency (ms)"
                    </button>
                    <button data-metric="memory" onclick="switchBenchMetric('memory')" class="btn tab-btn bench-metric-tab">
                        "Memory (MB)"
                    </button>
                    <button data-metric="efficiency" onclick="switchBenchMetric('efficiency')" class="btn tab-btn bench-metric-tab">
                        "Core Efficiency"
                    </button>
                </div>

                <div class="flex items-center gap-2 text-dim text-xs font-mono">
                    <span id="bench-chart-note">"Higher is better · 200 persistent connections"</span>
                    <button onclick="renderBenchChart()" class="p-1.5 hover:text-oxide transition-colors cursor-pointer" title="Replay chart animation">
                        {render_icon("refresh-cw", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                    </button>
                </div>
            </div>

            // Chart Container Plate
            <div class="plate bg-ink-2 p-6 sm:p-8 mt-6 space-y-6">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                
                // Delta dimension line
                <div class="dim-line pb-2">
                    <span>"EMPIRICAL DELTA"</span>
                    <div class="bar"></div>
                    <span id="bench-chart-dimension" class="text-oxide font-mono text-[11.5px] font-semibold">
                        "97.3% OF RAW ACTIX-WEB (ONLY 2.7% FRAMEWORK OVERHEAD)"
                    </span>
                </div>

                // Dynamic Bar Rows injected by JS
                <div id="bench-chart-rows" class="space-y-6">
                    // Fallback static row for instant rendering before JS triggers
                    <div class="space-y-2">
                        <div class="flex justify-between font-mono text-[13px]">
                            <span class="text-paper font-semibold flex items-center">
                                <span class="dot dot-ok pulse-dot mr-2"></span>
                                "yav.rs 0.1.3 · compiled RSX"
                            </span>
                            <span class="text-oxide font-bold">"124,500 req/s"</span>
                        </div>
                        <div class="hbar us"><i style="width:97.3%"></i></div>
                    </div>
                </div>

                // Bottom Chart Legend & Callout
                <div class="pt-4 border-t border-rule grid sm:grid-cols-3 gap-4 text-xs font-mono text-dim">
                    <div class="flex items-center gap-2">
                        <span class="w-3 h-3 rounded-sm bg-gradient-to-r from-oxide to-oxide-hi inline-block"></span>
                        <span class="text-paper">"Yav.rs"</span>
                        <span>"— Full SSR, RSX & sessions"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <span class="w-3 h-3 rounded-sm bg-dim inline-block"></span>
                        <span>"Compiled / Raw Metal"</span>
                        <span>"(Actix, Gin)"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <span class="w-3 h-3 rounded-sm bg-[rgba(236,233,227,0.1)] inline-block"></span>
                        <span>"Runtime Interpreted"</span>
                        <span>"(Node, Python, Ruby)"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
