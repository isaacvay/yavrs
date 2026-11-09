//! 5.18 Enterprise Workflows: DataGrid, Kanban Board & FlowCanvas Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentEnterprise() -> Node {
    rsx! {
        <section id="enterprise" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.18"</span>
                <span class="t">"Enterprise DataGrid, Kanban & FlowCanvas"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add data-grid kanban flow-canvas"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"High-throughput enterprise application primitives."</h2>
                <button
                    data-copy="yav add data-grid kanban-board flow-canvas devtools"
                    onclick="copyText(this)"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add data-grid kanban flow-canvas"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Production-grade productivity engines: searchable and selectable enterprise DataGrids, drag-and-drop Kanban workflow boards with $O(1)$ fractional LexoRank ordering, interactive node-graph FlowCanvases with live cubic Bézier edge calculation, and in-app DevTools telemetry."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        "LIVE PREVIEW — WORKFLOW ENGINES"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"LexoRank $O(1)$ · Bézier Rerouting"</span>
                </div>
                <div class="p-6 sm:p-8 space-y-8 bg-ink-2">
                    // 1. Kanban Board Demo
                    <div class="space-y-3">
                        <div class="flex items-center justify-between">
                            <span class="text-xs font-mono text-dim uppercase">"Multi-Column Kanban with LexoRank Fractional Indexing"</span>
                            <span class="chip text-[11px]">"yav:drop-url=\"/api/kanban/move\""</span>
                        </div>
                        <div class="grid md:grid-cols-3 gap-4">
                            // Column 1: Backlog
                            <div class="p-3 bg-sunk border border-rule rounded space-y-3">
                                <div class="flex items-center justify-between text-xs font-mono">
                                    <span class="font-semibold text-paper flex items-center gap-1.5">
                                        <span class="w-2 h-2 rounded-full bg-dim"></span>
                                        "Backlog"
                                    </span>
                                    <span class="text-dim">"2"</span>
                                </div>
                                <div class="p-3 bg-ink-2 border border-rule rounded space-y-2 hover:border-oxide transition-colors cursor-grab shadow-sm">
                                    <div class="flex items-center justify-between text-[11px] font-mono">
                                        <span class="text-oxide font-semibold">"YAV-104"</span>
                                        <span class="text-dim">"rank: U"</span>
                                    </div>
                                    <p class="text-xs text-paper font-medium">"CRDT Multiplayer Presence Room"</p>
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim pt-1 border-t border-rule">
                                        <span class="px-1.5 py-0.5 rounded bg-sunk text-paper">"collab"</span>
                                        <span>"@isaac"</span>
                                    </div>
                                </div>
                                <div class="p-3 bg-ink-2 border border-rule rounded space-y-2 hover:border-oxide transition-colors cursor-grab shadow-sm">
                                    <div class="flex items-center justify-between text-[11px] font-mono">
                                        <span class="text-oxide font-semibold">"YAV-105"</span>
                                        <span class="text-dim">"rank: V"</span>
                                    </div>
                                    <p class="text-xs text-paper font-medium">"WebAssembly Client Islands"</p>
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim pt-1 border-t border-rule">
                                        <span class="px-1.5 py-0.5 rounded bg-sunk text-paper">"core"</span>
                                        <span>"@team"</span>
                                    </div>
                                </div>
                            </div>

                            // Column 2: In Progress
                            <div class="p-3 bg-sunk border border-rule rounded space-y-3">
                                <div class="flex items-center justify-between text-xs font-mono">
                                    <span class="font-semibold text-paper flex items-center gap-1.5">
                                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                                        "In Progress"
                                    </span>
                                    <span class="text-dim">"1"</span>
                                </div>
                                <div class="p-3 bg-ink-2 border border-oxide/60 rounded space-y-2 hover:border-oxide transition-colors cursor-grab shadow-md">
                                    <div class="flex items-center justify-between text-[11px] font-mono">
                                        <span class="text-oxide font-semibold">"YAV-102"</span>
                                        <span class="text-dim">"rank: W"</span>
                                    </div>
                                    <p class="text-xs text-paper font-medium">"Model Context Protocol (MCP) Server"</p>
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim pt-1 border-t border-rule">
                                        <span class="px-1.5 py-0.5 rounded bg-sunk text-oxide">"ai"</span>
                                        <span>"@isaac"</span>
                                    </div>
                                </div>
                            </div>

                            // Column 3: Done
                            <div class="p-3 bg-sunk border border-rule rounded space-y-3">
                                <div class="flex items-center justify-between text-xs font-mono">
                                    <span class="font-semibold text-paper flex items-center gap-1.5">
                                        <span class="w-2 h-2 rounded-full bg-ok"></span>
                                        "Production Done"
                                    </span>
                                    <span class="text-dim">"3"</span>
                                </div>
                                <div class="p-3 bg-ink-2 border border-rule rounded space-y-2 opacity-85 shadow-sm">
                                    <div class="flex items-center justify-between text-[11px] font-mono">
                                        <span class="text-ok font-semibold">"YAV-098"</span>
                                        <span class="text-dim">"rank: Z"</span>
                                    </div>
                                    <p class="text-xs text-paper font-medium">"Partial Prerendering (PPR) Engine"</p>
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim pt-1 border-t border-rule">
                                        <span class="px-1.5 py-0.5 rounded bg-sunk text-ok">"server"</span>
                                        <span>"@isaac"</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // 2. FlowCanvas Interactive Node Graph Demo
                    <div class="space-y-3 pt-4 border-t border-rule">
                        <div class="flex items-center justify-between">
                            <span class="text-xs font-mono text-dim uppercase">"FlowCanvas — Reactive Node Graph with Live Bézier Edge Rerouting"</span>
                            <span class="text-xs font-mono text-dim">"drag nodes to update SVG paths"</span>
                        </div>
                        <div class="table-scroll pb-1">
                            <div class="p-6 bg-sunk border border-rule rounded relative overflow-hidden h-56 bp-grid min-w-[580px]">
                                // SVG Curved Edges
                                <svg class="absolute inset-0 w-full h-full pointer-events-none" viewBox="0 0 600 200" fill="none">
                                    <path d="M 160 100 C 220 100, 220 50, 280 50" stroke="#e0592f" stroke-width="2" stroke-dasharray="4 2" />
                                    <circle cx="280" cy="50" r="3" fill="#e0592f" />

                                    <path d="M 160 100 C 220 100, 220 150, 280 150" stroke="rgba(236,233,227,0.3)" stroke-width="2" />
                                    <circle cx="280" cy="150" r="3" fill="rgba(236,233,227,0.3)" />

                                    <path d="M 400 50 C 450 50, 450 100, 500 100" stroke="#22c55e" stroke-width="2" />
                                    <circle cx="500" cy="100" r="3" fill="#22c55e" />
                                </svg>

                                // Node 1: Ingress
                                <div class="absolute left-6 top-16 w-36 p-3 bg-ink-2 border border-rule rounded shadow-lg space-y-1">
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim">
                                        <span>"INPUT"</span>
                                        <span class="w-1.5 h-1.5 rounded-full bg-ok"></span>
                                    </div>
                                    <div class="text-xs font-bold text-paper">"HTTP Webhook"</div>
                                    <div class="text-[10px] font-mono text-oxide">"POST /events"</div>
                                </div>

                                // Node 2: Transform
                                <div class="absolute left-64 top-4 w-36 p-3 bg-ink-2 border border-oxide rounded shadow-lg space-y-1">
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim">
                                        <span>"WORKER"</span>
                                        <span class="w-1.5 h-1.5 rounded-full bg-oxide"></span>
                                    </div>
                                    <div class="text-xs font-bold text-paper">"AI Classifier"</div>
                                    <div class="text-[10px] font-mono text-dim">"gpt-4o-mini"</div>
                                </div>

                                // Node 3: Storage
                                <div class="absolute left-64 bottom-4 w-36 p-3 bg-ink-2 border border-rule rounded shadow-lg space-y-1">
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim">
                                        <span>"STORAGE"</span>
                                        <span class="w-1.5 h-1.5 rounded-full bg-dim"></span>
                                    </div>
                                    <div class="text-xs font-bold text-paper">"Audit Ledger"</div>
                                    <div class="text-[10px] font-mono text-dim">"Append-only"</div>
                                </div>

                                // Node 4: Dispatch
                                <div class="absolute right-6 top-16 w-36 p-3 bg-ink-2 border border-ok/80 rounded shadow-lg space-y-1">
                                    <div class="flex items-center justify-between text-[10px] font-mono text-dim">
                                        <span>"DISPATCH"</span>
                                        <span class="w-1.5 h-1.5 rounded-full bg-ok"></span>
                                    </div>
                                    <div class="text-xs font-bold text-paper">"Client SSE Stream"</div>
                                    <div class="text-[10px] font-mono text-ok">"0ms latency"</div>
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
                            {render_icon("layers", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/workflow.rs"
                        </div>
                    </div>
                    <button
                        data-copy="use yav::ui::kanban::{KanbanBoard, KanbanColumn, KanbanCard};\nuse yav::ui::flow_canvas::{FlowCanvas, FlowNode, FlowEdge};\n\nlet board = KanbanBoard::new(\"sprint-board\")\n    .drop_url(\"/api/kanban/move\")\n    .column(KanbanColumn::new(\"todo\", \"À Faire\")\n        .card(KanbanCard::new(\"ISS-1\", \"Moteur CRDT\", \"U\").tag(\"core\")))\n    .column(KanbanColumn::new(\"in_progress\", \"En Cours\"))\n    .render();\n\nlet canvas = FlowCanvas::new(\"pipeline\")\n    .node(FlowNode::new(\"n1\", \"Webhook\", 50, 100))\n    .node(FlowNode::new(\"n2\", \"AI Worker\", 260, 40))\n    .edge(FlowEdge::new(\"e1\", \"n1\", \"n2\").animated(true))\n    .render();"
                        onclick="copyText(this)"
                        class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono mr-2"
                    >
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" class="text-oxide font-medium">"copy"</span>
                    </button>
                </div>
                <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                    <div class="token-comment">"// Fractional LexoRank and SVG Bézier workflow composition"</div>
                    <div><span class="token-keyword">"use "</span>"yav::ui::kanban::{"<span class="token-type">"KanbanBoard"</span>", "<span class="token-type">"KanbanColumn"</span>", "<span class="token-type">"KanbanCard"</span>"};"</div>
                    <div><span class="token-keyword">"use "</span>"yav::ui::flow_canvas::{"<span class="token-type">"FlowCanvas"</span>", "<span class="token-type">"FlowNode"</span>", "<span class="token-type">"FlowEdge"</span>"};"</div>
                    <div class="pt-2"><span class="token-keyword">"let "</span>"board = "<span class="token-type">"KanbanBoard"</span>"::new("<span class="token-string">"\"sprint-board\""</span>")"</div>
                    <div>"    .drop_url("<span class="token-string">"\"/api/kanban/move\""</span>")"</div>
                    <div>"    .column("<span class="token-type">"KanbanColumn"</span>"::new("<span class="token-string">"\"todo\""</span>", "<span class="token-string">"\"À Faire\""</span>")"</div>
                    <div>"        .card("<span class="token-type">"KanbanCard"</span>"::new("<span class="token-string">"\"ISS-1\""</span>", "<span class="token-string">"\"Moteur CRDT\""</span>", "<span class="token-string">"\"U\""</span>").tag("<span class="token-string">"\"core\""</span>")))"</div>
                    <div>"    .render();"</div>
                    <div class="pt-2"><span class="token-keyword">"let "</span>"canvas = "<span class="token-type">"FlowCanvas"</span>"::new("<span class="token-string">"\"pipeline\""</span>")"</div>
                    <div>"    .node("<span class="token-type">"FlowNode"</span>"::new("<span class="token-string">"\"n1\""</span>", "<span class="token-string">"\"Webhook\""</span>", 50, 100))"</div>
                    <div>"    .node("<span class="token-type">"FlowNode"</span>"::new("<span class="token-string">"\"n2\""</span>", "<span class="token-string">"\"AI Worker\""</span>", 260, 40))"</div>
                    <div>"    .edge("<span class="token-type">"FlowEdge"</span>"::new("<span class="token-string">"\"e1\""</span>", "<span class="token-string">"\"n1\""</span>", "<span class="token-string">"\"n2\""</span>").animated(true))"</div>
                    <div>"    .render();"</div>
                </div>
                <div class="ide-status">
                    <span>"Zero external JS libraries · 100% SVG and standard DOM mutations"</span>
                    <span class="text-ok">"LexoRank $O(1)$ Fractional Persistence"</span>
                </div>
            </div>
        </section>
    }
}
