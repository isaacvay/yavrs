//! Ecosystem Showcase (Section 03) for Home Page.

use yav::prelude::*;

#[component]
pub fn HomeEcosystem() -> Node {
    rsx! {
        <section class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"03"</span>
                <span class="t" data-i18n="eco.badge">"Ecosystem"</span>
                <span class="sec-rule"></span>
                <span class="r" data-i18n="eco.meta">"built into binary"</span>
            </div>
            <div class="grid lg:grid-cols-12 gap-12 pt-6 pb-6">
                <h2 class="lg:col-span-8 h-section text-paper reveal">
                    <span data-i18n="eco.title.lead">"Batteries included. "</span><span class="serif text-oxide" data-i18n="eco.title.accent">"Zero external dependencies."</span>
                </h2>
                <p class="lg:col-span-4 text-mute text-[14.5px] leading-relaxed self-end reveal" data-i18n="eco.subtitle">
                    "Unlike JavaScript meta-frameworks requiring dozens of disjointed npm packages, Yav provides an integrated suite directly in Rust."
                </p>
            </div>
            <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 pt-4">
                <div class="plate p-6 space-y-4 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="label text-oxide">"yav-ui"</span>
                        <span class="chip text-[11px]" data-i18n="eco.c1.chip">"60+ primitives"</span>
                    </div>
                    <h3 class="text-[20px] text-paper" data-i18n="eco.c1.title">"UI Primitives"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="eco.c1.desc">
                        "Charts, DataGrid, Kanban with LexoRank, FlowCanvas, 8 Themes, 3D Wireframe Mesh, and KPI StatCards baked directly into the compiler."
                    </p>
                    <a href="/components" class="ulink inline-block font-mono text-[12px]" data-i18n="eco.c1.link">"Browse catalog →"</a>
                </div>

                <div class="plate p-6 space-y-4 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="label text-oxide">"yav-icons"</span>
                        <span class="chip text-[11px]" data-i18n="eco.c2.chip">"13,000+ icons"</span>
                    </div>
                    <h3 class="text-[20px] text-paper" data-i18n="eco.c2.title">"Vector Icons"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="eco.c2.desc">
                        "Complete Lucide, MynaUI and Phosphor icon sets embedded natively. Zero runtime network requests."
                    </p>
                    <a href="/components#icons" class="ulink inline-block font-mono text-[12px]" data-i18n="eco.c2.link">"Explore icon sets →"</a>
                </div>

                <div class="plate p-6 space-y-4 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="label text-oxide">"yav-motion"</span>
                        <span class="chip text-[11px]">"< 2 KB WAAPI"</span>
                    </div>
                    <h3 class="text-[20px] text-paper" data-i18n="eco.c3.title">"Spring Physics"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="eco.c3.desc">
                        "Framer-inspired spring dynamics, while_tap & while_hover gestures, and scroll progress bars with zero JS bloat."
                    </p>
                    <a href="/components#motion" class="ulink inline-block font-mono text-[12px]" data-i18n="eco.c3.link">"Test springs →"</a>
                </div>

                <div class="plate p-6 space-y-4 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="flex items-center justify-between">
                        <span class="label text-oxide">"yav-build"</span>
                        <span class="chip text-[11px]">"Brotli + Gzip"</span>
                    </div>
                    <h3 class="text-[20px] text-paper" data-i18n="eco.c4.title">"Asset Pipeline"</h3>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="eco.c4.desc">
                        "Transparent Brotli pre-compression, Blake3 ETags caching and conditional 304 handling out of the box."
                    </p>
                    <a href="/docs#asset-pipeline" class="ulink inline-block font-mono text-[12px]" data-i18n="eco.c4.link">"Build pipeline →"</a>
                </div>
            </div>
        </section>
    }
}
