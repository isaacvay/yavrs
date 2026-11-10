//! Documentation Section 2: Components & RSX (Syntax, Functional #[component], Children, .yavml ESM files)

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn DocsComponents() -> Node {
    rsx! {
        <div class="space-y-20">
            // =========================================================================
            // 2.1 RSX MACRO & SYNTAX
            // =========================================================================
            <section id="rsx-syntax" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"2.1"</span>
                    <span class="t" data-i18n="docs.comp.2_1_sec">"RSX Macro & Markup Syntax"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"rsx!"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.comp.2_1_title">"JSX ergonomics. Direct buffer streaming."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.comp.2_1_p1">"The "</span>
                    <code class="inline">"rsx!"</code>
                    <span data-i18n="docs.comp.2_1_p2">" procedural macro translates declarative tags directly into zero-allocation buffer append instructions at build time. There is no Virtual DOM diffing, no synthetic event layer, and zero runtime template parsing overhead."</span>
                </p>

                // IDE LOOK WINDOW
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/ProjectCard.rs"
                            </div>
                        </div>
                        <button data-copy="use yav::prelude::*;\n\n#[component]\npub fn ProjectCard(title: &str, stars: u32, is_featured: bool) -> Node {\n    rsx! {\n        <div class=&quot;p-6 border rounded-lg bg-surface&quot;>\n            <h3 class=&quot;text-xl font-bold&quot;>{title}</h3>\n            <p class=&quot;text-sm text-dim&quot;>Stars: {stars}</p>\n        </div>\n    }\n}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"<br/>"13"<br/>"14"<br/>"15"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"use "</span><span class="token-var">"yav::prelude::*;"</span></div>
                            <div class="pt-1"><span class="token-macro">"#[component]"</span></div>
                            <div><span class="token-keyword">"pub fn "</span><span class="token-fn">"ProjectCard"</span><span class="token-punct">"("</span><span class="token-var">"title"</span><span class="token-punct">": &str, "</span><span class="token-var">"stars"</span><span class="token-punct">": u32, "</span><span class="token-var">"is_featured"</span><span class="token-punct">": bool, "</span><span class="token-var">"tags"</span><span class="token-punct">": &[&str]) -> "</span><span class="token-type">"Node"</span>" {"</div>
                            <div>"    "<span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"p-6 border rounded-lg bg-surface\" "</span><span class="token-attr">"class:border-oxide="</span>"{"<span class="token-var">"is_featured"</span>"}"<span class="token-tag">">"</span></div>
                            <div>"            "<span class="token-tag">"<h3 "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-xl font-bold\""</span><span class="token-tag">">"</span>"{"<span class="token-var">"title"</span>"}"<span class="token-tag">"</h3>"</span></div>
                            <div>"            "<span class="token-tag">"<p "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-sm text-dim\""</span><span class="token-tag">">"</span>"Stars: {"<span class="token-var">"stars"</span>"}"<span class="token-tag">"</p>"</span></div>
                            <div class="token-comment pt-1">"            // Native if/else without closure brackets"</div>
                            <div>"            "<span class="token-keyword">"if "</span><span class="token-var">"is_featured "</span>"{"</div>
                            <div>"                "<span class="token-tag">"<span "</span><span class="token-attr">"class="</span><span class="token-string">"\"badge text-ok\""</span><span class="token-tag">">"</span>"Featured"<span class="token-tag">"</span>"</span></div>
                            <div>"            } "<span class="token-keyword">"else "</span>"{"</div>
                            <div>"                "<span class="token-tag">"<span "</span><span class="token-attr">"class="</span><span class="token-string">"\"badge text-dim\""</span><span class="token-tag">">"</span>"Community"<span class="token-tag">"</span>"</span></div>
                            <div>"            }"</div>
                            <div class="token-comment pt-1">"            // Native for loop directly in RSX"</div>
                            <div>"            "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"flex gap-2 mt-3\""</span><span class="token-tag">">"</span></div>
                            <div>"                "<span class="token-keyword">"for "</span><span class="token-var">"tag "</span><span class="token-keyword">"in "</span><span class="token-var">"tags"</span>".iter() {"</div>
                            <div>"                    "<span class="token-tag">"<span "</span><span class="token-attr">"class="</span><span class="token-string">"\"chip text-xs\""</span><span class="token-tag">">"</span>"{*"<span class="token-var">"tag"</span>"}"<span class="token-tag">"</span>"</span></div>
                            <div>"                }"</div>
                            <div>"            "<span class="token-tag">"</div>"</span></div>
                            <div>"        "<span class="token-tag">"</div>"</span></div>
                            <div>"    }"</div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"Rust · UTF-8 · ProjectCard.rs"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="docs.comp.2_1_st_r">"Compiled without runtime overhead"</span></span>
                    </div>
                </div>

                <div class="grid md:grid-cols-3 gap-4 pt-2 font-mono text-[12px] text-mute">
                    <div class="p-4 plate bg-ink-2 space-y-1.5">
                        <span class="text-oxide font-bold" data-i18n="docs.comp.c1_t">"Automatic XSS Escaping"</span>
                        <p class="text-[11.5px] leading-relaxed" data-i18n="docs.comp.c1_d">"Dynamic text and attributes are escaped automatically (&, <, >, \", '). Only explicit Node::raw() bypasses escaping."</p>
                    </div>
                    <div class="p-4 plate bg-ink-2 space-y-1.5">
                        <span class="text-oxide font-bold" data-i18n="docs.comp.c2_t">"No Quotes for Text"</span>
                        <p class="text-[11.5px] leading-relaxed" data-i18n="docs.comp.c2_d">"Just like in JSX, plain textual content does not require quotes: <p>Welcome to Yav</p> is valid RSX."</p>
                    </div>
                    <div class="p-4 plate bg-ink-2 space-y-1.5">
                        <span class="text-oxide font-bold" data-i18n="docs.comp.c3_t">"Fragments & Nesting"</span>
                        <p class="text-[11.5px] leading-relaxed" data-i18n="docs.comp.c3_d">"Group sibling nodes without introducing superfluous wrapper elements via empty fragments: <> ... </>."</p>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 2.2 COMPONENT PATTERNS
            // =========================================================================
            <section id="component-patterns" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"2.2"</span>
                    <span class="t" data-i18n="docs.comp.2_2_sec">"Component Patterns: Functional & Props Structs"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"#[component]"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.comp.2_2_title">"Two idiomatic patterns for maximum ergonomics."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="docs.comp.2_2_lede">
                    "Yav supports both auto-generated multi-argument props for everyday components and explicit typed structs for complex enterprise interfaces."
                </p>

                <div class="grid md:grid-cols-2 gap-6">
                    // IDE LOOK 1: MULTI-ARG AUTO PROPS
                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("file", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="docs.comp.pat_a">"Pattern A: Auto Props"</span>
                            </div>
                            <span class="text-[10px] text-dim font-mono">"UserBadge.rs"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// #[component] generates props automatically"</div>
                            <div><span class="token-macro">"#[component]"</span></div>
                            <div><span class="token-keyword">"pub fn "</span><span class="token-fn">"UserBadge"</span>"("</div>
                            <div>"    name: "<span class="token-type">"String"</span>","</div>
                            <div>"    is_admin: "<span class="token-type">"bool"</span>","</div>
                            <div>") -> "<span class="token-type">"Node"</span>" {"</div>
                            <div>"    "<span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"flex items-center gap-2\""</span><span class="token-tag">">"</span></div>
                            <div>"            "<span class="token-tag">"<span>"</span>"{name}"<span class="token-tag">"</span>"</span></div>
                            <div>"        "<span class="token-tag">"</div>"</span></div>
                            <div>"    }"</div>
                            <div>"}"</div>
                            <div class="pt-2 token-comment">"// Invocation in parent template:"</div>
                            <div><span class="token-tag">"<UserBadge "</span><span class="token-attr">"name="</span><span class="token-string">"\"Alice\" "</span><span class="token-attr">"is_admin="</span><span class="token-bool">"true"</span><span class="token-tag">" />"</span></div>
                        </div>
                    </div>

                    // IDE LOOK 2: EXPLICIT STRUCT
                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("file", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="docs.comp.pat_b">"Pattern B: Typed Struct"</span>
                            </div>
                            <span class="text-[10px] text-dim font-mono">"StatCard.rs"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Explicit struct with Default derive"</div>
                            <div><span class="token-macro">"#[derive(Clone, Default)]"</span></div>
                            <div><span class="token-keyword">"pub struct "</span><span class="token-type">"StatCardProps"</span>" {"</div>
                            <div>"    "<span class="token-keyword">"pub "</span>"title: "<span class="token-type">"String"</span>","</div>
                            <div>"    "<span class="token-keyword">"pub "</span>"value: "<span class="token-type">"u64"</span>","</div>
                            <div>"    "<span class="token-keyword">"pub "</span>"trend: "<span class="token-type">"Option"</span>"&lt;"<span class="token-type">"f32"</span>"&gt;,"</div>
                            <div>"}"</div>
                            <div class="pt-1"><span class="token-macro">"#[component]"</span></div>
                            <div><span class="token-keyword">"pub fn "</span><span class="token-fn">"StatCard"</span>"(props: "<span class="token-type">"StatCardProps"</span>") -> "<span class="token-type">"Node"</span>" {"</div>
                            <div>"    "<span class="token-macro">"rsx!"</span>" { "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"plate p-4\""</span><span class="token-tag">">"</span><span class="token-tag">"<h3>"</span>"{props.title}"<span class="token-tag">"</h3></div>"</span>" }"</div>
                            <div>"}"</div>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 2.3 CHILDREN & NESTING
            // =========================================================================
            <section id="component-children" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"2.3"</span>
                    <span class="t" data-i18n="docs.comp.2_3_sec">"Children & Shell Layouts"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"Vec<Node>"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.comp.2_3_title">"First-class slot composition with Vec<Node>."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.comp.2_3_p1">"Any component accepting a "</span>
                    <code class="inline">"children: Vec<Node>"</code>
                    <span data-i18n="docs.comp.2_3_p2">" property can directly wrap arbitrary nested RSX elements, enabling flexible layout hierarchies."</span>
                </p>

                // IDE LOOK WINDOW
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("layers", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/ModalDialog.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-dim font-mono" data-i18n="docs.comp.slot_badge">"Slot Wrapper"</span>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-macro">"#[component]"</span></div>
                            <div><span class="token-keyword">"pub fn "</span><span class="token-fn">"ModalDialog"</span>"(title: "<span class="token-type">"String"</span>", children: "<span class="token-type">"Vec"</span>"&lt;"<span class="token-type">"Node"</span>"&gt;) -> "<span class="token-type">"Node"</span>" {"</div>
                            <div>"    "<span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"modal-backdrop fixed inset-0 bg-ink/80 flex items-center justify-center\""</span><span class="token-tag">">"</span></div>
                            <div>"            "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"modal-card bg-surface p-6 border rounded-xl max-w-lg w-full\""</span><span class="token-tag">">"</span></div>
                            <div>"                "<span class="token-tag">"<h2 "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-2xl font-bold pb-4 border-b\""</span><span class="token-tag">">"</span>"{title}"<span class="token-tag">"</h2>"</span></div>
                            <div>"                "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"modal-body py-4\""</span><span class="token-tag">">"</span></div>
                            <div>"                    {children}"</div>
                            <div>"                "<span class="token-tag">"</div>"</span></div>
                            <div>"            "<span class="token-tag">"</div>"</span></div>
                            <div>"        "<span class="token-tag">"</div>"</span></div>
                            <div>"    }"</div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.comp.2_3_st_l">"Rust · Vec<Node> children slot"</span>
                        <span data-i18n="docs.comp.zero_ovh">"Zero runtime overhead"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 2.4 FILE-BASED .YAVML COMPONENTS
            // =========================================================================
            <section id="file-components" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"2.4"</span>
                    <span class="t" data-i18n="docs.comp.2_4_sec">"File-Based Components (.yavml / .yav)"</span>
                    <span class="sec-rule"></span>
                    <span class="r" data-i18n="docs.comp.2_4_r">"ESM syntax"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.comp.2_4_title">"Separation of concerns with modern ESM-style markup."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.comp.2_4_p1">"For frontend developers accustomed to JSX/TSX or Vue Single File Components, Yav allows authoring components in dedicated "</span>
                    <code class="inline">".yavml"</code>
                    <span data-i18n="docs.comp.2_4_p2">" files with imports and default function exports."</span>
                </p>

                // IDE LOOK WINDOW
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("file-text", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/ModernCard.yavml"
                            </div>
                        </div>
                        <span class="text-[11px] text-oxide font-mono" data-i18n="docs.comp.2_4_badge">"ESM Syntax"</span>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"<br/>"13"<br/>"14"<br/>"15"<br/>"16"<br/>"17"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"import"</span>" { "<span class="token-type">"Badge"</span>", "<span class="token-type">"Button"</span>" } "<span class="token-keyword">"from"</span>" "<span class="token-string">"\"yav/ui\""</span>";"</div>
                            <div class="pt-1"><span class="token-keyword">"export default function "</span><span class="token-fn">"ModernCard"</span>"("</div>
                            <div>"    title: "<span class="token-type">"String"</span>","</div>
                            <div>"    subtitle: "<span class="token-type">"String"</span>" = "<span class="token-string">"\"Default subtitle\""</span>".to_string(),"</div>
                            <div>"    active: "<span class="token-type">"bool"</span>" = "<span class="token-bool">"true"</span>","</div>
                            <div>") {"</div>
                            <div>"    "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"p-6 rounded-xl border bg-slate-900 text-white\""</span><span class="token-tag">">"</span></div>
                            <div>"        "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"flex items-center justify-between\""</span><span class="token-tag">">"</span></div>
                            <div>"            "<span class="token-tag">"<h3 "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-xl font-bold\""</span><span class="token-tag">">"</span>"{title}"<span class="token-tag">"</h3>"</span></div>
                            <div>"            {if active {"</div>
                            <div>"                rsx! { "<span class="token-tag">"<Badge "</span><span class="token-attr">"variant="</span><span class="token-string">"\"success\" "</span><span class="token-attr">"label="</span><span class="token-string">"\"Active\" "</span><span class="token-attr">"pill="</span><span class="token-bool">"true"</span><span class="token-tag">" />"</span>" }"</div>
                            <div>"            } else {"</div>
                            <div>"                rsx! { "<span class="token-tag">"<Badge "</span><span class="token-attr">"variant="</span><span class="token-string">"\"secondary\" "</span><span class="token-attr">"label="</span><span class="token-string">"\"Archived\" "</span><span class="token-attr">"pill="</span><span class="token-bool">"true"</span><span class="token-tag">" />"</span>" }"</div>
                            <div>"            }}"</div>
                            <div>"        "<span class="token-tag">"</div>"</span></div>
                            <div>"        "<span class="token-tag">"<p "</span><span class="token-attr">"class="</span><span class="token-string">"\"mt-2 text-slate-400\""</span><span class="token-tag">">"</span>"{subtitle}"<span class="token-tag">"</p>"</span></div>
                            <div>"    "<span class="token-tag">"</div>"</span></div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.comp.2_4_st_l">".yavml ESM component · VS Code Extension supported"</span>
                        <span data-i18n="docs.comp.2_4_st_r">"Parsed by yav-build"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 2.5 STATIC VDOM FOLDING & ZERO ALLOCATION
            // =========================================================================
            <section id="static-folding" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"2.5"</span>
                    <span class="t" data-i18n="docs.comp.2_5_sec">"Static Subtree Folding & Zero-Allocation Buffers"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"Node::Static"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.comp.2_5_title">"Compile-time immutable subtree folding."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.comp.2_5_p1">"Unlike typical SSR engines that recursively allocate Node trees on every single HTTP request, the "</span>
                    <code class="inline">"rsx!"</code>
                    <span data-i18n="docs.comp.2_5_p2">" proc-macro identifies immutable subtrees at compile-time and folds them into "</span>
                    <code class="inline">"Node::Static(&'static str)"</code>
                    <span data-i18n="docs.comp.2_5_p3">". These branches bypass attribute serialization and write directly to contiguous buffer pools."</span>
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("cpu", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "crates/yav-core/src/node.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono" data-i18n="docs.comp.2_5_badge">"Zero Allocations"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// Core zero-allocation enum with Cow<'static, str> slices"</div>
                        <div><span class="token-keyword">"pub enum "</span><span class="token-type">"Node"</span>" {"</div>
                        <div>"    "<span class="token-fn">"Element"</span>"(Element),"</div>
                        <div>"    "<span class="token-fn">"Text"</span>"(Cow&lt;'static, str&gt;),"</div>
                        <div>"    "<span class="token-fn">"Static"</span>"(&'static str), "<span class="token-comment">"// Folded at compile-time, zero heap allocation"</span></div>
                        <div>"    "<span class="token-fn">"Raw"</span>"(String),"</div>
                        <div>"    "<span class="token-fn">"Fragment"</span>"(Vec&lt;Node&gt;),"</div>
                        <div>"}"</div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.comp.2_5_st_l">"Zero heap overhead for static markup"</span>
                        <span class="text-ok" data-i18n="docs.comp.2_5_st_r">"97% Bare-Metal Throughput"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 2.6 NATIVE RSX CONTROL FLOW (IF / ELSE & FOR LOOPS)
            // =========================================================================
            <section id="native-control-flow" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"2.6"</span>
                    <span class="t" data-i18n="docs.comp.2_6_sec">"Native Control Flow in RSX"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"if / else & for"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.comp.2_6_title">"Write standard Rust loops directly inside RSX."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.comp.2_6_p1">"In Yav v0.1.5, you write standard Rust control flow directly inside "</span>
                    <code class="inline">"rsx!"</code>
                    <span data-i18n="docs.comp.2_6_p2">" without needing closure wrappers or "</span>
                    <code class="inline">".iter().map(...)"</code>
                    <span data-i18n="docs.comp.2_6_p3">" boilerplate. Short boolean attributes and class directives ("</span>
                    <code class="inline">"class:active={is_active}"</code>
                    <span data-i18n="docs.comp.2_6_p4">") are evaluated at compile time."</span>
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/UserList.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono" data-i18n="docs.comp.2_6_badge">"Zero Closure Overhead"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// Direct Rust control flow inside rsx! without closure hacks"</div>
                        <div><span class="token-macro">"rsx!"</span>" {"</div>
                        <div>"    &lt;"<span class="token-tag">"div "</span><span class="token-attr">"class="</span><span class="token-string">"\"user-roster\""</span>"&gt;"</div>
                        <div>"        if is_admin {"</div>
                        <div>"            &lt;"<span class="token-tag">"span "</span><span class="token-attr">"class="</span><span class="token-string">"\"badge admin\""</span>"&gt;\"Admin Access\"&lt;/"<span class="token-tag">"span"</span>"&gt;"</div>
                        <div>"        } else {"</div>
                        <div>"            &lt;"<span class="token-tag">"span "</span><span class="token-attr">"class="</span><span class="token-string">"\"badge member\""</span>"&gt;\"Member Access\"&lt;/"<span class="token-tag">"span"</span>"&gt;"</div>
                        <div>"        }"</div>
                        <div>"        &lt;"<span class="token-tag">"ul "</span><span class="token-attr">"class="</span><span class="token-string">"\"team-list\""</span>"&gt;"</div>
                        <div>"            for member in team.iter() {"</div>
                        <div>"                &lt;"<span class="token-tag">"li "</span><span class="token-attr">"class:leader="</span>"{member.is_lead}&gt;{*member.name}&lt;/"<span class="token-tag">"li"</span>"&gt;"</div>
                        <div>"            }"</div>
                        <div>"        &lt;/"<span class="token-tag">"ul"</span>"&gt;"</div>
                        <div>"    &lt;/"<span class="token-tag">"div"</span>"&gt;"</div>
                        <div>"}"</div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.comp.2_6_st_l">"Transforms into straight buffer appends during compilation"</span>
                        <span class="text-ok" data-i18n="docs.comp.2_6_st_r">"No closure allocation"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
