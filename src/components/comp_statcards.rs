//! 5.3 KPI & Data StatCards Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentStatCards() -> Node {
    rsx! {
        <section id="statcards" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.3"</span>
                <span class="t">"KPI & Data Display"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add stat-card"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"High-density StatCards."</h2>
                <button
                    onclick="copyText(this, 'yav add stat-card')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add stat-card"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Engineered for telemetry dashboards, infrastructure monitoring, and analytical consoles. Displays primary metrics, relative trend deltas, baseline ratios, and auxiliary sparkline progress indicators."
            </p>

            // Live Interactive Preview Cards
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="plate p-6 space-y-4 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between text-[11px] font-mono text-dim">
                        <span>"THROUGHPUT"</span>
                        <span class="text-ok flex items-center gap-1 font-semibold">
                            "↑ +18.4%"
                        </span>
                    </div>
                    <div class="text-[36px] font-medium text-paper tracking-tight font-mono">
                        "142,500"
                    </div>
                    <div class="space-y-1.5 pt-2 border-t border-[rgba(236,233,227,0.08)]">
                        <div class="flex justify-between text-[11px] font-mono text-dim">
                            <span>"Requests / sec"</span>
                            <span class="text-paper">"p99: 0.28ms"</span>
                        </div>
                        <div class="hbar us" style="--w: 94%"><i></i></div>
                    </div>
                </div>

                <div class="plate p-6 space-y-4 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between text-[11px] font-mono text-dim">
                        <span>"BINARY FOOTPRINT"</span>
                        <span class="text-oxide flex items-center gap-1 font-semibold">
                            "↓ -94%"
                        </span>
                    </div>
                    <div class="text-[36px] font-medium text-paper tracking-tight font-mono">
                        "2.1 MB"
                    </div>
                    <div class="space-y-1.5 pt-2 border-t border-[rgba(236,233,227,0.08)]">
                        <div class="flex justify-between text-[11px] font-mono text-dim">
                            <span>"Distroless ELF"</span>
                            <span class="text-paper">"18 MB RAM"</span>
                        </div>
                        <div class="hbar us" style="--w: 14%"><i></i></div>
                    </div>
                </div>

                <div class="plate p-6 space-y-4 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between text-[11px] font-mono text-dim">
                        <span>"CLIENT HYDRATION"</span>
                        <span class="text-ok flex items-center gap-1 font-semibold">
                            "0 KB"
                        </span>
                    </div>
                    <div class="text-[36px] font-medium text-paper tracking-tight font-mono">
                        "0.0 ms"
                    </div>
                    <div class="space-y-1.5 pt-2 border-t border-[rgba(236,233,227,0.08)]">
                        <div class="flex justify-between text-[11px] font-mono text-dim">
                            <span>"JavaScript Runtime"</span>
                            <span class="text-ok">"Instant TTI"</span>
                        </div>
                        <div class="hbar us" style="--w: 0%"><i></i></div>
                    </div>
                </div>
            </div>

            // Code Preview Windows: Tabbed IDE + Terminal
            <div class="space-y-3 pt-2">
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/stat_card.rs"
                            </div>
                        </div>
                        <button onclick="copyText(this, 'stat_card(StatCardProps { title: \"Throughput\".into(), value: \"142,500 req/s\".into(), trend: Some(18.4), ..Default::default() })')" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// High-density Telemetry StatCard invocation"</div>
                            <div><span class="token-keyword">"use"</span>" yav::ui::prelude::*;"</div>
                            <div>
                                <span class="token-fn">"stat_card"</span>"(StatCardProps {"
                            </div>
                            <div>
                                "    title: "<span class="token-string">"\"Throughput\""</span>".into(),"
                            </div>
                            <div>
                                "    value: "<span class="token-string">"\"142,500\""</span>".into(),"
                            </div>
                            <div>
                                "    trend: "<span class="token-type">"Some"</span>"("<span class="token-type">"StatTrend"</span>" { value: "<span class="token-num">"18.4"</span>", dir: "<span class="token-type">"TrendDir"</span>"::"<span class="token-fn">"Up"</span>" }),"
                            </div>
                            <div>
                                "    subtext: "<span class="token-type">"Some"</span>"("<span class="token-string">"\"p99: 0.28ms\""</span>".into()),"
                            </div>
                            <div>
                                "    progress_ratio: "<span class="token-type">"Some"</span>"("<span class="token-num">"0.94"</span>"),"
                            </div>
                            <div>
                                "})"
                            </div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"Responsive 3-column grid · Pure CSS progress bar"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"Zero Hydration"</span>
                    </div>
                </div>

                // Terminal Scaffolding Command
                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-1.5">
                                <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                            </div>
                            <span class="text-faint text-[11px] ml-2">"bash — yav-cli scaffold"</span>
                        </div>
                        <button onclick="copyText(this, 'yav add stat-card')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add stat-card"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/stat_card.rs [1.6 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
