//! 5.17 Enterprise Server-Rendered SVG Charts & Visual Analytics Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

fn render_52w_heatmap_svg() -> Node {
    let mut s = String::with_capacity(16384);
    s.push_str("<svg viewBox=\"0 0 460 98\" class=\"w-full min-w-[460px] h-auto block select-none\" xmlns=\"http://www.w3.org/2000/svg\">");

    // Months labels
    let months = [
        ("Jan", 0),
        ("Fév", 4),
        ("Mar", 8),
        ("Avr", 13),
        ("Mai", 17),
        ("Juin", 21),
        ("Juil", 26),
        ("Août", 30),
        ("Sep", 35),
        ("Oct", 39),
        ("Nov", 43),
        ("Déc", 48),
    ];
    for (name, wk) in months {
        let x = 20.0 + (wk as f64) * 8.2;
        s.push_str(&format!(
            "<text x=\"{x:.1}\" y=\"10\" font-size=\"8.5\" fill=\"rgba(236,233,227,0.4)\" font-family=\"monospace\">{name}</text>"
        ));
    }

    // Day labels (Mon, Wed, Fri)
    s.push_str("<text x=\"4\" y=\"29.5\" font-size=\"8\" fill=\"rgba(236,233,227,0.35)\" font-family=\"monospace\">L</text>");
    s.push_str("<text x=\"4\" y=\"46.0\" font-size=\"8\" fill=\"rgba(236,233,227,0.35)\" font-family=\"monospace\">M</text>");
    s.push_str("<text x=\"4\" y=\"62.5\" font-size=\"8\" fill=\"rgba(236,233,227,0.35)\" font-family=\"monospace\">V</text>");

    // 52 columns x 7 rows
    let colors = [
        "rgba(236,233,227,0.06)",
        "rgba(224,89,47,0.28)",
        "rgba(224,89,47,0.52)",
        "rgba(224,89,47,0.78)",
        "#e0592f",
        "#22c55e",
    ];

    for w in 0..52 {
        for d in 0..7 {
            let x = 20.0 + (w as f64) * 8.2;
            let y = 14.0 + (d as f64) * 8.2;

            let hash = ((w * 37 + d * 67 + 13) % 101) as u32;
            let is_weekend = d == 0 || d == 6;

            let (color, count) = if is_weekend && hash > 45 {
                (colors[0], 0)
            } else if hash > 92 && !is_weekend {
                (colors[5], 14)
            } else if hash > 75 {
                (colors[4], 8)
            } else if hash > 50 {
                (colors[3], 5)
            } else if hash > 25 {
                (colors[2], 3)
            } else if hash > 10 {
                (colors[1], 1)
            } else {
                (colors[0], 0)
            };

            s.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"6\" height=\"6\" rx=\"1.2\" fill=\"{color}\"><title>Semaine {w_num}, Jour {d_num}: {count} d&#233;ploiements</title></rect>",
                w_num = w + 1,
                d_num = d + 1,
            ));
        }
    }

    // Bottom info and legend
    s.push_str("<text x=\"20\" y=\"91\" font-size=\"8\" fill=\"rgba(236,233,227,0.4)\" font-family=\"monospace\">1 482 d&#233;ploiements &#183; 0 &#233;chec p99</text>");

    s.push_str("<g transform=\"translate(315, 84)\">");
    s.push_str("<text x=\"-30\" y=\"7\" font-size=\"8\" fill=\"rgba(236,233,227,0.4)\" font-family=\"monospace\">Moins</text>");
    s.push_str("<rect x=\"0\" y=\"0\" width=\"7\" height=\"7\" rx=\"1.2\" fill=\"rgba(236,233,227,0.06)\" />");
    s.push_str("<rect x=\"10\" y=\"0\" width=\"7\" height=\"7\" rx=\"1.2\" fill=\"rgba(224,89,47,0.28)\" />");
    s.push_str("<rect x=\"20\" y=\"0\" width=\"7\" height=\"7\" rx=\"1.2\" fill=\"rgba(224,89,47,0.52)\" />");
    s.push_str("<rect x=\"30\" y=\"0\" width=\"7\" height=\"7\" rx=\"1.2\" fill=\"rgba(224,89,47,0.78)\" />");
    s.push_str("<rect x=\"40\" y=\"0\" width=\"7\" height=\"7\" rx=\"1.2\" fill=\"#e0592f\" />");
    s.push_str("<rect x=\"50\" y=\"0\" width=\"7\" height=\"7\" rx=\"1.2\" fill=\"#22c55e\" />");
    s.push_str("<text x=\"62\" y=\"7\" font-size=\"8\" fill=\"rgba(236,233,227,0.4)\" font-family=\"monospace\">Plus</text>");
    s.push_str("</g>");

    s.push_str("</svg>");
    Node::raw(s)
}

