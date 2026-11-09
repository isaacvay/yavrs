//! Architectural Principles (Section 02) for Home Page.

use yav::prelude::*;

#[component]
pub fn HomePrinciples() -> Node {
    rsx! {
        <section class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"02"</span>
                <span class="t" data-i18n="bento.badge">"Principles"</span>
                <span class="sec-rule"></span>
                <span class="r">"six decisions"</span>
            </div>
            <div class="grid lg:grid-cols-12 gap-12 pt-6 pb-6">
                <h2 class="lg:col-span-8 h-section text-paper reveal" data-i18n="bento.title">
                    "Everything you need. "<span class="text-dim">"Nothing that ships to the browser by accident."</span>
                </h2>
                <p class="lg:col-span-4 text-mute text-[14.5px] leading-relaxed self-end reveal">
                    "Built from the ground up for teams who care about real latency, memory safety and eliminating frontend toolchain churn."
                </p>
            </div>
            <div class="cells grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
                <article class="cell">
                    <div class="cn pb-8">"2.1"</div>
                    <h3 class="text-[22px] text-paper pb-3" data-i18n="bento.card1.title">"Zero hydration"</h3>
                    <p class="text-[14.5px] leading-relaxed text-mute" data-i18n="bento.card1.desc">
                        "HTML is written straight into a buffer and sent. Interactivity comes from tiny islands, not a 500 KB runtime."
                    </p>
                </article>
                <article class="cell">
                    <div class="cn pb-8">"2.2"</div>
                    <h3 class="text-[22px] text-paper pb-3" data-i18n="bento.card2.title">"Compile-time RSX"</h3>
                    <p class="text-[14.5px] leading-relaxed text-mute" data-i18n="bento.card2.desc">
                        "JSX ergonomics, verified by rustc. Typed props, auto-escaping, no template engine at runtime."
                    </p>
                </article>
                <article class="cell">
                    <div class="cn pb-8">"2.3"</div>
                    <h3 class="text-[22px] text-paper pb-3" data-i18n="bento.card4.title">"File-system routes"</h3>
                    <p class="text-[14.5px] leading-relaxed text-mute" data-i18n="bento.card4.desc">
                        "Drop [id].rs in src/routes. It compiles to an O(k) radix tree with instant matchit lookups."
                    </p>
                </article>
                <article class="cell">
                    <div class="cn pb-8">"2.4"</div>
                    <h3 class="text-[22px] text-paper pb-3" data-i18n="bento.card3.title">"Tailwind, managed"</h3>
                    <p class="text-[14.5px] leading-relaxed text-mute" data-i18n="bento.card3.desc">
                        "The standalone compiler is downloaded, watched and fingerprinted for you. No Node toolchain in sight."
                    </p>
                </article>
                <article class="cell">
                    <div class="cn pb-8">"2.5"</div>
                    <h3 class="text-[22px] text-paper pb-3" data-i18n="bento.card5.title">"Plain Axum underneath"</h3>
                    <p class="text-[14.5px] leading-relaxed text-mute" data-i18n="bento.card5.desc">
                        "Extractors, Tower layers, WebSockets, SSE. Yav never hides the stack — you can always reach it."
                    </p>
                </article>
                <article class="cell">
                    <div class="cn pb-8">"2.6"</div>
                    <h3 class="text-[22px] text-paper pb-3">"Secure defaults"</h3>
                    <p class="text-[14.5px] leading-relaxed text-mute">
                        "CSRF tokens, HMAC-signed sessions and automatic context-aware escaping are on before you write a single line."
                    </p>
                </article>
            </div>
        </section>
    }
}
