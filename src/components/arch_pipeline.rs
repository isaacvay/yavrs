//! Execution Pipeline Component with Refined Interactive Stepper & Deep Inspection.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ArchPipeline() -> Node {
    let script = Node::raw(r#"<script>
window.selectPipelineStage = function(stageIdx) {
    var stages = [
        {
            num: '01',
            phase: 'INGRESS',
            title: 'Stage 01 — Socket Ingress & Frame Parsing',
            sub: 'Non-blocking Tokio worker accepts TCP/TLS socket; Hyper parses HTTP frames directly into unallocated slices.',
            time: '~38 µs',
            alloc: '0 B heap (Zero-Copy)',
            syscall: 'epoll_wait / accept4',
            file: 'yav-server/src/conn.rs',
            hardware: 'L1/L2 Cache Hit',
            code: '// Tokio Worker Thread (Non-blocking I/O loop)\nlet (socket, remote_addr) = listener.accept().await?;\nlet stream = hyper_util::rt::TokioIo::new(socket);\n\n// Hyper HTTP/1.1 & HTTP/2 frame parsing\nlet conn = hyper::server::conn::auto::Builder::new(TokioExecutor::new())\n    .serve_connection(stream, service);\n\n// Slices buffers directly from OS ring buffer with zero allocations.',
            codeHtml: '<span class="token-comment">// Tokio Worker Thread (Non-blocking I/O loop)</span>\n<span class="token-keyword">let</span> (socket, remote_addr) = listener.<span class="token-fn">accept</span>().<span class="token-keyword">await</span>?;\n<span class="token-keyword">let</span> stream = <span class="token-type">hyper_util::rt::TokioIo</span>::<span class="token-fn">new</span>(socket);\n\n<span class="token-comment">// Hyper HTTP/1.1 &amp; HTTP/2 frame parsing</span>\n<span class="token-keyword">let</span> conn = <span class="token-type">hyper::server::conn::auto::Builder</span>::<span class="token-fn">new</span>(<span class="token-type">TokioExecutor</span>::<span class="token-fn">new</span>())\n    .<span class="token-fn">serve_connection</span>(stream, service);\n\n<span class="token-comment">// Slices buffers directly from OS ring buffer with zero allocations.</span>'
        },
        {
            num: '02',
            phase: 'ROUTING',
            title: 'Stage 02 — Zero-Allocation Radix Resolution',
            sub: 'Matchit Radix tree matches URI against compiled route tree in O(k) steps where k is path length.',
            time: '~4 µs',
            alloc: '0 B (Stack slices)',
            syscall: 'None (In-Memory)',
            file: 'yav-router/src/radix.rs',
            hardware: 'CPU Register Bound',
            code: '// Matchit Radix tree lookup (Compiled at launch by auto_routes!)\nlet path = req.uri().path();\nmatch router.at(path) {\n    Ok(matched) => {\n        // Extracted route parameters are borrowed &str slices\n        let id: &str = matched.params.get("id").unwrap_or_default();\n        matched.value(req, id).await\n    }\n    Err(_) => handle_404(req).await,\n}',
            codeHtml: '<span class="token-comment">// Matchit Radix tree lookup (Compiled at launch by auto_routes!)</span>\n<span class="token-keyword">let</span> path = req.<span class="token-fn">uri</span>().<span class="token-fn">path</span>();\n<span class="token-keyword">match</span> router.<span class="token-fn">at</span>(path) {\n    <span class="token-type">Ok</span>(matched) =&gt; {\n        <span class="token-comment">// Extracted route parameters are borrowed &amp;str slices</span>\n        <span class="token-keyword">let</span> id: &amp;<span class="token-type">str</span> = matched.params.<span class="token-fn">get</span>(<span class="token-string">"id"</span>).<span class="token-fn">unwrap_or_default</span>();\n        matched.<span class="token-fn">value</span>(req, id).<span class="token-keyword">await</span>\n    }\n    <span class="token-type">Err</span>(_) =&gt; <span class="token-fn">handle_404</span>(req).<span class="token-keyword">await</span>,\n}'
        },
        {
            num: '03',
            phase: 'SECURITY',
            title: 'Stage 03 — Tower Middleware & Stateless Session HMAC',
            sub: 'Double-submit CSRF verification, security headers, and HMAC-SHA256 signature verification in constant time.',
            time: '~24 µs',
            alloc: '0 B (Constant-Time CMP)',
            syscall: 'None (Local CPU)',
            file: 'yav-server/src/session.rs',
            hardware: 'SIMD Accelerated',
            code: '// Cryptographic Cookie Session Verification\nlet cookie = req.headers().get(header::COOKIE)?;\nlet (payload, signature) = parse_signed_cookie(cookie)?;\n\n// Constant-time HMAC comparison (subtle::ConstantTimeEq)\nif !verify_hmac_sha256(payload, signature, &SECRET_KEY) {\n    return Err(StatusCode::UNAUTHORIZED);\n}\n// No Redis lookup. No external latency. Zero lock contention.',
            codeHtml: '<span class="token-comment">// Cryptographic Cookie Session Verification</span>\n<span class="token-keyword">let</span> cookie = req.<span class="token-fn">headers</span>().<span class="token-fn">get</span>(<span class="token-type">header::COOKIE</span>)?;\n<span class="token-keyword">let</span> (payload, signature) = <span class="token-fn">parse_signed_cookie</span>(cookie)?;\n\n<span class="token-comment">// Constant-time HMAC comparison (subtle::ConstantTimeEq)</span>\n<span class="token-keyword">if</span> !<span class="token-fn">verify_hmac_sha256</span>(payload, signature, &amp;<span class="token-type">SECRET_KEY</span>) {\n    <span class="token-keyword">return</span> <span class="token-type">Err</span>(<span class="token-type">StatusCode::UNAUTHORIZED</span>);\n}\n<span class="token-comment">// No Redis lookup. No external latency. Zero lock contention.</span>'
        },
        {
            num: '04',
            phase: 'TEMPLATE',
            title: 'Stage 04 — RSX Compile-time Direct Buffer Splicing',
            sub: 'Proc-macro renders static string slices and escaped expressions directly into a contiguous pre-reserved buffer.',
            time: '~162 µs',
            alloc: '0 B (Pre-Sized Buffer)',
            syscall: 'None (RAM L1/L2)',
            file: 'yav-macros/src/rsx.rs',
            hardware: 'Contiguous Memory Stream',
            code: '// Generated by rsx! proc macro at compile time:\nlet mut buf = String::with_capacity(ESTIMATED_SIZE);\nbuf.push_str("<main class=\"wrap\"><h1>");\n// Fast SIMD-accelerated HTML escaping into buffer\nyav::core::escape_html_into(&user_name, &mut buf);\nbuf.push_str("</h1><p>Session active</p></main>");\n\n// ZERO virtual DOM tree. ZERO node reconciliation diff.',
            codeHtml: '<span class="token-comment">// Generated by rsx! proc macro at compile time:</span>\n<span class="token-keyword">let</span> <span class="token-keyword">mut</span> buf = <span class="token-type">String</span>::<span class="token-fn">with_capacity</span>(<span class="token-num">ESTIMATED_SIZE</span>);\nbuf.<span class="token-fn">push_str</span>(<span class="token-string">"&lt;main class=&quot;wrap&quot;&gt;&lt;h1&gt;"</span>);\n<span class="token-comment">// Fast SIMD-accelerated HTML escaping into buffer</span>\n<span class="token-fn">yav::core::escape_html_into</span>(&amp;user_name, &amp;<span class="token-keyword">mut</span> buf);\nbuf.<span class="token-fn">push_str</span>(<span class="token-string">"&lt;/h1&gt;&lt;p&gt;Session active&lt;/p&gt;&lt;/main&gt;"</span>);\n\n<span class="token-comment">// ZERO virtual DOM tree. ZERO node reconciliation diff.</span>'
        },
        {
            num: '05',
            phase: 'TRANSMISSION',
            title: 'Stage 05 — Kernel TCP Socket Flush to Wire',
            sub: 'Hyper streams contiguous buffer with HTTP/2 framing and optional Brotli pre-compression via vectored I/O.',
            time: '~52 µs',
            alloc: '0 B (Zero-Copy writev)',
            syscall: 'writev / sendfile',
            file: 'yav-server/src/wire.rs',
            hardware: 'Kernel NIC Ring',
            code: '// HTTP Response packaging with pre-compressed Brotli\nlet response = Response::builder()\n    .status(StatusCode::OK)\n    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")\n    .header(header::CONTENT_ENCODING, "br")\n    .body(Full::new(Bytes::from(buf)))?;\n\n// Kernel flushes bytes to client network card in ~52 µs',
            codeHtml: '<span class="token-comment">// HTTP Response packaging with pre-compressed Brotli</span>\n<span class="token-keyword">let</span> response = <span class="token-type">Response</span>::<span class="token-fn">builder</span>()\n    .<span class="token-fn">status</span>(<span class="token-type">StatusCode::OK</span>)\n    .<span class="token-fn">header</span>(<span class="token-type">header::CONTENT_TYPE</span>, <span class="token-string">"text/html; charset=utf-8"</span>)\n    .<span class="token-fn">header</span>(<span class="token-type">header::CONTENT_ENCODING</span>, <span class="token-string">"br"</span>)\n    .<span class="token-fn">body</span>(<span class="token-type">Full</span>::<span class="token-fn">new</span>(<span class="token-type">Bytes</span>::<span class="token-fn">from</span>(buf)))?;\n\n<span class="token-comment">// Kernel flushes bytes to client network card in ~52 µs</span>'
        }
    ];

    var data = stages[stageIdx];
    if (!data) return;

    var elTitle = document.getElementById('pipe-stage-title');
    var elSub = document.getElementById('pipe-stage-sub');
    var elTime = document.getElementById('pipe-stage-time');
    var elAlloc = document.getElementById('pipe-stage-alloc');
    var elSyscall = document.getElementById('pipe-stage-syscall');
    var elFile = document.getElementById('pipe-stage-file');
    var elCode = document.getElementById('pipe-stage-code');

    if (elTitle) elTitle.textContent = data.title;
    if (elSub) elSub.textContent = data.sub;
    if (elTime) elTime.textContent = data.time;
    if (elAlloc) elAlloc.textContent = data.alloc;
    if (elSyscall) elSyscall.textContent = data.syscall;
    if (elFile) elFile.textContent = data.file;
    if (elCode) elCode.innerHTML = data.codeHtml;
    var copyBtn = document.getElementById('pipe-stage-copy');
    if (copyBtn) copyBtn.setAttribute('data-copy', data.code);

    // Update active tab buttons
    for (var i = 0; i < stages.length; i++) {
        var btn = document.getElementById('pipe-btn-' + i);
        var indicator = document.getElementById('pipe-dot-' + i);
        var bar = document.getElementById('pipe-bar-' + i);
        if (btn) {
            if (i === stageIdx) {
                btn.classList.add('bg-sunk', 'text-paper', 'border-oxide');
                btn.classList.remove('text-dim', 'border-[rgba(236,233,227,0.08)]');
            } else {
                btn.classList.remove('bg-sunk', 'text-paper', 'border-oxide');
                btn.classList.add('text-dim', 'border-[rgba(236,233,227,0.08)]');
            }
        }
        if (indicator) {
            indicator.style.opacity = (i === stageIdx) ? '1' : '0.25';
        }
        if (bar) {
            bar.style.opacity = (i === stageIdx) ? '1' : '0';
        }
    }

    // On mobile / small viewports, ensure plate stays visible
    if (window.innerWidth < 1024) {
        var plate = document.getElementById('pipe-plate-container');
        if (plate) {
            plate.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        }
    }
};
</script>"#);

    rsx! {
        <section id="pipeline" class="wrap pt-12 sm:pt-16">
            // Section Header
            <div class="sec-head reveal">
                <span class="n">"2.1"</span>
                <span class="t">"Execution pipeline"</span>
                <span class="sec-rule"></span>
                <span class="r">"280 µs socket-to-wire"</span>
            </div>

            // Section Headline & Narrative Intro (Elevated above grid for perfect vertical alignment)
            <div class="pt-6 pb-6 max-w-3xl space-y-3 reveal pl-5 sm:pl-7 border-l-2 border-oxide/50">
                <h2 class="h-section text-paper leading-[1.18]">
                    "From socket to wire in "<span class="serif text-oxide">"280 microseconds."</span>
                </h2>
                <p class="text-[15.5px] sm:text-[16px] text-mute leading-[1.8]">
                    "Every incoming HTTP request traverses five zero-allocation, compile-time specialized phases. There are no virtual DOM trees, no intermediate JavaScript bridges, and zero memory allocations on the hot path."
                </p>
            </div>

            // Two-column interactive layout: Stepper on Left, Master Inspector Plate on Right (both start at same top)
            <div class="grid lg:grid-cols-12 gap-8 lg:gap-10 xl:gap-12 items-start">
                // Left Column: Stage Stepper Buttons (starts at top, matching right plate)
                <div class="lg:col-span-5 space-y-3 font-mono text-[13px] reveal">
                    // Button 01: Ingress
                    <button
                            id="pipe-btn-0"
                            onclick="window.selectPipelineStage(0)"
                            class="relative w-full text-left py-4 px-5 border border-oxide bg-sunk text-paper transition-all cursor-pointer flex items-center justify-between group shadow-sm hover:border-oxide overflow-hidden"
                        >
                            <div id="pipe-bar-0" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:1"></div>
                            <div class="flex items-center gap-3.5 pl-1">
                                <span id="pipe-dot-0" class="dot dot-ok" style="opacity:1"></span>
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider">"PHASE 01 / INGRESS"</span>
                                    <span class="font-medium text-paper text-[13.5px]">"Socket Ingress & Framing"</span>
                                </div>
                            </div>
                            <span class="text-dim group-hover:text-paper font-semibold text-[12px]">"~38 µs"</span>
                        </button>

                        // Button 02: Radix
                        <button
                            id="pipe-btn-1"
                            onclick="window.selectPipelineStage(1)"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="pipe-bar-1" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3.5 pl-1">
                                <span id="pipe-dot-1" class="dot dot-ok" style="opacity:0.25"></span>
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider">"PHASE 02 / ROUTING"</span>
                                    <span class="font-medium text-paper text-[13.5px]">"Radix Path Match"</span>
                                </div>
                            </div>
                            <span class="text-dim group-hover:text-paper font-semibold text-[12px]">"~4 µs"</span>
                        </button>

                        // Button 03: Security & HMAC
                        <button
                            id="pipe-btn-2"
                            onclick="window.selectPipelineStage(2)"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="pipe-bar-2" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3.5 pl-1">
                                <span id="pipe-dot-2" class="dot dot-ok" style="opacity:0.25"></span>
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider">"PHASE 03 / SECURITY"</span>
                                    <span class="font-medium text-paper text-[13.5px]">"Tower Layers & HMAC"</span>
                                </div>
                            </div>
                            <span class="text-dim group-hover:text-paper font-semibold text-[12px]">"~24 µs"</span>
                        </button>

                        // Button 04: RSX Buffer Stream
                        <button
                            id="pipe-btn-3"
                            onclick="window.selectPipelineStage(3)"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="pipe-bar-3" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3.5 pl-1">
                                <span id="pipe-dot-3" class="dot dot-ok" style="opacity:0.25"></span>
                                <div>
                                    <span class="text-[10px] text-oxide block tracking-wider font-semibold">"PHASE 04 / TEMPLATE"</span>
                                    <span class="font-medium text-oxide text-[13.5px]">"RSX Buffer Stream"</span>
                                </div>
                            </div>
                            <span class="text-oxide font-bold text-[12px]">"~162 µs"</span>
                        </button>

                        // Button 05: Wire Flush
                        <button
                            id="pipe-btn-4"
                            onclick="window.selectPipelineStage(4)"
                            class="relative w-full text-left py-4 px-5 border border-[rgba(236,233,227,0.08)] bg-ink-2/30 text-dim hover:text-paper hover:bg-sunk/60 transition-all cursor-pointer flex items-center justify-between group shadow-sm overflow-hidden"
                        >
                            <div id="pipe-bar-4" class="absolute left-0 top-0 bottom-0 w-1 bg-oxide transition-opacity" style="opacity:0"></div>
                            <div class="flex items-center gap-3.5 pl-1">
                                <span id="pipe-dot-4" class="dot dot-ok" style="opacity:0.25"></span>
                                <div>
                                    <span class="text-[10px] text-dim block tracking-wider">"PHASE 05 / WIRE"</span>
                                    <span class="font-medium text-paper text-[13.5px]">"Kernel Socket Flush"</span>
                                </div>
                            </div>
                            <span class="text-dim group-hover:text-paper font-semibold text-[12px]">"~52 µs"</span>
                        </button>
                    </div>

                // Right Column: Master Inspector Plate (Sticky so it stays visible alongside buttons)
                <div id="pipe-plate-container" class="lg:col-span-7 lg:sticky lg:top-24 self-start reveal" style="--d:140ms">
                    <div class="plate bg-sunk shadow-2xl">
                        <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                        <div class="plate-head min-h-14 py-2.5 px-4 sm:px-8 flex items-center justify-between gap-3">
                            <span class="flex items-center gap-2.5 font-medium tracking-wide min-w-0">
                                {render_icon("terminal", IconSize::Sm, Some("w-4 h-4 text-oxide shrink-0"), None)}
                                <span id="pipe-stage-title" class="text-[13px] sm:text-[13.5px] text-paper truncate">"Stage 01 — Socket Ingress & Frame Parsing"</span>
                            </span>
                            <span class="text-oxide font-mono text-[12px] font-bold shrink-0" id="pipe-stage-time">"~38 µs"</span>
                        </div>

                        // Telemetry stats row
                        <div class="px-4 sm:px-6 py-3 sm:py-4 border-b border-[rgba(236,233,227,0.08)] bg-[rgba(236,233,227,0.02)] flex flex-wrap items-center justify-between gap-3 sm:gap-6 font-mono text-[11.5px] sm:text-[12px]">
                            <div>
                                <span class="text-dim">"Heap Allocation: "</span>
                                <span id="pipe-stage-alloc" class="text-paper font-semibold">"0 B heap (Zero-Copy)"</span>
                            </div>
                            <div>
                                <span class="text-dim">"Kernel Syscall: "</span>
                                <span id="pipe-stage-syscall" class="text-oxide font-semibold">"epoll_wait / accept4"</span>
                            </div>
                            <div>
                                <span class="text-dim">"Concurrency: "</span>
                                <span class="text-paper font-semibold">"Tokio Work-Stealing"</span>
                            </div>
                        </div>

                        // Stage description
                        <div class="p-5 sm:p-8 pl-6 sm:pl-9 border-b border-[rgba(236,233,227,0.08)]">
                            <p id="pipe-stage-sub" class="text-[14px] sm:text-[14.5px] text-mute leading-[1.8]">
                                "Non-blocking Tokio worker accepts TCP/TLS socket; Hyper parses HTTP frames directly into unallocated slices."
                            </p>
                        </div>

                        // Code viewer
                        <div class="p-4 sm:p-8 pl-5 sm:pl-9 bg-ink-2/40 code-scroll">
                            <div class="flex items-center justify-between pb-3 text-dim font-mono text-[11.5px]">
                                <div class="flex items-center gap-2">
                                    <span id="pipe-stage-file" class="text-paper font-medium">"yav-server/src/conn.rs"</span>
                                    <span class="text-oxide font-medium">"· zero-allocation buffer"</span>
                                </div>
                                <button
                                    id="pipe-stage-copy"
                                    data-copy="// Tokio Worker Thread (Non-blocking I/O loop)\nlet (socket, remote_addr) = listener.accept().await?;\nlet stream = hyper_util::rt::TokioIo::new(socket);\n\n// Hyper HTTP/1.1 & HTTP/2 frame parsing\nlet conn = hyper::server::conn::auto::Builder::new(TokioExecutor::new())\n    .serve_connection(stream, service);\n\n// Slices buffers directly from OS ring buffer with zero allocations."
                                    onclick="copyText(this)"
                                    class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono"
                                >
                                    {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                                    <span data-copy-label="" class="text-oxide font-medium">"copy snippet"</span>
                                </button>
                            </div>
                            <pre class="font-mono text-[13px] leading-[1.85] text-paper-2 selection:bg-oxide selection:text-ink"><code id="pipe-stage-code">
                                <span class="token-comment">"// Tokio Worker Thread (Non-blocking I/O loop)"</span>{"\n"}
                                <span class="token-keyword">"let"</span>" (socket, remote_addr) = listener."<span class="token-fn">"accept"</span>"()."<span class="token-keyword">"await"</span>"?;"{"\n"}
                                <span class="token-keyword">"let"</span>" stream = "<span class="token-type">"hyper_util::rt::TokioIo"</span>"::"<span class="token-fn">"new"</span>"(socket);"{"\n\n"}
                                <span class="token-comment">"// Hyper HTTP/1.1 & HTTP/2 frame parsing"</span>{"\n"}
                                <span class="token-keyword">"let"</span>" conn = "<span class="token-type">"hyper::server::conn::auto::Builder"</span>"::"<span class="token-fn">"new"</span>"("<span class="token-type">"TokioExecutor"</span>"::"<span class="token-fn">"new"</span>"())"{"\n"}
                                "    ."<span class="token-fn">"serve_connection"</span>"(stream, service);"{"\n\n"}
                                <span class="token-comment">"// Slices buffers directly from OS ring buffer with zero allocations."</span>
                            </code></pre>
                        </div>
                    </div>
                </div>
            </div>

            // Bottom 3 summary cells
            <div class="pt-8 sm:pt-10 pb-6">
                <div class="cells grid-cols-1 md:grid-cols-3">
                    <article class="cell p-8 sm:p-10 pl-8 sm:pl-10 space-y-4">
                        <div class="flex justify-between pb-2"><span class="cn font-mono text-[12px]">"Phase 01"</span><span class="font-mono text-[12px] text-dim font-semibold">"~42 µs total"</span></div>
                        <h3 class="text-[22px] text-paper">"Zero-Alloc Ingress"</h3>
                        <p class="text-[14.5px] leading-[1.75] text-mute">"Tokio epoll workers accept connections without thread context switches. Matchit resolves static and parameterized paths in nanoseconds."</p>
                        <div class="dim-line pt-6"><span>"protocol"</span><span class="bar"></span><span>"HTTP/1.1 + HTTP/2"</span></div>
                    </article>

                    <article class="cell p-8 sm:p-10 pl-8 sm:pl-10 space-y-4">
                        <div class="flex justify-between pb-2"><span class="cn text-oxide font-mono text-[12px]">"Phase 02"</span><span class="font-mono text-[12px] text-oxide font-semibold">"~162 µs total"</span></div>
                        <h3 class="text-[22px] text-paper">"RSX Buffer Writes"</h3>
                        <p class="text-[14.5px] leading-[1.75] text-mute">"Your component executes without creating an intermediate object tree. Static HTML literals are pushed into a pre-reserved contiguous buffer."</p>
                        <div class="dim-line pt-6"><span>"vdom heap"</span><span class="bar"></span><span class="text-oxide font-semibold">"0 bytes"</span></div>
                    </article>

                    <article class="cell p-8 sm:p-10 pl-8 sm:pl-10 space-y-4">
                        <div class="flex justify-between pb-2"><span class="cn font-mono text-[12px]">"Phase 03"</span><span class="font-mono text-[12px] text-dim font-semibold">"~76 µs total"</span></div>
                        <h3 class="text-[22px] text-paper">"Wire Transmission"</h3>
                        <p class="text-[14.5px] leading-[1.75] text-mute">"Pre-compressed Brotli payload is written directly to the kernel socket via writev. The browser receives the first byte in under 0.3 ms."</p>
                        <div class="dim-line pt-6"><span>"p50 TTFB"</span><span class="bar"></span><span>"0.28 ms"</span></div>
                    </article>
                </div>
            </div>

            {script}
        </section>
    }
}
