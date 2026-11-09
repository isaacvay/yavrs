//! Memory Model & Compile-time Specialization Component (Airy & Generous).

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ArchMemory() -> Node {
    rsx! {
        <section id="memory" class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"2.2"</span>
                <span class="t">"Memory Architecture"</span>
                <span class="sec-rule"></span>
                <span class="r">"zero VDOM heap overhead"</span>
            </div>

            <div class="pt-6 grid lg:grid-cols-12 gap-10 lg:gap-14 items-start pb-6">
                <div class="lg:col-span-5 space-y-8 reveal pl-2 sm:pl-4">
                    <div class="space-y-4 pl-5 sm:pl-7 border-l-2 border-oxide/50">
                        <h2 class="h-section text-paper leading-[1.2]">
                            "Compile-time layout vs "<span class="serif text-oxide">"runtime VDOM."</span>
                        </h2>
                        <p class="text-[15.5px] text-mute leading-[1.8]">
                            "Traditional web frameworks allocate a dynamic Virtual DOM tree on every single render. Hundreds of micro-objects are scattered across the heap, followed by expensive tree-diffing algorithms and periodic garbage collector pauses."
                        </p>
                    </div>

                    <p class="text-[15.5px] text-mute leading-[1.8] pl-5 sm:pl-7">
                        "Yav moves this entire topology analysis to <strong class=\"text-paper font-semibold\">compile time</strong>. The Rust compiler embeds all static HTML fragments directly into the read-only data segment (<code class=\"text-oxide font-mono\">.rodata</code>) of the binary. At runtime, dynamic components merely append slices into a single contiguous buffer."
                    </p>

                    <div class="p-6 sm:p-7 border border-[rgba(236,233,227,0.08)] bg-sunk space-y-3.5 font-mono text-[12.5px] border-l-2 border-l-oxide pl-6 sm:pl-8">
                        <div class="flex items-center gap-2.5 text-oxide font-semibold text-[13px]">
                            {render_icon("zap", IconSize::Sm, Some("w-4 h-4"), None)}
                            <span>"Why Contiguous Memory Wins"</span>
                        </div>
                        <p class="text-dim leading-[1.75]">
                            "A single pre-sized buffer stays inside L1/L2 CPU cache lines (64 bytes). Pointer-chasing tree structures trigger constant RAM cache misses."
                        </p>
                    </div>
                </div>

                // Comparative Memory Architecture Blueprint with open spacing
                <div class="lg:col-span-7 reveal" style="--d:140ms">
                    <div class="plate bg-sunk shadow-2xl">
                        <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                        <div class="plate-head h-12 px-6">
                            <span class="tracking-wider text-[12px] font-medium">"MEMORY TOPOLOGY COMPARISON"</span>
                            <span class="text-dim font-mono text-[11px]">"Heap Allocation Analysis"</span>
                        </div>

                        <div class="p-7 sm:p-9 pl-8 sm:pl-10 space-y-8">
                            // Traditional Frameworks
                            <div class="space-y-3">
                                <div class="flex justify-between font-mono text-[13px]">
                                    <span class="text-dim font-semibold">"Traditional VDOM (Next.js / Nuxt / Leptos WASM)"</span>
                                    <span class="text-[#ff7878] font-medium">"~350–500 KB heap allocations / req"</span>
                                </div>
                                <div class="p-4 border border-[rgba(255,120,120,0.2)] bg-[rgba(255,120,120,0.03)] rounded-xs space-y-3 font-mono text-[11.5px]">
                                    <div class="flex items-center justify-between text-dim">
                                        <span>"Heap: [VNode*] -> [Props HashMap] -> [Children Vec] -> [Diff Tree]"</span>
                                        <span class="text-[#ff7878]">"Cache Misses"</span>
                                    </div>
                                    <div class="w-full bg-[rgba(255,120,120,0.15)] h-2.5 rounded-xs overflow-hidden">
                                        <div class="bg-[#ff5555] h-full w-[88%]"></div>
                                    </div>
                                </div>
                            </div>

                            // Yav Compiled Model
                            <div class="space-y-3">
                                <div class="flex justify-between font-mono text-[13px]">
                                    <span class="text-paper font-semibold flex items-center gap-2.5">
                                        <span class="dot dot-ok pulse-dot"></span>
                                        "Yav Proc-Macro Compiled Slices"
                                    </span>
                                    <span class="text-oxide font-bold">"0 dynamic allocations (L1 cache)"</span>
                                </div>
                                <div class="p-4 border border-[rgba(224,89,47,0.3)] bg-[rgba(224,89,47,0.05)] rounded-xs space-y-3 font-mono text-[11.5px]">
                                    <div class="flex items-center justify-between text-dim">
                                        <span>".rodata: [&'static str (headers)] + [&'static str (footers)] -> BytesMut"</span>
                                        <span class="text-oxide font-medium">"Zero-Copy"</span>
                                    </div>
                                    <div class="w-full bg-[rgba(236,233,227,0.06)] h-2.5 rounded-xs overflow-hidden">
                                        <div class="bg-oxide h-full w-[4%]"></div>
                                    </div>
                                </div>
                            </div>

                            // Code transformation explanation
                            <div class="pt-4 border-t border-[rgba(236,233,227,0.08)]">
                                <div class="text-dim font-mono text-[11.5px] pb-3 uppercase tracking-wider flex items-center justify-between">
                                    <span>"MACRO EXPANSION UNDER THE HOOD"</span>
                                    <button
                                        data-copy="// 1. What you write in RSX:\nrsx! { <div class=&quot;card&quot;><h2>{title}</h2></div> }\n\n// 2. What the compiler emits into machine code:\nbuf.push_str(&quot;<static slice at 0x401020>&quot;);  // zero alloc\nyav::core::escape_html_into(&title, &mut buf); // SIMD escape\nbuf.push_str(&quot;<static slice at 0x401048>&quot;);  // zero alloc"
                                        onclick="copyText(this)"
                                        class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono"
                                    >
                                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                                        <span data-copy-label="" class="text-oxide font-medium">"copy snippet"</span>
                                    </button>
                                </div>
                                <pre class="p-5 sm:p-6 bg-ink-2/60 border border-[rgba(236,233,227,0.08)] font-mono text-[12.5px] leading-[1.8] text-paper-2 overflow-x-auto"><code><span class="token-comment">"// 1. What you write in RSX:"</span>{"\n"}
<span class="token-macro">"rsx!"</span>" { "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"card\""</span><span class="token-tag">"><h2>"</span>"{title}"<span class="token-tag">"</h2></div>"</span>" }"{"\n\n"}
<span class="token-comment">"// 2. What the compiler emits into machine code:"</span>{"\n"}
<span class="token-type">"buf"</span>".push_str("<span class="text-dim">"&lt;static slice at 0x401020&gt;"</span>");  "<span class="token-comment">"// zero alloc"</span>{"\n"}
<span class="token-fn">"yav::core::escape_html_into"</span>"(&title, &mut buf); "<span class="token-comment">"// SIMD escape"</span>{"\n"}
<span class="token-type">"buf"</span>".push_str("<span class="text-dim">"&lt;static slice at 0x401048&gt;"</span>");  "<span class="token-comment">"// zero alloc"</span></code></pre>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
