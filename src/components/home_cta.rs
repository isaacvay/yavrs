//! Final Call-To-Action Banner for Home Page.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn HomeCta() -> Node {
    rsx! {
        <section class="wrap pt-12 sm:pt-16 pb-16">
            <div class="sec-head reveal">
                <span class="n">"08"</span>
                <span class="t" data-i18n="cta.badge">"Next steps"</span>
                <span class="sec-rule"></span>
                <span class="r" data-i18n="cta.meta">"production binary"</span>
            </div>
            <div class="reveal grid lg:grid-cols-12 gap-10 items-end pt-6">
                <div class="lg:col-span-8 space-y-4">
                    <h2 class="h-section text-paper">
                        <span data-i18n="cta.title.line1">"Ship the binary. "</span>
                        <br class="hidden sm:inline" />
                        <span class="serif text-oxide" data-i18n="cta.title.line2">"Keep the weekend."</span>
                    </h2>
                    <p class="text-mute text-[15px] sm:text-[16px] max-w-xl leading-relaxed" data-i18n="cta.subtitle">
                        "Join the next generation of full-stack Rust developers. Type safe, memory safe, and blazingly fast."
                    </p>
                </div>
                <div class="lg:col-span-4 flex flex-col sm:flex-row lg:flex-col gap-3 lg:items-stretch">
                    <a id="cta-docs" href="/docs" class="btn btn-solid justify-between">
                        <span data-i18n="cta.docs">"Read the docs"</span>
                        {render_icon("arrow-right", IconSize::Sm, Some("w-4 h-4 ml-2"), None)}
                    </a>
                    <a id="cta-github" href="https://github.com/isaacvay/yav.rs" target="_blank" rel="noreferrer" class="btn btn-line justify-between">
                        <span class="flex items-center gap-2">
                            {render_icon("minimal:github", IconSize::Sm, Some("w-4 h-4"), None)}
                            <span data-i18n="cta.github">"Star on GitHub"</span>
                        </span>
                        <span class="arr">"↗"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}
