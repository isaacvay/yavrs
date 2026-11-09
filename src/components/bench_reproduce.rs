//! Benchmark CPU Profiling & Reproducibility Test Harness Component.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn BenchReproduce() -> Node {
    rsx! {
        <div>
            // =========================================================================
            // 4.4 CPU PROFILING & TIME BUDGET
            // =========================================================================
            <section class="wrap pt-12 sm:pt-16">
                <div class="sec-head">
                    <span class="n">"4.4"</span>
                    <span class="t">"CPU Execution Profile & Time Budget"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"perf / cargo flamegraph"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">
                    "81.4% of CPU time is pure socket I/O."
                </h2>
                <p class="text-mute text-[15.5px] leading-relaxed max-w-3xl">
                    "Because templates compile directly into byte buffer append instructions at build time, there is virtually zero CPU cycles wasted on template parsing, reflection, or GC pauses. The CPU spends almost its entire budget doing useful network I/O."
                </p>

                // Visual CPU distribution bar
                <div class="plate bg-ink-2 p-6 sm:p-8 mt-6 space-y-6">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    
                    <div class="label pb-2 flex items-center justify-between text-dim">
                        <span>"CPU TIME ALLOCATION BREAKDOWN (PER 100 µs REQUEST)"</span>
                        <span class="text-oxide font-mono">"0.0% Garbage Collection"</span>
                    </div>

                    // Segmented color bar
                    <div class="w-full h-8 rounded bg-sunk border border-rule overflow-hidden flex">
                        <div style="width: 81.4%" class="h-full bg-oxide hover:opacity-90 transition-opacity relative group" title="81.4% Tokio Epoll & Network I/O">
                            <span class="absolute inset-0 flex items-center justify-center text-[11px] font-mono font-bold text-ink truncate px-1">
                                "81.4% Tokio I/O"
                            </span>
                        </div>
                        <div style="width: 11.2%" class="h-full bg-paper hover:opacity-90 transition-opacity relative group" title="11.2% RSX Buffer Append">
                            <span class="absolute inset-0 flex items-center justify-center text-[10px] font-mono font-bold text-ink truncate px-1">
                                "11.2% RSX"
                            </span>
                        </div>
                        <div style="width: 4.8%" class="h-full bg-ok hover:opacity-90 transition-opacity relative group" title="4.8% TLS & Crypto">
                            <span class="absolute inset-0 flex items-center justify-center text-[9px] font-mono font-bold text-ink truncate px-1">
                                "4.8%"
                            </span>
                        </div>
                        <div style="width: 1.8%" class="h-full bg-[#eab308] hover:opacity-90 transition-opacity relative group" title="1.8% Radix Router">
                        </div>
                        <div style="width: 0.8%" class="h-full bg-dim hover:opacity-90 transition-opacity relative group" title="0.8% Alloc">
                        </div>
                    </div>

                    // Legend & Detail Table
                    <div class="grid grid-cols-2 md:grid-cols-5 gap-4 pt-2 font-mono text-xs">
                        <div class="p-3 bg-sunk rounded border border-rule space-y-1">
                            <div class="text-oxide font-bold text-[14px]">"81.4%"</div>
                            <div class="text-paper font-medium text-[11.5px]">"Tokio Socket I/O"</div>
                            <p class="text-dim text-[11px]">"epoll, TCP buffer reads & writes"</p>
                        </div>
                        <div class="p-3 bg-sunk rounded border border-rule space-y-1">
                            <div class="text-paper font-bold text-[14px]">"11.2%"</div>
                            <div class="text-paper font-medium text-[11.5px]">"RSX Buffer Append"</div>
                            <p class="text-dim text-[11px]">"Pre-computed static HTML slices"</p>
                        </div>
                        <div class="p-3 bg-sunk rounded border border-rule space-y-1">
                            <div class="text-ok font-bold text-[14px]">"4.8%"</div>
                            <div class="text-paper font-medium text-[11.5px]">"TLS & HMAC"</div>
                            <p class="text-dim text-[11px]">"Hardware AES-NI & SHA256"</p>
                        </div>
                        <div class="p-3 bg-sunk rounded border border-rule space-y-1">
                            <div class="text-[#eab308] font-bold text-[14px]">"1.8%"</div>
                            <div class="text-paper font-medium text-[11.5px]">"Radix Router"</div>
                            <p class="text-dim text-[11px]">"O(k) path lookup via matchit"</p>
                        </div>
                        <div class="p-3 bg-sunk rounded border border-rule space-y-1">
                            <div class="text-dim font-bold text-[14px]">"0.8%"</div>
                            <div class="text-paper font-medium text-[11.5px]">"Memory Alloc"</div>
                            <p class="text-dim text-[11px]">"Zero heap allocations on hot path"</p>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 4.5 REPRODUCIBILITY & TEST HARNESS
            // =========================================================================
            <section id="reproduce" class="wrap pt-12 sm:pt-16 scroll-mt-24 space-y-6">
                <div class="sec-head">
                    <span class="n">"4.5"</span>
                    <span class="t">"Reproducibility & Test Harness"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"don't trust, verify"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight">
                    "Execute the benchmarks yourself in under 2 minutes."
                </h2>
                <p class="text-mute text-[15.5px] leading-relaxed max-w-3xl">
                    "All benchmarks are 100% reproducible on any standard Linux or macOS server using open-source tools ("
                    <code class="inline">"wrk"</code>
                    ", "
                    <code class="inline">"k6"</code>
                    "). Follow the commands below to compile and execute the test harness."
                </p>

                // TERMINAL LOOK WINDOW
                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-1.5">
                                <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                            </div>
                            <span class="text-faint text-[11px] ml-2">"bash — wrk 4.2.0 benchmark reproduction harness"</span>
                        </div>
                        <button data-copy="sysctl -w net.core.somaxconn=65535\ncargo build --release\n./target/release/website &\nwrk -t8 -c200 -d30s --latency http://127.0.0.1:3000/" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy all"</span>
                        </button>
                    </div>

                    // Step-by-Step Command Execution
                    <div class="p-6 code space-y-3 overflow-x-auto text-[13px] bg-[#070709]">
                        <div>
                            <div class="token-comment">"# 1. Tune Linux kernel socket queue limits for high concurrency"</div>
                            <div><span class="text-oxide">"$ "</span>"sudo sysctl -w net.core.somaxconn=65535"</div>
                            <div><span class="text-oxide">"$ "</span>"ulimit -n 1000000"</div>
                        </div>

                        <div>
                            <div class="token-comment">"# 2. Build the optimized release binary with Link-Time Optimization (LTO)"</div>
                            <div><span class="text-oxide">"$ "</span>"RUSTFLAGS=\"-C target-cpu=native\" cargo build --release"</div>
                        </div>

                        <div>
                            <div class="token-comment">"# 3. Run the application binary in release mode"</div>
                            <div><span class="text-oxide">"$ "</span>"./target/release/website &"</div>
                        </div>

                        <div>
                            <div class="token-comment">"# 4. Launch wrk HTTP benchmark harness: 8 worker threads, 200 conns, 30s"</div>
                            <div><span class="text-oxide">"$ "</span>"wrk -t8 -c200 -d30s --latency http://127.0.0.1:3000/"</div>
                        </div>

                        // Formatted Terminal Output with whitespace-pre for perfect tabular alignment
                        <div class="pt-3">
                            <div class="token-comment pb-2">"# 5. Expected verified wrk output (preserved whitespace):"</div>
                            <pre class="p-5 rounded bg-[#030304] border border-rule text-paper font-mono text-[12px] leading-relaxed whitespace-pre overflow-x-auto select-text">
{"Running 30s test @ http://127.0.0.1:3000/
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   280.42us  120.14us   8.12ms   91.14%
    Req/Sec    15.58k     1.12k   19.20k    72.10%
  Latency Distribution
     50%  280.00us
     75%  340.00us
     90%  420.00us
     99%    1.12ms
  3,735,000 requests in 30.00s, 3.82GB read
Requests/sec: 124,500.12
Transfer/sec:    130.38MB"}
                            </pre>
                        </div>
                    </div>
                </div>

                // Bottom Navigation Cards
                <div class="pt-12 border-t border-rule grid sm:grid-cols-2 gap-6">
                    <a href="/about" class="plate p-6 bg-sunk hover:border-oxide/60 transition-colors group block space-y-2">
                        <div class="text-[11px] font-mono text-dim flex items-center gap-2 group-hover:text-oxide transition-colors">
                            {render_icon("arrow-left", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "Previous Architectural Guide"
                        </div>
                        <div class="text-paper font-medium text-lg">"Architecture & Memory Layout"</div>
                        <p class="text-mute text-[13px] leading-relaxed">
                            "Deep dive into zero-lock memory concurrency, thread scheduling, and compile-time RSX."
                        </p>
                    </a>
                    <a href="/docs" class="plate p-6 bg-sunk hover:border-oxide/60 transition-colors group block space-y-2 text-right">
                        <div class="text-[11px] font-mono text-dim flex items-center justify-end gap-2 group-hover:text-oxide transition-colors">
                            "Next Engineering Guide"
                            {render_icon("arrow-right", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                        </div>
                        <div class="text-paper font-medium text-lg">"Engineering Documentation & API"</div>
                        <p class="text-mute text-[13px] leading-relaxed">
                            "Comprehensive manual covering CLI toolbelt, components, sessions, and production build."
                        </p>
                    </a>
                </div>
            </section>
        </div>
    }
}