#[component]
pub fn ComponentCharts() -> Node {
    rsx! {
        <section id="charts" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.17"</span>
                <span class="t">"Enterprise SVG Charts"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add charts"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"Server-rendered vector charts. Zero client JS."</h2>
                <button
                    data-copy="yav add charts"
                    onclick="copyText(this)"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add charts"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Pure Rust SVG visualizations computed on the server: smooth cubic Bézier splines, categorical grouped bar charts, proportional donut segments, calendar heatmaps, and multi-axis radar charts. Lightweight, responsive, and completely immune to client-side charting library bloat."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        "LIVE PREVIEW — SERVER-RENDERED VECTOR CHARTS"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"100% SVG · Zero Canvas · Zero npm"</span>
                </div>
                <div class="p-6 sm:p-8 space-y-8 bg-ink-2">
                    // Grid 1: Line Chart & Donut Chart
                    <div class="grid lg:grid-cols-12 gap-6">
                        // Line Chart (8 cols)
                        <div class="lg:col-span-8 p-5 bg-sunk border border-rule rounded space-y-4">
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-xs font-mono text-dim block">"HTTP THROUGHPUT (24H)"</span>
                                    <span class="text-xl font-bold font-mono text-paper">"124,500 req/s"</span>
                                </div>
                                <div class="flex items-center gap-4 text-xs font-mono">
                                    <span class="flex items-center gap-1.5 text-oxide">
                                        <span class="w-2.5 h-2.5 rounded-sm bg-oxide inline-block"></span>
                                        "Edge Cluster"
                                    </span>
                                    <span class="flex items-center gap-1.5 text-ok">
                                        <span class="w-2.5 h-2.5 rounded-sm bg-ok inline-block"></span>
                                        "Primary DB"
                                    </span>
                                </div>
                            </div>
                            
                            // SVG Vector Spline Line Chart
                            <div class="w-full overflow-hidden">
                                <svg viewBox="0 0 540 180" class="w-full h-44 block" fill="none">
                                    <defs>
                                        <linearGradient id="chart-area-grad" x1="0" y1="0" x2="0" y2="1">
                                            <stop offset="0%" stop-color="#e0592f" stop-opacity="0.25" />
                                            <stop offset="100%" stop-color="#e0592f" stop-opacity="0.0" />
                                        </linearGradient>
                                        <linearGradient id="chart-ok-grad" x1="0" y1="0" x2="0" y2="1">
                                            <stop offset="0%" stop-color="#22c55e" stop-opacity="0.2" />
                                            <stop offset="100%" stop-color="#22c55e" stop-opacity="0.0" />
                                        </linearGradient>
                                    </defs>
                                    // Grid horizontal lines
                                    <line x1="40" y1="20" x2="520" y2="20" stroke="rgba(236,233,227,0.06)" stroke-dasharray="2 3" />
                                    <line x1="40" y1="60" x2="520" y2="60" stroke="rgba(236,233,227,0.06)" stroke-dasharray="2 3" />
                                    <line x1="40" y1="100" x2="520" y2="100" stroke="rgba(236,233,227,0.06)" stroke-dasharray="2 3" />
                                    <line x1="40" y1="140" x2="520" y2="140" stroke="rgba(236,233,227,0.12)" />

                                    // Y Axis labels
                                    <text x="32" y="24" text-anchor="end" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"150k"</text>
                                    <text x="32" y="64" text-anchor="end" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"100k"</text>
                                    <text x="32" y="104" text-anchor="end" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"50k"</text>
                                    <text x="32" y="144" text-anchor="end" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"0"</text>

                                    // Primary Curve Area & Line (Oxide)
                                    <path d="M 50 120 C 110 90, 160 40, 220 50 C 280 60, 320 30, 380 25 C 440 20, 480 35, 510 30 L 510 140 L 50 140 Z" fill="url(#chart-area-grad)" />
                                    <path d="M 50 120 C 110 90, 160 40, 220 50 C 280 60, 320 30, 380 25 C 440 20, 480 35, 510 30" stroke="#e0592f" stroke-width="2.5" />

                                    // Secondary Curve (Ok Green)
                                    <path d="M 50 135 C 120 125, 170 100, 220 95 C 280 90, 330 75, 380 70 C 440 65, 480 80, 510 75 L 510 140 L 50 140 Z" fill="url(#chart-ok-grad)" />
                                    <path d="M 50 135 C 120 125, 170 100, 220 95 C 280 90, 330 75, 380 70 C 440 65, 480 80, 510 75" stroke="#22c55e" stroke-width="2" stroke-dasharray="4 2" />

                                    // Metric Point Marker
                                    <circle cx="380" cy="25" r="4" fill="#e0592f" stroke="#0b0b0d" stroke-width="2" />

                                    // X Axis labels
                                    <text x="50" y="160" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"00:00"</text>
                                    <text x="165" y="160" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"06:00"</text>
                                    <text x="280" y="160" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"12:00"</text>
                                    <text x="395" y="160" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"18:00"</text>
                                    <text x="510" y="160" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"Now"</text>
                                </svg>
                            </div>
                        </div>

                        // Donut Chart (4 cols)
                        <div class="lg:col-span-4 p-5 bg-sunk border border-rule rounded space-y-4 flex flex-col justify-between">
                            <div>
                                <span class="text-xs font-mono text-dim block">"CACHE EFFICIENCY"</span>
                                <span class="text-xl font-bold font-mono text-paper">"98.4% Hit Rate"</span>
                            </div>

                            <div class="flex items-center justify-center py-2">
                                <svg viewBox="0 0 120 120" class="w-32 h-32 block">
                                    // Donut Segments
                                    <circle cx="60" cy="60" r="45" fill="none" stroke="rgba(236,233,227,0.06)" stroke-width="14" />
                                    <circle cx="60" cy="60" r="45" fill="none" stroke="#e0592f" stroke-width="14" stroke-dasharray="210 283" stroke-dashoffset="0" transform="rotate(-90 60 60)" />
                                    <circle cx="60" cy="60" r="45" fill="none" stroke="#22c55e" stroke-width="14" stroke-dasharray="60 283" stroke-dashoffset="-215" transform="rotate(-90 60 60)" />
                                    <circle cx="60" cy="60" r="45" fill="none" stroke="#6366f1" stroke-width="14" stroke-dasharray="13 283" stroke-dashoffset="-278" transform="rotate(-90 60 60)" />
                                    // Center text
                                    <text x="60" y="58" text-anchor="middle" font-size="14" font-weight="bold" fill="#ece9e3" font-family="monospace">"304"</text>
                                    <text x="60" y="72" text-anchor="middle" font-size="8" fill="rgba(236,233,227,0.4)" font-family="monospace">"NOT MODIFIED"</text>
                                </svg>
                            </div>

                            <div class="space-y-1.5 text-xs font-mono pt-2 border-t border-rule">
                                <div class="flex items-center justify-between">
                                    <span class="flex items-center gap-1.5 text-dim"><span class="w-2 h-2 rounded-full bg-oxide"></span>"In-Memory L1 (ETag)"</span>
                                    <span class="text-paper">"74%"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="flex items-center gap-1.5 text-dim"><span class="w-2 h-2 rounded-full bg-ok"></span>"XFetch Distributed"</span>
                                    <span class="text-paper">"21%"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="flex items-center gap-1.5 text-dim"><span class="w-2 h-2 rounded-full bg-[#6366f1]"></span>"Origin Render"</span>
                                    <span class="text-paper">"5%"</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Grid 2: Bar Chart, Sparkline & Activity Heatmap
                    <div class="grid md:grid-cols-12 gap-6">
                        // Categorical Bar Chart (6 cols)
                        <div class="md:col-span-6 p-5 bg-sunk border border-rule rounded space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="text-xs font-mono text-dim">"SSR LATENCY DISTRIBUTION (P50/P95/P99)"</span>
                                <span class="text-xs font-mono text-oxide">"µs scale"</span>
                            </div>
                            <svg viewBox="0 0 320 120" class="w-full h-32 block">
                                <line x1="20" y1="100" x2="310" y2="100" stroke="rgba(236,233,227,0.12)" />
                                // Bars
                                <rect x="35" y="65" width="28" height="35" fill="#e0592f" rx="2" />
                                <text x="49" y="58" text-anchor="middle" font-size="9" fill="#ece9e3" font-family="monospace">"280µs"</text>
                                <text x="49" y="114" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"Index"</text>

                                <rect x="95" y="45" width="28" height="55" fill="#e0592f" rx="2" />
                                <text x="109" y="38" text-anchor="middle" font-size="9" fill="#ece9e3" font-family="monospace">"440µs"</text>
                                <text x="109" y="114" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"Docs"</text>

                                <rect x="155" y="55" width="28" height="45" fill="#e0592f" rx="2" />
                                <text x="169" y="48" text-anchor="middle" font-size="9" fill="#ece9e3" font-family="monospace">"360µs"</text>
                                <text x="169" y="114" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"Crates"</text>

                                <rect x="215" y="25" width="28" height="75" fill="#e0592f" rx="2" />
                                <text x="229" y="18" text-anchor="middle" font-size="9" fill="#ece9e3" font-family="monospace">"620µs"</text>
                                <text x="229" y="114" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"Catalog"</text>

                                <rect x="275" y="75" width="28" height="25" fill="#22c55e" rx="2" />
                                <text x="289" y="68" text-anchor="middle" font-size="9" fill="#ece9e3" font-family="monospace">"190µs"</text>
                                <text x="289" y="114" text-anchor="middle" font-size="9" fill="rgba(236,233,227,0.4)" font-family="monospace">"Health"</text>
                            </svg>
                        </div>

                        // Activity Calendar Heatmap & Sparkline (6 cols)
                        <div class="md:col-span-6 p-5 bg-sunk border border-rule rounded space-y-3.5">
                            <div class="flex flex-wrap items-center justify-between gap-2 pb-2 border-b border-rule">
                                <div class="flex items-center gap-2">
                                    <span class="p-1 rounded bg-oxide/10 text-oxide border border-oxide/20">
                                        {render_icon("git-commit", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                    </span>
                                    <div>
                                        <div class="text-[11.5px] font-mono text-paper font-bold tracking-wider">
                                            "COMMIT DEPLOYMENT HEATMAP"
                                        </div>
                                        <div class="text-[10px] font-mono text-dim">
                                            "52 Semaines · Traçabilité CI/CD"
                                        </div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-1.5 px-2.5 py-1 rounded bg-[#22c55e]/10 border border-[#22c55e]/25">
                                    <span class="w-2 h-2 rounded-full bg-[#22c55e] inline-block animate-pulse"></span>
                                    <span class="text-[11px] font-mono text-ok font-semibold">"99.99% Up"</span>
                                </div>
                            </div>

                            // 52-Week SVG Heatmap
                            <div class="table-scroll pb-1">
                                {render_52w_heatmap_svg()}
                            </div>

                            // Micro-Trend Sparkline
                            <div class="pt-2.5 flex flex-wrap items-center justify-between text-[11px] font-mono text-dim border-t border-rule gap-2">
                                <div class="flex items-center gap-1.5">
                                    <span class="text-mute">"Cadence 7j :"</span>
                                    <span class="text-ok font-bold flex items-center gap-1">
                                        {render_icon("trending-up", IconSize::Sm, Some("w-3 h-3 text-ok"), None)}
                                        "+24.2%"
                                    </span>
                                </div>
                                <div class="flex items-center gap-2.5">
                                    <svg viewBox="0 0 100 20" class="w-24 h-5 block">
                                        <defs>
                                            <linearGradient id="heat-spark" x1="0" y1="0" x2="0" y2="1">
                                                <stop offset="0%" stop-color="#22c55e" stop-opacity="0.35" />
                                                <stop offset="100%" stop-color="#22c55e" stop-opacity="0.0" />
                                            </linearGradient>
                                        </defs>
                                        <polygon points="0,15 15,12 30,16 45,8 60,10 75,4 90,6 100,2 100,20 0,20" fill="url(#heat-spark)" />
                                        <path d="M 0 15 L 15 12 L 30 16 L 45 8 L 60 10 L 75 4 L 90 6 L 100 2" fill="none" stroke="#22c55e" stroke-width="1.8" stroke-linecap="round" />
                                    </svg>
                                    <span class="text-[10px] text-faint">"14.2s / run"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Code Specification Window
            <div class="ide-window">
                <div class="ide-head">
                    <div class="flex items-center h-full">
                        <div class="ide-tab">
                            {render_icon("activity", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/analytics.rs"
                        </div>
                    </div>
                    <button
                        data-copy="use yav::ui::charts::{LineChart, ChartSeries, DonutChart, DonutSegment};\n\nlet line_chart = LineChart::new(\"mrr-growth\")\n    .series(ChartSeries::new(\"Edge Cluster\", vec![32000.0, 58000.0, 124500.0]).color(\"#e0592f\"))\n    .labels(vec![\"Jan\", \"Feb\", \"Mar\"])\n    .height(240)\n    .render();\n\nlet donut = DonutChart::new(\"cache-ratio\")\n    .segment(DonutSegment::new(\"L1 ETag\", 74.0, \"#e0592f\"))\n    .segment(DonutSegment::new(\"XFetch L2\", 21.0, \"#22c55e\"))\n    .render();"
                        onclick="copyText(this)"
                        class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono mr-2"
                    >
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                    </button>
                </div>
                <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                    <div class="token-comment">"// 100% Server-side SVG rendering with zero client JS dependencies"</div>
                    <div><span class="token-keyword">"use "</span>"yav::ui::charts::{"<span class="token-type">"LineChart"</span>", "<span class="token-type">"ChartSeries"</span>", "<span class="token-type">"DonutChart"</span>", "<span class="token-type">"DonutSegment"</span>"};"</div>
                    <div class="pt-2"><span class="token-keyword">"let "</span>"line_chart = "<span class="token-type">"LineChart"</span>"::new("<span class="token-string">"\"mrr-growth\""</span>")"</div>
                    <div>"    .series("<span class="token-type">"ChartSeries"</span>"::new("<span class="token-string">"\"Edge Cluster\""</span>", "<span class="token-macro">"vec!"</span>"[32000.0, 58000.0, 124500.0]).color("<span class="token-string">"\"#e0592f\""</span>"))"</div>
                    <div>"    .labels(vec![\"Jan\", \"Feb\", \"Mar\"])"</div>
                    <div>"    .height(240)"</div>
                    <div>"    .render();"</div>
                    <div class="pt-2"><span class="token-keyword">"let "</span>"donut = "<span class="token-type">"DonutChart"</span>"::new("<span class="token-string">"\"cache-ratio\""</span>")"</div>
                    <div>"    .segment("<span class="token-type">"DonutSegment"</span>"::new("<span class="token-string">"\"L1 ETag\""</span>", 74.0, "<span class="token-string">"\"#e0592f\""</span>"))"</div>
                    <div>"    .segment("<span class="token-type">"DonutSegment"</span>"::new("<span class="token-string">"\"XFetch L2\""</span>", 21.0, "<span class="token-string">"\"#22c55e\""</span>"))"</div>
                    <div>"    .render();"</div>
                </div>
                <div class="ide-status">
                    <span>"Emits compact vector SVG markup into response stream"</span>
                    <span class="text-ok">"Zero Canvas / Zero WebGL / Zero WASM"</span>
                </div>
            </div>
        </section>
    }
}
