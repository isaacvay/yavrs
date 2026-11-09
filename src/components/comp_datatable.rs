//! 5.4 Radix Data Table Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentDataTable() -> Node {
    rsx! {
        <section id="datatable" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.4"</span>
                <span class="t">"Radix Data Table"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add data-table"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"Server-driven data table."</h2>
                <button
                    onclick="copyText(this, 'yav add data-table')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add data-table"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Engineered for high-volume logs, telemetry records, and administrative interfaces. Features sortable column headers, HTTP status badges, latency badges, and row-level quick actions with pure server-rendered markup."
            </p>

            // Live Interactive Preview Plate
            <div class="plate overflow-hidden">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        "LIVE TELEMETRY TABLE — RADIX RADIX-GRID"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"4 of 24 routes compiled"</span>
                </div>

                // Filter & Search Controls bar
                <div class="p-4 bg-ink-2 border-b border-[rgba(236,233,227,0.08)] flex flex-col sm:flex-row items-center justify-between gap-4">
                    <div class="relative w-full sm:w-72">
                        <div class="absolute left-3 top-2.5 pointer-events-none text-dim">
                            {render_icon("search", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                        </div>
                        <input
                            type="text"
                            placeholder="Filter routes or methods..."
                            class="w-full bg-sunk border border-rule rounded pl-9 pr-3 py-1.5 text-xs font-mono text-paper placeholder:text-dim outline-none focus:border-oxide transition-colors"
                        />
                    </div>
                    <div class="flex items-center gap-2 w-full sm:w-auto justify-end">
                        <span class="text-[11px] font-mono text-dim">"Status:"</span>
                        <div class="flex items-center border border-rule bg-sunk rounded text-xs font-mono">
                            <button class="px-2.5 py-1 text-paper bg-ink border-r border-rule font-medium">"All"</button>
                            <button class="px-2.5 py-1 text-dim hover:text-paper border-r border-rule">"2xx OK"</button>
                            <button class="px-2.5 py-1 text-dim hover:text-paper">"Error"</button>
                        </div>
                    </div>
                </div>

                // Data Table Body
                <div class="overflow-x-auto bg-[#09090b]">
                    <table class="w-full text-left font-mono text-[12.5px] border-collapse">
                        <thead>
                            <tr class="border-b border-[rgba(236,233,227,0.08)] text-[11px] text-dim uppercase tracking-wider bg-ink-2/60">
                                <th class="py-3 px-4 font-medium">"Route Endpoint"</th>
                                <th class="py-3 px-4 font-medium">"Method"</th>
                                <th class="py-3 px-4 font-medium">"Status"</th>
                                <th class="py-3 px-4 font-medium">"p99 Latency"</th>
                                <th class="py-3 px-4 font-medium">"Memory RSS"</th>
                                <th class="py-3 px-4 font-medium text-right">"Action"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-[rgba(236,233,227,0.06)]">
                            <tr class="hover:bg-[rgba(236,233,227,0.02)] transition-colors">
                                <td class="py-3 px-4 text-paper font-semibold flex items-center gap-2">
                                    <span class="w-1.5 h-1.5 rounded-full bg-ok"></span>
                                    "/"
                                </td>
                                <td class="py-3 px-4">
                                    <span class="px-2 py-0.5 rounded text-[10px] bg-ok/10 text-ok border border-ok/20 font-bold">"GET"</span>
                                </td>
                                <td class="py-3 px-4 text-paper">
                                    <span class="chip text-[11px]">"200 OK"</span>
                                </td>
                                <td class="py-3 px-4 text-ok">"0.18 ms"</td>
                                <td class="py-3 px-4 text-dim">"14 KB"</td>
                                <td class="py-3 px-4 text-right">
                                    <button class="btn btn-sm btn-line text-[11px] py-0.5 px-2">"Replay"</button>
                                </td>
                            </tr>

                            <tr class="hover:bg-[rgba(236,233,227,0.02)] transition-colors">
                                <td class="py-3 px-4 text-paper font-semibold flex items-center gap-2">
                                    <span class="w-1.5 h-1.5 rounded-full bg-oxide"></span>
                                    "/api/v1/checkout"
                                </td>
                                <td class="py-3 px-4">
                                    <span class="px-2 py-0.5 rounded text-[10px] bg-oxide/10 text-oxide border border-oxide/20 font-bold">"POST"</span>
                                </td>
                                <td class="py-3 px-4 text-paper">
                                    <span class="chip text-[11px]">"201 Created"</span>
                                </td>
                                <td class="py-3 px-4 text-oxide">"0.42 ms"</td>
                                <td class="py-3 px-4 text-dim">"32 KB"</td>
                                <td class="py-3 px-4 text-right">
                                    <button class="btn btn-sm btn-line text-[11px] py-0.5 px-2">"Inspect"</button>
                                </td>
                            </tr>

                            <tr class="hover:bg-[rgba(236,233,227,0.02)] transition-colors">
                                <td class="py-3 px-4 text-paper font-semibold flex items-center gap-2">
                                    <span class="w-1.5 h-1.5 rounded-full bg-ok"></span>
                                    "/benchmarks"
                                </td>
                                <td class="py-3 px-4">
                                    <span class="px-2 py-0.5 rounded text-[10px] bg-ok/10 text-ok border border-ok/20 font-bold">"GET"</span>
                                </td>
                                <td class="py-3 px-4 text-paper">
                                    <span class="chip text-[11px]">"200 OK"</span>
                                </td>
                                <td class="py-3 px-4 text-ok">"0.24 ms"</td>
                                <td class="py-3 px-4 text-dim">"18 KB"</td>
                                <td class="py-3 px-4 text-right">
                                    <button class="btn btn-sm btn-line text-[11px] py-0.5 px-2">"Replay"</button>
                                </td>
                            </tr>

                            <tr class="hover:bg-[rgba(236,233,227,0.02)] transition-colors">
                                <td class="py-3 px-4 text-paper font-semibold flex items-center gap-2">
                                    <span class="w-1.5 h-1.5 rounded-full bg-blue-400"></span>
                                    "/ws/telemetry"
                                </td>
                                <td class="py-3 px-4">
                                    <span class="px-2 py-0.5 rounded text-[10px] bg-blue-500/10 text-blue-400 border border-blue-500/20 font-bold">"WS"</span>
                                </td>
                                <td class="py-3 px-4 text-paper">
                                    <span class="chip text-[11px]">"101 Switch"</span>
                                </td>
                                <td class="py-3 px-4 text-ok">"0.08 ms"</td>
                                <td class="py-3 px-4 text-dim">"8 KB"</td>
                                <td class="py-3 px-4 text-right">
                                    <button class="btn btn-sm btn-line text-[11px] py-0.5 px-2">"Logs"</button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                // Table Pagination Footer
                <div class="p-3 bg-ink-2 border-t border-[rgba(236,233,227,0.08)] flex items-center justify-between text-xs font-mono">
                    <span class="text-dim">"Showing 1–4 of 24 routes"</span>
                    <div class="flex items-center gap-2">
                        <button class="btn btn-sm btn-line text-[11px] py-1 px-2.5">"← Prev"</button>
                        <span class="px-2 py-1 bg-ink border border-oxide text-paper font-semibold text-[11px]">"1"</span>
                        <span class="px-2 py-1 text-dim hover:text-paper cursor-pointer text-[11px]">"2"</span>
                        <span class="px-2 py-1 text-dim hover:text-paper cursor-pointer text-[11px]">"3"</span>
                        <button class="btn btn-sm btn-line text-[11px] py-1 px-2.5">"Next →"</button>
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
                                "src/components/data_table.rs"
                            </div>
                        </div>
                        <button onclick="copyText(this, 'data_table(DataTableProps { columns: vec![\"Route\", \"Method\", \"p99\"], rows })')" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Server-side Data Table rendering"</div>
                            <div><span class="token-tag">"<table "</span><span class="token-attr">"class="</span><span class="token-string">"\"w-full font-mono text-xs border-collapse\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<thead>"</span><span class="token-tag">"<tr "</span><span class="token-attr">"class="</span><span class="token-string">"\"border-b border-rule text-dim\""</span><span class="token-tag">">"</span>"...items..."<span class="token-tag">"</tr>"</span><span class="token-tag">"</thead>"</span></div>
                            <div>"    "<span class="token-tag">"<tbody>"</span></div>
                            <div>"        "<span>"{"</span><span class="token-var">"routes"</span>"."<span class="token-fn">"iter"</span>"()."<span class="token-fn">"map"</span>"(|r| "<span class="token-macro">"rsx!"</span>" { "<span class="token-tag">"<RouteRow "</span><span class="token-attr">"route="</span>"{r}"<span class="token-tag">" />"</span>" })"<span>"}"</span></div>
                            <div>"    "<span class="token-tag">"</tbody>"</span></div>
                            <div><span class="token-tag">"</table>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"Zero client JS hydration · 100% SEO indexable markup"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"Streaming Compatible"</span>
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
                        <button onclick="copyText(this, 'yav add data-table')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add data-table"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/data_table.rs [2.4 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
