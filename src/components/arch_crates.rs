//! Crate Composition Component with Refined Interactive Crate Inspector.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ArchCrates() -> Node {
    let script = Node::raw(r#"<script>
window.selectCrate = function(crateId) {
    var crates = {
        'core': {
            name: 'yav-core',
            badge: 'Tier 01 — Foundation Runtime',
            file: 'crates/yav-core/src/lib.rs',
            desc: 'The foundational substrate of Yav. Houses the core Node and Element abstractions, SIMD HTML escaping, fine-grained reactive Signals, StateMachine statecharts, CRDTs (LwwRegister, LwwMap), multiplayer PresenceRoom, LexoRank O(1) ordering, Local-First offline sync, and zero-overhead WebAssembly Client Islands.',
            deps: 'syn (build-only), serde, serde_json',
            code: '// yav-core: Foundation runtime & CRDTs\npub enum Node {\n    Element(Element),\n    Text(String),\n    Static(&\'static str),\n    Raw(String),\n    Fragment(Vec<Node>),\n}\n\n// CRDT collaboration & reactive Signals\nlet count = Signal::new(0);\nlet double = Computed::new(move || count.get() * 2);\nlet lww = LwwRegister::new("initial_state", 1);',
            codeHtml: '<span class="token-comment">// yav-core: Foundation runtime &amp; CRDTs</span>\n<span class="token-keyword">pub enum</span> <span class="token-type">Node</span> {\n    <span class="token-fn">Element</span>(<span class="token-type">Element</span>),\n    <span class="token-fn">Text</span>(<span class="token-type">String</span>),\n    <span class="token-fn">Static</span>(&amp;<span class="token-keyword">\'static</span> <span class="token-type">str</span>),\n    <span class="token-fn">Raw</span>(<span class="token-type">String</span>),\n    <span class="token-fn">Fragment</span>(<span class="token-type">Vec</span>&lt;<span class="token-type">Node</span>&gt;),\n}\n\n<span class="token-comment">// CRDT collaboration &amp; reactive Signals</span>\n<span class="token-keyword">let</span> count = <span class="token-type">Signal</span>::<span class="token-fn">new</span>(<span class="token-num">0</span>);\n<span class="token-keyword">let</span> double = <span class="token-type">Computed</span>::<span class="token-fn">new</span>(<span class="token-keyword">move</span> || count.<span class="token-fn">get</span>() * <span class="token-num">2</span>);\n<span class="token-keyword">let</span> lww = <span class="token-type">LwwRegister</span>::<span class="token-fn">new</span>(<span class="token-string">"initial_state"</span>, <span class="token-num">1</span>);'
        },
        'macros': {
            name: 'yav-macros',
            badge: 'Tier 01 — Proc-Macro Compiler',
            file: 'crates/yav-macros/src/lib.rs',
            desc: 'Contains the RSX parser, #[component] macro, #[server] Server Actions compiler, and code generators. Transforms declarative JSX-like syntax into zero-allocation buffer appends with native if/else and for control flow.',
            deps: 'proc-macro2, syn, quote',
            code: '// yav-macros: Compile-time specialization & Server Actions\n#[proc_macro]\npub fn rsx(input: TokenStream) -> TokenStream {\n    let parsed = syn::parse_macro_input!(input as RsxRoot);\n    parsed.generate_optimized_code().into()\n}\n\n#[proc_macro_attribute]\npub fn server(attr: TokenStream, item: TokenStream) -> TokenStream;',
            codeHtml: '<span class="token-comment">// yav-macros: Compile-time specialization &amp; Server Actions</span>\n<span class="token-macro">#[proc_macro]</span>\n<span class="token-keyword">pub fn</span> <span class="token-fn">rsx</span>(input: <span class="token-type">TokenStream</span>) -&gt; <span class="token-type">TokenStream</span> {\n    <span class="token-keyword">let</span> parsed = <span class="token-macro">syn::parse_macro_input!</span>(input <span class="token-keyword">as</span> <span class="token-type">RsxRoot</span>);\n    parsed.<span class="token-fn">generate_optimized_code</span>().<span class="token-fn">into</span>()\n}\n\n<span class="token-macro">#[proc_macro_attribute]</span>\n<span class="token-keyword">pub fn</span> <span class="token-fn">server</span>(attr: <span class="token-type">TokenStream</span>, item: <span class="token-type">TokenStream</span>) -&gt; <span class="token-type">TokenStream</span>;'
        },
        'server': {
            name: 'yav-server',
            badge: 'Tier 02 — Full-Stack HTTP Engine & AI SDK',
            file: 'crates/yav-server/src/lib.rs',
            desc: 'The unified server engine built on Axum 0.8 and Tokio. Powers Model Context Protocol (MCP) servers, native AI SDK & streaming UI, WebAuthn/Passkeys, OpenTelemetry distributed tracing, CircuitBreaker resilience, Outbox pattern, multi-region DB replicas, pure-Rust WebP pipeline, vector PDF signers, GraphQL, TurboStream, and Tus v1.0.0 resumable uploads.',
            deps: 'axum (0.8), tokio, hyper, tower, tower-http, sha2, hmac',
            code: '// yav-server: Enterprise full-stack server & AI engine\npub struct YavApp {\n    pub host: String,\n    pub port: u16,\n    router: axum::Router,\n}\n\nimpl YavApp {\n    pub fn mcp_server(mut self, server: McpServer) -> Self { ... }\n    pub fn passkeys(mut self, config: PasskeyConfig) -> Self { ... }\n    pub fn circuit_breaker(mut self, cb: CircuitBreaker) -> Self { ... }\n}',
            codeHtml: '<span class="token-comment">// yav-server: Enterprise full-stack server &amp; AI engine</span>\n<span class="token-keyword">pub struct</span> <span class="token-type">YavApp</span> {\n    <span class="token-keyword">pub</span> host: <span class="token-type">String</span>,\n    <span class="token-keyword">pub</span> port: <span class="token-type">u16</span>,\n    router: <span class="token-type">axum::Router</span>,\n}\n\n<span class="token-keyword">impl</span> <span class="token-type">YavApp</span> {\n    <span class="token-keyword">pub fn</span> <span class="token-fn">mcp_server</span>(<span class="token-keyword">mut</span> self, server: <span class="token-type">McpServer</span>) -&gt; <span class="token-type">Self</span> { ... }\n    <span class="token-keyword">pub fn</span> <span class="token-fn">passkeys</span>(<span class="token-keyword">mut</span> self, config: <span class="token-type">PasskeyConfig</span>) -&gt; <span class="token-type">Self</span> { ... }\n    <span class="token-keyword">pub fn</span> <span class="token-fn">circuit_breaker</span>(<span class="token-keyword">mut</span> self, cb: <span class="token-type">CircuitBreaker</span>) -&gt; <span class="token-type">Self</span> { ... }\n}'
        },
        'router': {
            name: 'yav-router',
            badge: 'Tier 02 — O(L) Radix Tree Router',
            file: 'crates/yav-router/src/lib.rs',
            desc: 'Next.js App Router conventions with O(L) Radix RouteTrie resolution. Supports file routing, Parallel Routes (@slot), Intercepting Routes ((.) / (..) / (...)), Optional Catch-All ([[...slug]]), and compile-time auto_routes! discovery with #[server] RPC auto-mounting.',
            deps: 'matchit, walkdir',
            code: '// yav-router: Radix prefix trie & parallel slots\nlet mut trie = RouteTrie::new();\ntrie.insert("/", root_handler)?;\ntrie.insert("/shop/[[...slug]]", shop_catchall)?;\ntrie.insert("/@analytics/page", analytics_slot)?;\n\n// Lookup complexity is O(L) independent of total routes\nlet matched = trie.resolve(request_path)?;',
            codeHtml: '<span class="token-comment">// yav-router: Radix prefix trie &amp; parallel slots</span>\n<span class="token-keyword">let</span> <span class="token-keyword">mut</span> trie = <span class="token-type">RouteTrie</span>::<span class="token-fn">new</span>();\ntrie.<span class="token-fn">insert</span>(<span class="token-string">"/"</span>, root_handler)?;\ntrie.<span class="token-fn">insert</span>(<span class="token-string">"/shop/[[...slug]]"</span>, shop_catchall)?;\ntrie.<span class="token-fn">insert</span>(<span class="token-string">"/@analytics/page"</span>, analytics_slot)?;\n\n<span class="token-comment">// Lookup complexity is O(L) independent of total routes</span>\n<span class="token-keyword">let</span> matched = trie.<span class="token-fn">resolve</span>(request_path)?;'
        },
        'cli': {
            name: 'yav-cli',
            badge: 'Tier 03 — Developer Toolchain (20 Cmds)',
            file: 'crates/yav-cli/src/main.rs',
            desc: 'Unified developer toolbelt with 20 commands: rapid project scaffolding, live reload server with WebSocket swap, standalone Tailwind JIT, SQLite/Postgres migrations (yav db) with SHA-256 journal, static site export (yav export), system diagnostics (yav doctor), route table inspection (yav routes), and cloud deploy generators.',
            deps: 'clap, notify, tokio, reqwest',
            code: '// yav-cli: Unified developer terminal suite\n$ yav dev               # Start live reload daemon + Tailwind watcher\n$ yav db migrate        # Execute SQL migrations with SHA-256 journal\n$ yav export            # Static Site Generation (SSG) to dist/\n$ yav doctor            # Full local system & toolchain diagnostic audit',
            codeHtml: '<span class="token-comment">// yav-cli: Unified developer terminal suite</span>\n<span class="token-keyword">$</span> yav dev               <span class="token-comment"># Start live reload daemon + Tailwind watcher</span>\n<span class="token-keyword">$</span> yav db migrate        <span class="token-comment"># Execute SQL migrations with SHA-256 journal</span>\n<span class="token-keyword">$</span> yav export            <span class="token-comment"># Static Site Generation (SSG) to dist/</span>\n<span class="token-keyword">$</span> yav doctor            <span class="token-comment"># Full local system &amp; toolchain diagnostic audit</span>'
        },
        'ui': {
            name: 'yav-ui',
            badge: 'Tier 03 — 60+ Enterprise Components & Themes',
            file: 'crates/yav-ui/src/lib.rs',
            desc: 'Library of 60+ enterprise UI primitives: server-rendered SVG Charts (Line, Bar, Donut, Sparkline, Heatmap, Sankey, Radar), DataGrid, KanbanBoard with O(1) LexoRank, FlowCanvas node graph with live Bézier curve rerouting, in-app DevTools HUD, Calendar, reactive Spreadsheet formula engine, 3D ModelViewer, and 8 design themes.',
            deps: 'yav-core, yav-macros, yav-icons',
            code: '// yav-ui: 60+ enterprise UI components\nuse yav::ui::charts::{LineChart, DonutChart};\nuse yav::ui::kanban::KanbanBoard;\nuse yav::ui::model_viewer_3d::ModelViewer3D;\n\nrsx! {\n    <LineChart id="perf" series={perf_data} />\n    <KanbanBoard id="sprint" drop_url="/api/move" />\n    <ModelViewer3D shape="cube" orbit_controls=true />\n}',
            codeHtml: '<span class="token-comment">// yav-ui: 60+ enterprise UI components</span>\n<span class="token-keyword">use</span> yav::ui::charts::{<span class="token-type">LineChart</span>, <span class="token-type">DonutChart</span>};\n<span class="token-keyword">use</span> yav::ui::kanban::<span class="token-type">KanbanBoard</span>;\n<span class="token-keyword">use</span> yav::ui::model_viewer_3d::<span class="token-type">ModelViewer3D</span>;\n\n<span class="token-macro">rsx!</span> {\n    <span class="token-tag">&lt;LineChart</span> <span class="token-attr">id</span>=<span class="token-string">"perf"</span> <span class="token-attr">series</span>={<span class="token-var">perf_data</span>} <span class="token-tag">/&gt;</span>\n    <span class="token-tag">&lt;KanbanBoard</span> <span class="token-attr">id</span>=<span class="token-string">"sprint"</span> <span class="token-attr">drop_url</span>=<span class="token-string">"/api/move"</span> <span class="token-tag">/&gt;</span>\n    <span class="token-tag">&lt;ModelViewer3D</span> <span class="token-attr">shape</span>=<span class="token-string">"cube"</span> <span class="token-attr">orbit_controls</span>=<span class="token-bool">true</span> <span class="token-tag">/&gt;</span>\n}'
        },
        'icons': {
            name: 'yav-icons',
            badge: 'Tier 03 — Vector Icon Engine',
            file: 'crates/yav-icons/src/lib.rs',
            desc: 'Over 13,000 pre-optimized SVG icons embedded directly into the binary across 3 design systems: Classic (Lucide), Minimal (MynaUI), and System (Phosphor). Zero network requests or webfont downloads.',
            deps: 'yav-core',
            code: '// yav-icons: Zero-request embedded SVGs\nuse yav::icons::{render_icon, IconSize};\n\n// Functional helper with size enum and custom classes\nlet icon = render_icon("zap", IconSize::Md, Some("text-oxide"), None);\n\n// Terminal inspection\n$ yav icon search "terminal"',
            codeHtml: '<span class="token-comment">// yav-icons: Zero-request embedded SVGs</span>\n<span class="token-keyword">use</span> yav::icons::{<span class="token-fn">render_icon</span>, <span class="token-type">IconSize</span>};\n\n<span class="token-comment">// Functional helper with size enum and custom classes</span>\n<span class="token-keyword">let</span> icon = <span class="token-fn">render_icon</span>(<span class="token-string">"zap"</span>, <span class="token-type">IconSize::Md</span>, <span class="token-type">Some</span>(<span class="token-string">"text-oxide"</span>), <span class="token-type">None</span>);\n\n<span class="token-comment">// Terminal inspection</span>\n<span class="token-keyword">$</span> yav icon search <span class="token-string">"terminal"</span>'
        },
        'build': {
            name: 'yav-build',
            badge: 'Tier 02 — Manifest & Asset Pipeline',
            file: 'crates/yav-build/src/lib.rs',
            desc: 'Manages package.yavml single-file manifest parsing, standalone Tailwind CSS JIT compilation without Node.js, asset fingerprinting, in-memory Brotli and Gzip pre-compression, and markdown parsing.',
            deps: 'flate2, toml, sha2',
            code: '// yav-build: Asset pipeline & compression\nlet manifest = PackageManifest::load("package.yavml")?;\n\n// Pre-compresses assets to .br and .gz in a single in-memory pass\nasset_pipeline::process_and_fingerprint("public", "dist")?;\n\n// Zero-dependency markdown parser with frontmatter\nlet post = parse_markdown(raw_markdown_str);',
            codeHtml: '<span class="token-comment">// yav-build: Asset pipeline &amp; compression</span>\n<span class="token-keyword">let</span> manifest = <span class="token-type">PackageManifest</span>::<span class="token-fn">load</span>(<span class="token-string">"package.yavml"</span>)?;\n\n<span class="token-comment">// Pre-compresses assets to .br and .gz in a single in-memory pass</span>\nasset_pipeline::<span class="token-fn">process_and_fingerprint</span>(<span class="token-string">"public"</span>, <span class="token-string">"dist"</span>)?;\n\n<span class="token-comment">// Zero-dependency markdown parser with frontmatter</span>\n<span class="token-keyword">let</span> post = <span class="token-fn">parse_markdown</span>(raw_markdown_str);'
        }
    };

    var data = crates[crateId];
    if (!data) return;

    var elName = document.getElementById('crate-detail-name');
    var elBadge = document.getElementById('crate-detail-badge');
    var elFile = document.getElementById('crate-detail-file');
    var elDesc = document.getElementById('crate-detail-desc');
    var elDeps = document.getElementById('crate-detail-deps');
    var elCode = document.getElementById('crate-detail-code');

    if (elName) elName.textContent = data.name;
    if (elBadge) elBadge.textContent = data.badge;
    if (elFile) elFile.textContent = data.file;
    if (elDesc) elDesc.textContent = data.desc;
    if (elDeps) elDeps.textContent = data.deps;
    if (elCode) elCode.innerHTML = data.codeHtml;
    var copyBtn = document.getElementById('crate-detail-copy');
    if (copyBtn) copyBtn.setAttribute('data-copy', data.code);

    // Toggle active state on list buttons
    var allKeys = ['core', 'macros', 'server', 'router', 'build', 'icons', 'ui', 'cli'];
    allKeys.forEach(function(k) {
        var el = document.getElementById('crate-btn-' + k);
        var bar = document.getElementById('crate-bar-' + k);
        if (el) {
            if (k === crateId) {
                el.classList.add('border-oxide', 'bg-sunk', 'text-paper');
                el.classList.remove('border-[rgba(236,233,227,0.08)]', 'text-dim');
            } else {
                el.classList.remove('border-oxide', 'bg-sunk', 'text-paper');
                el.classList.add('border-[rgba(236,233,227,0.08)]', 'text-dim');
            }
        }
        if (bar) {
            bar.style.opacity = (k === crateId) ? '1' : '0';
        }
    });

    // On mobile / small viewports, ensure plate stays visible
    if (window.innerWidth < 1024) {
        var plate = document.getElementById('crate-plate-container');
        if (plate) {
            plate.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        }
    }
};
</script>"#);

    rsx! {
        <section id="crates" class="wrap pt-12 sm:pt-16">
            // Section Header
            <div class="sec-head reveal">
                <span class="n">"2.3"</span>
                <span class="t">"Crate composition"</span>
                <span class="sec-rule"></span>
                <span class="r">"modular architecture"</span>
            </div>

            // Section Headline & Narrative Intro (Elevated above grid for perfect vertical alignment)
            <div class="pt-6 pb-6 max-w-3xl space-y-3 reveal pl-5 sm:pl-7 border-l-2 border-oxide/50">
                <h2 class="h-section text-paper leading-[1.18]">
                    "Small crates. "<span class="serif text-oxide">"Hard edges."</span>
                </h2>
                <p class="text-[15.5px] sm:text-[16px] text-mute leading-[1.8]">
                    "Yav is intentionally decomposed into focused crates with strict boundaries and zero circular dependencies. Each crate can be audited, imported or replaced independently."
                </p>
            </div>

            // Two-column layout: Left Selector + Right Master Inspector Plate (both start at same top)
            <div class="grid lg:grid-cols-12 gap-8 lg:gap-10 xl:gap-12 items-start">
                // Left Column: Crate List (starts at top, matching right plate)
                <div class="lg:col-span-5 space-y-3 font-mono text-[13px] reveal">
                    // 1. yav-core
                        <button
                            id="crate-btn-core"
                            onclick="window.selectCrate('core')"
                            class="relative w-full text-left py-4 px-5 border border-oxide bg-sunk text-paper transition-all cursor-pointer flex items-center justify-between group shadow-sm hover:border-oxide overflow-hidden"
                        >
                            <div id="crate-bar-core" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:1"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("layers", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 01 · Foundation"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-core"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"foundation"</span>
                        </button>

                        // 2. yav-macros
                        <button
                            id="crate-btn-macros"
                            onclick="window.selectCrate('macros')"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="crate-bar-macros" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("code", IconSize::Sm, Some("w-4 h-4 text-dim group-hover:text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 01 · Compiler"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-macros"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"proc macros"</span>
                        </button>

                        // 3. yav-server
                        <button
                            id="crate-btn-server"
                            onclick="window.selectCrate('server')"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="crate-bar-server" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("server", IconSize::Sm, Some("w-4 h-4 text-dim group-hover:text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 02 · Web Protocol"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-server"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"http engine"</span>
                        </button>

                        // 4. yav-router
                        <button
                            id="crate-btn-router"
                            onclick="window.selectCrate('router')"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="crate-bar-router" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("git-branch", IconSize::Sm, Some("w-4 h-4 text-dim group-hover:text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 02 · Routing"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-router"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"radix routing"</span>
                        </button>

                        // 5. yav-build
                        <button
                            id="crate-btn-build"
                            onclick="window.selectCrate('build')"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="crate-bar-build" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("file-text", IconSize::Sm, Some("w-4 h-4 text-dim group-hover:text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 02 · Manifest & Assets"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-build"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"pipeline"</span>
                        </button>

                        // 6. yav-icons
                        <button
                            id="crate-btn-icons"
                            onclick="window.selectCrate('icons')"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="crate-bar-icons" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("star", IconSize::Sm, Some("w-4 h-4 text-dim group-hover:text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 03 · 13k+ SVGs"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-icons"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"vector icons"</span>
                        </button>

                        // 7. yav-ui
                        <button
                            id="crate-btn-ui"
                            onclick="window.selectCrate('ui')"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="crate-bar-ui" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("box", IconSize::Sm, Some("w-4 h-4 text-dim group-hover:text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 03 · 60+ Primitives"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-ui"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"components"</span>
                        </button>

                        // 8. yav-cli
                        <button
                            id="crate-btn-cli"
                            onclick="window.selectCrate('cli')"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="crate-bar-cli" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3 pl-1">
                                {render_icon("terminal", IconSize::Sm, Some("w-4 h-4 text-dim group-hover:text-oxide"), None)}
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider uppercase">"Tier 03 · Toolchain"</span>
                                    <span class="font-semibold text-paper text-[14px] group-hover:text-oxide">"yav-cli"</span>
                                </div>
                            </div>
                            <span class="label text-[10.5px]">"20 commands"</span>
                        </button>
                    </div>

                // Right Column: Master Crate Detail Inspector Plate (Sticky so it stays visible alongside buttons)
                <div id="crate-plate-container" class="lg:col-span-7 lg:sticky lg:top-24 self-start reveal" style="--d:140ms">
                    <div class="plate bg-sunk shadow-2xl">
                        <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                        <div class="plate-head h-14 px-6 sm:px-8">
                            <span class="flex items-center gap-2.5 font-medium tracking-wide">
                                {render_icon("box", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                                <span id="crate-detail-name" class="font-mono text-[14px] text-paper">"yav-core"</span>
                            </span>
                            <span id="crate-detail-badge" class="text-oxide font-mono text-[11.5px] font-semibold">"Tier 01 — Foundation Runtime"</span>
                        </div>

                        <div class="p-7 sm:p-9 pl-8 sm:pl-10 space-y-7">
                            // Overview
                            <div class="space-y-2">
                                <span class="text-dim font-mono text-[11px] uppercase tracking-wider block">"Scope & Responsibilities"</span>
                                <p id="crate-detail-desc" class="text-[15px] text-mute leading-[1.8]">
                                    "The foundational substrate of Yav. Houses the core Node abstraction, high-performance SIMD HTML escaping algorithms, context stacks, and the JSON payload serializer for internationalization."
                                </p>
                            </div>

                            // Dependencies metadata bar
                            <div class="p-4 border border-[rgba(236,233,227,0.08)] bg-[rgba(236,233,227,0.02)] flex flex-wrap items-center justify-between gap-4 font-mono text-[12.5px]">
                                <span class="text-dim">"Cargo Dependencies:"</span>
                                <span id="crate-detail-deps" class="text-paper font-semibold">"syn (build-only), serde, serde_json"</span>
                            </div>

                            // Standalone Code interface
                            <div class="space-y-3">
                                <div class="flex items-center justify-between font-mono text-[11.5px] text-dim">
                                    <div class="flex items-center gap-2">
                                        <span class="uppercase tracking-wider">"Core Interface & Code Signature"</span>
                                        <span class="text-faint">"·"</span>
                                        <span id="crate-detail-file" class="text-paper font-medium">"crates/yav-core/src/lib.rs"</span>
                                    </div>
                                    <button
                                        id="crate-detail-copy"
                                        data-copy="// yav-core: High-performance primitive\npub enum Node {\n    Element(Element),\n    Text(String),\n    Static(&'static str),\n    Raw(String),\n    Fragment(Vec<Node>),\n}\n\n// Fast SIMD HTML escaping directly into contiguous buffer\npub fn escape_html_into(input: &str, out: &mut String) {\n    // Scans 16/32 bytes per instruction using AVX2 / ARM NEON\n}"
                                        onclick="copyText(this)"
                                        class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono"
                                    >
                                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                                        <span data-copy-label="" class="text-oxide font-medium">"copy snippet"</span>
                                    </button>
                                </div>
                                <pre class="p-6 bg-ink-2/60 border border-[rgba(236,233,227,0.08)] font-mono text-[13px] leading-[1.85] text-paper-2 overflow-x-auto"><code id="crate-detail-code">
                                    <span class="token-comment">"// yav-core: High-performance primitive"</span>{"\n"}
                                    <span class="token-keyword">"pub enum"</span>" "<span class="token-type">"Node"</span>" {"{"\n"}
                                    "    "<span class="token-fn">"Element"</span>"("<span class="token-type">"Element"</span>"),"{"\n"}
                                    "    "<span class="token-fn">"Text"</span>"("<span class="token-type">"String"</span>"),"{"\n"}
                                    "    "<span class="token-fn">"Static"</span>"(&"<span class="token-keyword">"'static"</span>" "<span class="token-type">"str"</span>"),"{"\n"}
                                    "    "<span class="token-fn">"Raw"</span>"("<span class="token-type">"String"</span>"),"{"\n"}
                                    "    "<span class="token-fn">"Fragment"</span>"("<span class="token-type">"Vec"</span>"&lt;"<span class="token-type">"Node"</span>"&gt;),"{"\n"}
                                    "}"{"\n\n"}
                                    <span class="token-comment">"// Fast SIMD HTML escaping directly into contiguous buffer"</span>{"\n"}
                                    <span class="token-keyword">"pub fn"</span>" "<span class="token-fn">"escape_html_into"</span>"(input: &"<span class="token-type">"str"</span>", out: &"<span class="token-keyword">"mut"</span>" "<span class="token-type">"String"</span>") {"{"\n"}
                                    "    "<span class="token-comment">"// Scans 16/32 bytes per instruction using AVX2 / ARM NEON"</span>{"\n"}
                                    "}"
                                </code></pre>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            {script}
        </section>
    }
}
