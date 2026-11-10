//! Getting Started & yav-cli Command Suite for Home Page.

use yav::prelude::*;

#[component]
pub fn HomeToolchain() -> Node {
    rsx! {
        // =====================================================
        // 06 — START IN 60 SECONDS
        // =====================================================
        <section class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"06"</span>
                <span class="t" data-i18n="steps.badge">"Start"</span>
                <span class="sec-rule"></span>
                <span class="r" data-i18n="steps.meta">"≈ 60 seconds"</span>
            </div>
            <ol class="pt-2">
                <li class="group grid grid-cols-12 gap-6 items-center py-5 border-b border-[rgba(236,233,227,0.08)]">
                    <span class="col-span-2 md:col-span-1 font-mono text-[13px] text-dim group-hover:text-oxide transition-colors">"6.1"</span>
                    <div class="col-span-10 md:col-span-4">
                        <span class="text-[20px] tracking-[-0.03em] text-paper block" data-i18n="steps.step1.title">"Install the CLI"</span>
                        <span class="text-[12px] text-dim block pt-0.5" data-i18n="steps.step1.desc">"Installs the standalone yav development toolchain."</span>
                    </div>
                    <button id="step-copy-1" onclick="copyText(this, 'cargo install yav-cli')" class="col-span-12 md:col-span-7 flex items-center justify-between gap-4 h-11 px-4 bg-sunk border border-[rgba(236,233,227,0.08)] hover:border-[rgba(236,233,227,0.2)] font-mono text-[13px] text-paper text-left transition-colors cursor-pointer">
                        <span><span class="text-oxide">"$ "</span>"cargo install yav-cli"</span>
                        <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium text-[11px]">"copy"</span>
                    </button>
                </li>
                <li class="group grid grid-cols-12 gap-6 items-center py-5 border-b border-[rgba(236,233,227,0.08)]">
                    <span class="col-span-2 md:col-span-1 font-mono text-[13px] text-dim group-hover:text-oxide transition-colors">"6.2"</span>
                    <div class="col-span-10 md:col-span-4">
                        <span class="text-[20px] tracking-[-0.03em] text-paper block" data-i18n="steps.step2.title">"Scaffold a project"</span>
                        <span class="text-[12px] text-dim block pt-0.5" data-i18n="steps.step2.desc">"Generates all layouts, routes, styles, and manifest."</span>
                    </div>
                    <button id="step-copy-2" onclick="copyText(this, 'yav new my-app')" class="col-span-12 md:col-span-7 flex items-center justify-between gap-4 h-11 px-4 bg-sunk border border-[rgba(236,233,227,0.08)] hover:border-[rgba(236,233,227,0.2)] font-mono text-[13px] text-paper text-left transition-colors cursor-pointer">
                        <span><span class="text-oxide">"$ "</span>"yav new my-app"</span>
                        <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium text-[11px]">"copy"</span>
                    </button>
                </li>
                <li class="group grid grid-cols-12 gap-6 items-center py-5 border-b border-[rgba(236,233,227,0.08)]">
                    <span class="col-span-2 md:col-span-1 font-mono text-[13px] text-dim group-hover:text-oxide transition-colors">"6.3"</span>
                    <div class="col-span-10 md:col-span-4">
                        <span class="text-[20px] tracking-[-0.03em] text-paper block" data-i18n="steps.step3.title">"Run dev server"</span>
                        <span class="text-[12px] text-dim block pt-0.5" data-i18n="steps.step3.desc">"Starts live reload and Tailwind CSS compiler."</span>
                    </div>
                    <button id="step-copy-3" onclick="copyText(this, 'cd my-app && yav dev')" class="col-span-12 md:col-span-7 flex items-center justify-between gap-4 h-11 px-4 bg-sunk border border-[rgba(236,233,227,0.08)] hover:border-[rgba(236,233,227,0.2)] font-mono text-[13px] text-paper text-left transition-colors cursor-pointer">
                        <span><span class="text-oxide">"$ "</span>"cd my-app && yav dev"</span>
                        <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium text-[11px]">"copy"</span>
                    </button>
                </li>
            </ol>
        </section>

        // =====================================================
        // 07 — CLI TOOLCHAIN & DEVELOPER EXPERIENCE
        // =====================================================
        <section class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"07"</span>
                <span class="t" data-i18n="cli.badge">"Toolchain"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav CLI"</span>
            </div>
            <div class="grid lg:grid-cols-12 gap-12 pt-6 pb-6">
                <h2 class="lg:col-span-8 h-section text-paper reveal">
                    <span data-i18n="cli.title.lead">"One CLI for development, "</span><span class="text-dim" data-i18n="cli.title.dim">"production & deployment."</span>
                </h2>
                <p class="lg:col-span-4 text-mute text-[14.5px] leading-relaxed self-end reveal" data-i18n="cli.subtitle">
                    "Built in Rust with zero runtime dependencies. Replaces npm, webpack, vite, and cargo-watch in a single binary."
                </p>
            </div>
            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6 pt-4">
                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav dev"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p1.title">"Live Reload & HMR"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p1.desc">
                        "Auto-reloads browser via WebSocket :35729, watches Tailwind CSS inputs, and displays live compile-error overlay."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav doctor"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p2.title">"System & Toolchain Audit"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p2.desc">
                        "Diagnoses Rust toolchains, package.yavml health, standalone Tailwind cache, routes directory, and port bindings."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav routes"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p3.title">"Route Table Inspection"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p3.desc">
                        "Inspects file-based route endpoints, HTTP methods, dynamic slugs, and active layout hierarchy in a terminal table."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav export"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p4.title">"Static Site Generation (SSG)"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p4.desc">
                        "Pre-renders all static pages to HTML in dist/ with asset bundling and canonical tags for zero-server hosting."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav build"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p5.title">"Production Packaging"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p5.desc">
                        "Compiles release binary, tree-shakes styles, hashes asset filenames, and pre-compresses Brotli (.br) & Gzip (.gz)."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav update"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p6.title">"CLI & Deps Upgrades"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p6.desc">
                        "Updates the yav CLI binary itself and upgrades project dependencies and cached Tailwind binaries seamlessly."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav add <component>"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p7.title">"Component Scaffolding"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p7.desc">
                        "Instantly scaffolds UI templates: drawers, command palettes, 8 design themes, 3D tilt plates, and KPI stat cards."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav codegen"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p8.title">"Types & OpenAPI 3.1"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p8.desc">
                        "Exports TypeScript type definitions and OpenAPI 3.1 specifications directly from Rust serde structs."
                    </p>
                </div>

                <div class="plate p-6 space-y-3 bg-ink-2">
                    <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                    <div class="font-mono text-oxide text-[13px] font-bold">"yav deploy"</div>
                    <div class="text-[16px] text-paper font-medium" data-i18n="cli.p9.title">"Multi-Cloud Deploy"</div>
                    <p class="text-[13px] text-mute leading-relaxed" data-i18n="cli.p9.desc">
                        "Generates production deployment configurations for Shuttle.rs, Fly.io, Railway, Docker, and Systemd."
                    </p>
                </div>
            </div>
        </section>
    }
}
