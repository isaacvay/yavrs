//! Documentation Section 1: Getting Started (Installation, Quickstart, Project Anatomy, Manifest, CLI Toolbelt)

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn DocsGettingStarted() -> Node {
    rsx! {
        <div class="space-y-20">
            // =========================================================================
            // 1.1 PREREQUISITES & INSTALLATION
            // =========================================================================
            <section id="installation" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"1.1"</span>
                    <span class="t" data-i18n="docs.gs.1_1_sec">"Prerequisites & Installation"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"crates.io"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.gs.1_1_title">"Zero Node.js dependency. Pure native Rust."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="docs.gs.1_1_lede">
                    "Yav is engineered to free full-stack web engineering from fragile JavaScript toolchains, npm cascades, and heavy hydration runtimes. All you need is standard Rust (1.75+)."
                </p>

                <div class="grid sm:grid-cols-2 gap-4 text-xs font-mono">
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-ok">
                        <div class="text-paper font-semibold flex items-center gap-2">
                            {render_icon("check-circle", IconSize::Sm, Some("w-3.5 h-3.5 text-ok"), None)}
                            <span data-i18n="docs.gs.req_t">"Required Prerequisite"</span>
                        </div>
                        <p class="text-mute text-[11.5px]" data-i18n="docs.gs.req_d">"Rust stable (1.75+) installed via rustup.rs."</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5 border-l-2 border-l-oxide">
                        <div class="text-paper font-semibold flex items-center gap-2">
                            {render_icon("x-circle", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                            <span data-i18n="docs.gs.elim_t">"Eliminated Dependencies"</span>
                        </div>
                        <p class="text-mute text-[11.5px]" data-i18n="docs.gs.elim_d">"No Node.js, no npm, no pnpm, no webpack, no vite."</p>
                    </div>
                </div>

                // TERMINAL LOOK WINDOW
                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-1.5">
                                <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                            </div>
                            <span class="text-faint text-[11px] ml-2">"bash — yav-cli install"</span>
                        </div>
                        <button data-copy="cargo install yav-cli\nyav update" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#070709]">
                        <div class="token-comment">"# 1. Install the standalone CLI binary onto your PATH from crates.io"</div>
                        <div><span class="text-oxide">"$ "</span>"cargo install yav-cli"</div>
                        <div class="token-comment pt-2">"# Or install cutting-edge release directly from GitHub main"</div>
                        <div><span class="text-oxide">"$ "</span>"cargo install --git https://github.com/isaacvay/yav.rs yav-cli"</div>
                        <div class="token-comment pt-2">"# Verify installation and upgrade project toolchain at any time"</div>
                        <div><span class="text-oxide">"$ "</span>"yav update"</div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 1.2 QUICKSTART & DEV SERVER
            // =========================================================================
            <section id="quickstart" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"1.2"</span>
                    <span class="t" data-i18n="docs.gs.1_2_sec">"Quickstart & High-Velocity Dev Server"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"yav dev"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.gs.1_2_title">"From zero to running server in 60 seconds."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="docs.gs.1_2_lede">
                    "The Yav CLI automates everything: project scaffolding, automated standalone Tailwind JIT provisioning, live incremental compilation, and WebSocket hot reloading without touching Node.js."
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
                            <span class="text-faint text-[11px] ml-2">"zsh — yav new & dev"</span>
                        </div>
                        <button data-copy="yav new my-app\ncd my-app\nyav dev" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#070709]">
                        <div class="token-comment">"# 1. Scaffold a fresh Yav project with file routing and Tailwind CSS"</div>
                        <div><span class="text-oxide">"$ "</span>"yav new my-app"</div>
                        <div class="token-comment pt-2">"# 2. Enter workspace directory"</div>
                        <div><span class="text-oxide">"$ "</span>"cd my-app"</div>
                        <div class="token-comment pt-2">"# 3. Start high-velocity dev server with live reload on http://127.0.0.1:3000"</div>
                        <div><span class="text-oxide">"$ "</span>"yav dev"</div>
                        <div class="text-ok text-[12px] pt-1 flex items-center gap-2">
                            <span class="dot dot-ok"></span>
                            <span data-i18n="docs.gs.dev_ok">"✔ Compiled Tailwind CSS · Server running at http://127.0.0.1:3000"</span>
                        </div>
                    </div>
                </div>

                <div class="p-5 plate bg-ink-2 space-y-2 border-l-2 border-l-oxide">
                    <div class="font-mono text-[12.5px] text-paper font-semibold flex items-center gap-2">
                        {render_icon("zap", IconSize::Sm, Some("w-4 h-4 text-oxide"), None)}
                        <span data-i18n="docs.gs.overlay_t">"Glassmorphism Live Compiler Error Overlay"</span>
                    </div>
                    <p class="text-[13.5px] text-mute leading-relaxed" data-i18n="docs.gs.overlay_d">
                        "When a syntax mistake or mismatched Rust type occurs during development, the dev server captures rustc compiler diagnostics and immediately projects an interactive dark-mode error overlay directly onto your browser page, recovering automatically the millisecond you save the fix."
                    </p>
                </div>
            </section>

            // =========================================================================
            // 1.3 PROJECT ANATOMY
            // =========================================================================
            <section id="project-anatomy" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"1.3"</span>
                    <span class="t" data-i18n="docs.gs.1_3_sec">"Project Anatomy & Directory Layout"</span>
                    <span class="sec-rule"></span>
                    <span class="r" data-i18n="docs.gs.1_3_r">"layout"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.gs.1_3_title">"Predictable, modular convention over configuration."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="docs.gs.1_3_lede">
                    "Every Yav project follows a crystal-clear structure designed for scale, separating page handlers, layout wrappers, standalone UI components, and static public assets."
                </p>

                // IDE LOOK WINDOW FOR DIRECTORY STRUCTURE
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("folder", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="docs.gs.tree_tab">"my-app / Workspace Tree"</span>
                            </div>
                        </div>
                        <span class="text-[11px] text-dim font-mono" data-i18n="docs.gs.file_exp">"File Explorer"</span>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"<br/>"13"<br/>"14"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="text-paper font-semibold">"my-app/"</div>
                            <div>"├── package.yavml        "<span class="token-comment">"# Central project descriptor (server, tailwind, pwa, scripts)"</span></div>
                            <div>"├── Cargo.toml           "<span class="token-comment">"# Standard Rust crate manifest & dependencies"</span></div>
                            <div>"├── public/              "<span class="token-comment">"# Static files served with Blake3 ETags & Brotli"</span></div>
                            <div>"│   ├── favicon.svg"</div>
                            <div>"│   └── styles.css       "<span class="token-comment">"# JIT compiled Tailwind output"</span></div>
                            <div>"└── src/"</div>
                            <div>"    ├── main.rs          "<span class="token-comment">"# Application entrypoint and YavApp bootstrap"</span></div>
                            <div>"    ├── routes/          "<span class="token-comment">"# Radix file-system routing directory"</span></div>
                            <div>"    │   ├── mod.rs       "<span class="token-comment">"# auto_routes!(app) declaration"</span></div>
                            <div>"    │   ├── index.rs     "<span class="token-comment">"# Maps to GET /"</span></div>
                            <div>"    │   └── about.rs     "<span class="token-comment">"# Maps to GET /about"</span></div>
                            <div>"    ├── components/      "<span class="token-comment">"# Reusable UI components (RSX & .yavml)"</span></div>
                            <div>"    └── styles/app.css   "<span class="token-comment">"# Tailwind source input"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.gs.tree_st">"Workspace root · 14 files · 0 node_modules"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="docs.gs.clean_tree">"Clean tree"</span></span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 1.4 PACKAGE.YAVML MANIFEST
            // =========================================================================
            <section id="manifest-reference" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"1.4"</span>
                    <span class="t" data-i18n="docs.gs.1_4_sec">"Central Manifest: package.yavml"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"TOML"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.gs.1_4_title">"Single source of truth for the entire application."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <code class="inline">"package.yavml"</code>
                    <span data-i18n="docs.gs.1_4_lede1">" is the definitive project manifest in Yav (comparable to "</span>
                    <code class="inline">"package.json"</code>
                    <span data-i18n="docs.gs.1_4_lede2">" in React or "</span>
                    <code class="inline">"pubspec.yaml"</code>
                    <span data-i18n="docs.gs.1_4_lede3">" in Flutter), configuring server parameters, Tailwind JIT, PWA headers, and deployment assets."</span>
                </p>

                // IDE LOOK WINDOW FOR PACKAGE.YAVML
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("file-text", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "package.yavml"
                            </div>
                        </div>
                        <button data-copy="[package]\nname = &quot;my-app&quot;\nversion = &quot;0.1.0&quot;\n\n[server]\nport = 3000\nhost = &quot;127.0.0.1&quot;\n\n[tailwind]\nenabled = true\nversion = &quot;3.4.17&quot;\n\n[build]\nminify = true\nfingerprint = true" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"<br/>"13"<br/>"14"<br/>"15"<br/>"16"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-macro">"[package]"</span></div>
                            <div><span class="token-attr">"name"</span> <span class="token-punct">"="</span> <span class="token-string">"\"my-app\""</span></div>
                            <div><span class="token-attr">"version"</span> <span class="token-punct">"="</span> <span class="token-string">"\"0.1.0\""</span></div>
                            <div><span class="token-attr">"description"</span> <span class="token-punct">"="</span> <span class="token-string">"\"High-velocity web application\""</span></div>
                            <div class="pt-1"><span class="token-macro">"[server]"</span></div>
                            <div><span class="token-attr">"port"</span> <span class="token-punct">"="</span> <span class="token-num">"3000"</span></div>
                            <div><span class="token-attr">"host"</span> <span class="token-punct">"="</span> <span class="token-string">"\"127.0.0.1\""</span></div>
                            <div class="pt-1"><span class="token-macro">"[tailwind]"</span></div>
                            <div><span class="token-attr">"enabled"</span> <span class="token-punct">"="</span> <span class="token-bool">"true"</span></div>
                            <div><span class="token-attr">"version"</span> <span class="token-punct">"="</span> <span class="token-string">"\"3.4.17\""</span></div>
                            <div><span class="token-attr">"input"</span> <span class="token-punct">"="</span> <span class="token-string">"\"src/styles/app.css\""</span></div>
                            <div><span class="token-attr">"output"</span> <span class="token-punct">"="</span> <span class="token-string">"\"public/styles.css\""</span></div>
                            <div class="pt-1"><span class="token-macro">"[build]"</span></div>
                            <div><span class="token-attr">"minify"</span> <span class="token-punct">"="</span> <span class="token-bool">"true"</span></div>
                            <div><span class="token-attr">"fingerprint"</span> <span class="token-punct">"="</span> <span class="token-bool">"true"</span></div>
                            <div><span class="token-attr">"pre_compress"</span> <span class="token-punct">"="</span> <span class="token-punct">"["</span><span class="token-string">"\"br\""</span><span class="token-punct">", "</span><span class="token-string">"\"gzip\""</span><span class="token-punct">"]"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"TOML · UTF-8 · package.yavml"</span>
                        <span data-i18n="docs.gs.parser_ver">"Yav Engine Parser v0.1.5"</span>
                    </div>
                </div>

                <div class="table-scroll pb-2">
                    <table class="dtable min-w-[580px]">
                        <thead>
                            <tr>
                                <th data-i18n="docs.gs.th_sec">"Section"</th>
                                <th data-i18n="docs.gs.th_fields">"Key Fields"</th>
                                <th data-i18n="docs.gs.th_def">"Default"</th>
                                <th data-i18n="docs.gs.th_role">"Role"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="text-paper font-semibold">"[package]"</td>
                                <td>"name, version, authors"</td>
                                <td class="text-dim">"—"</td>
                                <td data-i18n="docs.gs.m_pkg">"Project identity & semver versioning"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"[server]"</td>
                                <td>"port, host, cors, https"</td>
                                <td class="text-dim">"3000, 127.0.0.1"</td>
                                <td data-i18n="docs.gs.m_srv">"Axum HTTP listener & TLS configuration"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"[tailwind]"</td>
                                <td>"enabled, version, input, output"</td>
                                <td class="text-dim">"3.4.17"</td>
                                <td data-i18n="docs.gs.m_tw">"Automated standalone compiler integration"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"[motion]"</td>
                                <td>"enabled, reduced_motion"</td>
                                <td class="text-dim">"true, user"</td>
                                <td data-i18n="docs.gs.m_mot">"Physics spring runtime injection"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"[build]"</td>
                                <td>"minify, fingerprint, pre_compress"</td>
                                <td class="text-dim">"true, br+gz"</td>
                                <td data-i18n="docs.gs.m_bld">"Production asset hashing & compression"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>

            // =========================================================================
            // 1.5 YAV CLI TOOLBELT
            // =========================================================================
            <section id="cli-toolbelt" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"1.5"</span>
                    <span class="t" data-i18n="docs.gs.1_5_sec">"Yav CLI Toolbelt (20 Production Commands)"</span>
                    <span class="sec-rule"></span>
                    <span class="r" data-i18n="docs.gs.1_5_r">"toolbelt"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.gs.1_5_title">"A unified toolchain for the entire lifecycle."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.gs.1_5_lede1">"The "</span>
                    <code class="inline">"yav"</code>
                    <span data-i18n="docs.gs.1_5_lede2">" binary replaces external task runners, migration managers, icon fetchers, container generators, and static site crawlers with 20 purpose-built subcommands."</span>
                </p>

                <div class="table-scroll pb-2">
                    <table class="dtable min-w-[640px]">
                    <thead>
                        <tr>
                            <th data-i18n="docs.gs.th_cmd">"Command"</th>
                            <th data-i18n="docs.gs.th_args">"Arguments & Flags"</th>
                            <th data-i18n="docs.gs.th_desc">"Description"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav new <name>"</td>
                            <td class="text-dim">"—"</td>
                            <td data-i18n="docs.gs.c_new">"Scaffolds a complete project with routes, layouts, and Tailwind"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav dev"</td>
                            <td class="text-dim">"--port 3000, --host"</td>
                            <td data-i18n="docs.gs.c_dev">"Runs dev server with live SSE hot reload and dark error overlay"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav doctor"</td>
                            <td class="text-dim">"—"</td>
                            <td data-i18n="docs.gs.c_doc">"Runs comprehensive diagnostic audit on toolchains, manifest, routes, and port"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav routes"</td>
                            <td class="text-dim">"—"</td>
                            <td data-i18n="docs.gs.c_rte">"Inspects file-based route table, HTTP handlers, dynamic parameters, and layouts"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav export"</td>
                            <td class="text-dim">"--output dist, --base-url"</td>
                            <td data-i18n="docs.gs.c_exp">"Static site generation (SSG) — exports all static pages to HTML in dist/"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav update"</td>
                            <td class="text-dim">"--check, --self, --project"</td>
                            <td data-i18n="docs.gs.c_upd">"Upgrades CLI binary, manifest dependencies, and Tailwind binary (aliases: up, upgrade)"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav add <component>"</td>
                            <td class="text-dim">"auth-card, drawer, theme-selector..."</td>
                            <td data-i18n="docs.gs.c_add">"Copies production UI component code directly into src/components/"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav build"</td>
                            <td class="text-dim">"--release"</td>
                            <td data-i18n="docs.gs.c_bld">"Minifies CSS, fingerprints assets, pre-compresses Brotli/Gzip"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav g <type> <name>"</td>
                            <td class="text-dim">"route, component, layout"</td>
                            <td data-i18n="docs.gs.c_gen">"Generates boilerplate code adhering to Yav conventions (alias: yav generate)"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav check"</td>
                            <td class="text-dim">"—"</td>
                            <td data-i18n="docs.gs.c_chk">"Validates project manifest, checks Tailwind CSS assets, and runs cargo check"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav test"</td>
                            <td class="text-dim">"—"</td>
                            <td data-i18n="docs.gs.c_tst">"Runs the workspace test suite across all project modules"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav run <script>"</td>
                            <td class="text-dim">"<script-name>"</td>
                            <td data-i18n="docs.gs.c_run">"Executes custom scripts defined in package.yavml [scripts] section"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav icon <action>"</td>
                            <td class="text-dim">"list, search <term>, info"</td>
                            <td data-i18n="docs.gs.c_ico">"Terminal explorer for 13,000+ embedded icons across 3 sets (Lucide, MynaUI, Phosphor)"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav ext <action>"</td>
                            <td class="text-dim">"install, status"</td>
                            <td data-i18n="docs.gs.c_ext">"Installs or verifies official Yav syntax extension for VS Code, Cursor, Insiders, VSCodium"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav deploy <target>"</td>
                            <td class="text-dim">"fly, shuttle, railway, systemd"</td>
                            <td data-i18n="docs.gs.c_dep">"Scaffolds production deployment configurations for cloud providers and Linux Systemd"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav docker"</td>
                            <td class="text-dim">"--force"</td>
                            <td data-i18n="docs.gs.c_dkr">"Generates multi-stage distroless Alpine Dockerfile (< 30 MB)"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav codegen"</td>
                            <td class="text-dim">"--ts, --openapi"</td>
                            <td data-i18n="docs.gs.c_cod">"Exports TypeScript client interfaces or OpenAPI 3.1 specifications"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav analyze"</td>
                            <td class="text-dim">"—"</td>
                            <td data-i18n="docs.gs.c_ana">"Audits routes, CSS purge efficiency, asset sizes, and production readiness scorecard"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav pwa"</td>
                            <td class="text-dim">"--force"</td>
                            <td data-i18n="docs.gs.c_pwa">"Scaffolds offline Service Worker, Web App Manifest, and fallback offline page"</td>
                        </tr>
                        <tr>
                            <td class="text-paper font-semibold">"yav db <action>"</td>
                            <td class="text-dim">"init, new <name>, status"</td>
                            <td data-i18n="docs.gs.c_db">"Timestamped SQL database migrations management and .env initialization"</td>
                        </tr>
                        <tr class="us">
                            <td class="text-paper font-semibold">"yav sitemap"</td>
                            <td class="text-dim">"--base-url <url>"</td>
                            <td data-i18n="docs.gs.c_map">"Scans routes and writes public/sitemap.xml and robots.txt"</td>
                        </tr>
                    </tbody>
                </table>
                </div>
            </section>
        </div>
    }
}
