//! 5.16 Syntax CodeBlock, Typewriter & Stepper Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentCodeBlock() -> Node {
    let script = Node::raw(r#"<script>
window.replayTypewriter = function() {
    var tw = document.getElementById('demo-typewriter-text');
    if (!tw) return;
    var full = tw.getAttribute('data-full-text') || 'pub fn main() {\n    println!("Fast and Elegant");\n}';
    tw.textContent = '';
    var idx = 0;
    var timer = setInterval(function() {
        if (idx < full.length) {
            tw.textContent += full.charAt(idx);
            idx++;
        } else {
            clearInterval(timer);
            tw.innerHTML = '<span class="token-keyword">pub fn </span><span class="token-fn">main</span>() -&gt; <span class="token-type">Result</span>&lt;(), <span class="token-type">YavError</span>&gt; {\n    <span class="token-macro">println!</span>(<span class="token-string">\"Fast and Elegant: 124,500 req/s\"</span>);\n    <span class="token-type">Ok</span>(())\n}';
        }
    }, 22);
};
</script>"#);

    rsx! {
        <section id="codeblock" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.16"</span>
                <span class="t">"CodeBlock & Typewriter"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add code-block stepper"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"Syntax CodeBlocks, Typewriter & Stepper."</h2>
                <button
                    onclick="copyText(this, 'yav add code-block stepper')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add code-block stepper"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Technical documentation primitives: syntax-highlighted code blocks with line numbering and copy action, syntax-preserving typewriter animations, and multi-step progression steppers."
            </p>

            // Live Interactive Preview Plate
            <div class="plate">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-oxide"></span>
                        "LIVE PREVIEW — TECHNICAL PRIMITIVES"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"click typewriter to replay"</span>
                </div>
                <div class="p-6 sm:p-8 space-y-8 bg-ink-2">
                    // Stepper Progress Indicator
                    <div class="space-y-2">
                        <div class="text-[11px] font-mono text-dim uppercase tracking-wider">"Multi-Step Process Indicator (Stepper)"</div>
                        <div class="flex items-center justify-between max-w-xl font-mono text-xs">
                            <div class="flex items-center gap-2 text-ok">
                                <span class="w-6 h-6 rounded-full bg-ok/20 border border-ok flex items-center justify-center font-bold text-[11px]">"✓"</span>
                                <span>"1. Manifest"</span>
                            </div>
                            <div class="h-0.5 flex-1 bg-ok mx-3"></div>
                            <div class="flex items-center gap-2 text-oxide font-bold">
                                <span class="w-6 h-6 rounded-full bg-oxide/20 border border-oxide flex items-center justify-center text-[11px]">"2"</span>
                                <span>"2. Compile"</span>
                            </div>
                            <div class="h-0.5 flex-1 bg-rule mx-3"></div>
                            <div class="flex items-center gap-2 text-dim">
                                <span class="w-6 h-6 rounded-full bg-ink-2 border border-rule flex items-center justify-center text-[11px]">"3"</span>
                                <span>"3. Deploy"</span>
                            </div>
                        </div>
                    </div>

                    // Syntax-Preserving Typewriter
                    <div class="space-y-2">
                        <div class="flex items-center justify-between">
                            <span class="text-[11px] font-mono text-dim uppercase tracking-wider">"TypewriterCode Engine"</span>
                            <button onclick="replayTypewriter()" class="text-xs font-mono text-oxide hover:underline cursor-pointer flex items-center gap-1">
                                {render_icon("refresh-cw", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                                "Replay Animation"
                            </button>
                        </div>
                        <div onclick="replayTypewriter()" class="p-6 rounded-xl border border-rule bg-sunk font-mono text-[13px] leading-[1.8] cursor-pointer hover:border-oxide/60 transition-colors select-none">
                            <pre class="code"><code id="demo-typewriter-text" data-full-text="pub fn main() -> Result<(), YavError> {
    println!(\"Fast and Elegant: 124,500 req/s\");
    Ok(())
}"><span class="token-keyword">"pub fn "</span><span class="token-fn">"main"</span>"() -> "<span class="token-type">"Result"</span>"&lt;(), "<span class="token-type">"YavError"</span>"&gt; {"{"\n    "}<span class="token-macro">"println!"</span>"("<span class="token-string">"\"Fast and Elegant: 124,500 req/s\""</span>");"{"\n    "}<span class="token-type">"Ok"</span>"(())"{"\n"}"}"</code><span class="inline-block w-2 h-4 bg-oxide ml-1 animate-pulse align-middle"></span></pre>
                        </div>
                    </div>
                </div>
            </div>

            // IDE Code Window
            <div class="ide-window">
                <div class="ide-head">
                    <div class="flex items-center h-full">
                        <div class="ide-tab">
                            {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            "src/components/TechPrimitives.rs"
                        </div>
                    </div>
                    <button data-copy="use yav::ui::{CodeBlock, TypewriterCode, Stepper};\n\nrsx! {\n    <TypewriterCode speed_ms={20} skip_on_click=true>\n        <code>&quot;pub fn main() {}&quot;</code>\n    </TypewriterCode>\n}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                        {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                        <span data-copy-label="" class="text-oxide font-medium">"copy rsx"</span>
                    </button>
                </div>
                <div class="flex bg-[#0b0b0d]">
                    <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                        "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                    </div>
                    <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                        <div class="token-comment">"// Technical Primitives in RSX"</div>
                        <div><span class="token-keyword">"use "</span>"yav::ui::{"<span class="token-type">"CodeBlock"</span>", "<span class="token-type">"TypewriterCode"</span>", "<span class="token-type">"Stepper"</span>"};"</div>
                        <div class="pt-1"><span class="token-macro">"rsx!"</span>" {"</div>
                        <div>"    "<span class="token-tag">"<TypewriterCode "</span><span class="token-attr">"speed_ms="</span>"{"<span class="token-num">"18"</span>"} "<span class="token-attr">"skip_on_click="</span><span class="token-bool">"true"</span><span class="token-tag">">"</span></div>
                        <div>"        "<span class="token-tag">"<div>"</span>"\"pub fn main() { println!(\\\"Fast\\\"); }\""<span class="token-tag">"</div>"</span></div>
                        <div>"    "<span class="token-tag">"</TypewriterCode>"</span></div>
                        <div>"}"</div>
                    </div>
                </div>
                <div class="ide-status">
                    <span>"Zero external JS libraries · WAAPI Accelerated"</span>
                    <span class="text-ok">"SSR Compatible"</span>
                </div>
            </div>

            {script}
        </section>
    }
}
