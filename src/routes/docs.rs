//! Comprehensive Documentation & Getting Started for Yav.rs.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn docs(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="max-w-7xl mx-auto px-4 sm:px-8 py-10">
                <div class="grid grid-cols-1 lg:grid-cols-12 gap-10">
                    // Sidebar Navigation (3 cols)
                    <aside class="hidden lg:block lg:col-span-3 space-y-6 text-xs font-mono sticky top-24 h-fit">
                        <div class="space-y-2">
                            <span class="text-[10px] uppercase font-bold text-amber-500 tracking-wider">
                                "Getting Started"
                            </span>
                            <ul class="space-y-1 text-zinc-600 dark:text-zinc-400">
                                <li><a href="#quickstart" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Quickstart in 60s"</a></li>
                                <li><a href="#cli-installation" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"CLI Installation"</a></li>
                                <li><a href="#project-structure" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Project Anatomy"</a></li>
                            </ul>
                        </div>

                        <div class="space-y-2">
                            <span class="text-[10px] uppercase font-bold text-zinc-400 dark:text-zinc-500 tracking-wider">
                                "Core Concepts"
                            </span>
                            <ul class="space-y-1 text-zinc-600 dark:text-zinc-400">
                                <li><a href="#rsx-templates" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"RSX Syntax &amp; Macros"</a></li>
                                <li><a href="#components-props" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Components &amp; Props"</a></li>
                                <li><a href="#layouts-nesting" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Layouts &amp; Hierarchy"</a></li>
                            </ul>
                        </div>

                        <div class="space-y-2">
                            <span class="text-[10px] uppercase font-bold text-zinc-400 dark:text-zinc-500 tracking-wider">
                                "Routing &amp; HTTP"
                            </span>
                            <ul class="space-y-1 text-zinc-600 dark:text-zinc-400">
                                <li><a href="#filesystem-routing" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"File-System Routing"</a></li>
                                <li><a href="#dynamic-parameters" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Dynamic Parameters [id]"</a></li>
                                <li><a href="#forms-csrf" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Forms &amp; CSRF Defense"</a></li>
                                <li><a href="#signed-sessions" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Encrypted Sessions"</a></li>
                            </ul>
                        </div>

                        <div class="space-y-2">
                            <span class="text-[10px] uppercase font-bold text-zinc-400 dark:text-zinc-500 tracking-wider">
                                "Tooling &amp; Production"
                            </span>
                            <ul class="space-y-1 text-zinc-600 dark:text-zinc-400">
                                <li><a href="#tailwind-pipeline" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Tailwind Pipeline"</a></li>
                                <li><a href="#yav-motion" class="block py-1 hover:text-amber-500 text-amber-600 dark:text-amber-400 font-semibold transition-colors">"Motion &amp; Physics"</a></li>
                                <li><a href="#asset-fingerprinting" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Production Bundling"</a></li>
                                <li><a href="#docker-deployment" class="block py-1 hover:text-zinc-900 dark:hover:text-white transition-colors">"Docker &amp; Single Binary"</a></li>
                            </ul>
                        </div>
                    </aside>

                    // Main Content (9 cols)
                    <main class="lg:col-span-9 space-y-16">
                        // Header
                        <div class="space-y-4 border-b border-zinc-200 dark:border-zinc-800 pb-8">
                            <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-amber-50 dark:bg-amber-950/40 border border-amber-200 dark:border-amber-800 text-xs font-mono text-amber-700 dark:text-amber-400">
                                {render_icon("book-open", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                <span>"Official Documentation • v0.1.3"</span>
                            </div>
                            <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight text-zinc-900 dark:text-white">
                                "Getting Started with Yav"
                            </h1>
                            <p class="text-lg text-zinc-600 dark:text-zinc-400 leading-relaxed max-w-3xl font-sans">
                                "Learn how to build high-velocity full-stack web applications with Rust, Axum 0.8, and compile-time RSX templates."
                            </p>
                        </div>

                        // Section 1: Quickstart
                        <section id="quickstart" class="space-y-6">
                            <h2 class="text-2xl font-bold tracking-tight text-zinc-900 dark:text-white flex items-center gap-2">
                                <span class="text-amber-500">"#"</span>
                                <span>"Quickstart in 60 Seconds"</span>
                            </h2>
                            <p class="text-sm text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "The fastest way to create a new Yav project is through the official CLI toolchain. Run the following commands in your terminal:"
                            </p>

                            <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-950 p-5 font-mono text-xs text-zinc-200 space-y-3 shadow-sm">
                                <div class="flex items-center justify-between text-zinc-400 border-b border-zinc-800 pb-2">
                                    <span>"Terminal Session"</span>
                                    <button
                                        onclick="navigator.clipboard.writeText('cargo install yav-cli\nyav new my-app\ncd my-app\nyav dev'); alertNotification('Copied quickstart commands!');"
                                        class="text-zinc-400 hover:text-white cursor-pointer flex items-center gap-1"
                                    >
                                        {render_icon("copy", IconSize::Sm, Some("w-3.5 h-3.5"), None)}
                                        <span>"copy all"</span>
                                    </button>
                                </div>
                                <div class="space-y-1.5">
                                    <div class="text-zinc-500">"# 1. Install the standalone CLI"</div>
                                    <div class="text-amber-400">"cargo install yav-cli"</div>
                                    <div class="text-zinc-500 pt-2">"# 2. Scaffold a complete project"</div>
                                    <div class="text-amber-400">"yav new my-app"</div>
                                    <div class="text-zinc-500 pt-2">"# 3. Start live development server"</div>
                                    <div class="text-amber-400">"cd my-app &amp;&amp; yav dev"</div>
                                </div>
                            </div>
                        </section>

                        // Section 2: Project Anatomy
                        <section id="project-structure" class="space-y-6">
                            <h2 class="text-2xl font-bold tracking-tight text-zinc-900 dark:text-white flex items-center gap-2">
                                <span class="text-amber-500">"#"</span>
                                <span>"Project Anatomy"</span>
                            </h2>
                            <p class="text-sm text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "Every Yav application follows a clean, predictable structure designed to scale from small microservices to large web portals:"
                            </p>

                            <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-950 p-5 font-mono text-xs text-zinc-300 space-y-2">
                                <div class="text-zinc-400">"my-app/"</div>
                                <div class="pl-4 text-zinc-400">"├── package.yavml        "<span class="text-zinc-500">"# Framework configuration (port, tailwind, build)"</span></div>
                                <div class="pl-4 text-zinc-400">"├── Cargo.toml           "<span class="text-zinc-500">"# Rust dependencies (yav, axum, tokio)"</span></div>
                                <div class="pl-4 text-zinc-400">"├── public/              "<span class="text-zinc-500">"# Static assets (styles.css, images, favicons)"</span></div>
                                <div class="pl-4 text-zinc-400">"└── src/"</div>
                                <div class="pl-8 text-zinc-400">"├── main.rs          "<span class="text-zinc-500">"# Server entrypoint (YavApp &amp; auto_routes!)"</span></div>
                                <div class="pl-8 text-zinc-400">"├── layouts/         "<span class="text-zinc-500">"# Shared application layouts (app_layout.rs)"</span></div>
                                <div class="pl-8 text-zinc-400">"├── components/      "<span class="text-zinc-500">"# Reusable UI components"</span></div>
                                <div class="pl-8 text-zinc-400">"├── routes/          "<span class="text-zinc-500">"# File-system route handlers (index.rs, about.rs)"</span></div>
                                <div class="pl-8 text-zinc-400">"└── styles/          "<span class="text-zinc-500">"# Tailwind source input (app.css)"</span></div>
                            </div>
                        </section>

                        // Section 3: RSX Syntax
                        <section id="rsx-templates" class="space-y-6">
                            <h2 class="text-2xl font-bold tracking-tight text-zinc-900 dark:text-white flex items-center gap-2">
                                <span class="text-amber-500">"#"</span>
                                <span>"RSX Syntax &amp; Macros"</span>
                            </h2>
                            <p class="text-sm text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "Yav uses the `rsx!` procedural macro to write declarative HTML templates directly in Rust. RSX is validated strictly at compile time, eliminating syntax errors, unclosed tags, and type mismatches before runtime."
                            </p>

                            <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-950 p-5 font-mono text-xs text-zinc-200 space-y-2 overflow-x-auto">
                                <div><span class="token-keyword">"let"</span><span> " title = "</span><span class="token-string">"\"Hello, Yav!\""</span><span>";"</span></div>
                                <div><span class="token-keyword">"let"</span><span> " count = "</span><span class="token-val">"5"</span><span>";"</span></div>
                                <div class="pt-2"><span class="token-keyword">"let"</span><span> " node = "</span><span class="token-macro">"rsx!"</span><span> " {"</span></div>
                                <div class="pl-4"><span class="token-tag">"&lt;div"</span> <span class="token-attr">"class="</span><span class="token-string">"\"p-4 rounded-lg bg-zinc-900\""</span><span class="token-tag">"&gt;"</span></div>
                                <div class="pl-8"><span class="token-tag">"&lt;h1"</span> <span class="token-attr">"class="</span><span class="token-string">"\"text-xl font-bold\""</span><span class="token-tag">"&gt;"</span><span>"{title}"</span><span class="token-tag">"&lt;/h1&gt;"</span></div>
                                <div class="pl-8"><span class="token-keyword">"if"</span><span> " count &gt; 0 {"</span></div>
                                <div class="pl-12"><span class="token-tag">"&lt;span"</span> <span class="token-attr">"class="</span><span class="token-string">"\"text-emerald-400\""</span><span class="token-tag">"&gt;"</span><span class="token-string">"\"Count is positive\""</span><span class="token-tag">"&lt;/span&gt;"</span></div>
                                <div class="pl-8"><span>"}"</span></div>
                                <div class="pl-4"><span class="token-tag">"&lt;/div&gt;"</span></div>
                                <div><span>"};"</span></div>
                            </div>
                        </section>

                        // Section 4: File-System Routing
                        <section id="filesystem-routing" class="space-y-6">
                            <h2 class="text-2xl font-bold tracking-tight text-zinc-900 dark:text-white flex items-center gap-2">
                                <span class="text-amber-500">"#"</span>
                                <span>"File-System Routing &amp; Auto-Discovery"</span>
                            </h2>
                            <p class="text-sm text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                "The `auto_routes!(app)` macro scans `src/routes/` at compile time and wires route handlers directly onto the Axum router with constant-time radix table lookups."
                            </p>

                            <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 divide-y divide-zinc-200 dark:divide-zinc-800 text-xs font-mono overflow-hidden">
                                <div class="px-4 py-3 bg-zinc-50 dark:bg-zinc-950 font-bold text-zinc-500 flex justify-between">
                                    <span>"File Path"</span>
                                    <span>"Registered HTTP Route"</span>
                                </div>
                                <div class="px-4 py-3 flex justify-between">
                                    <span class="text-zinc-800 dark:text-zinc-200">"src/routes/index.rs"</span>
                                    <span class="text-amber-600 dark:text-amber-400 font-bold">"GET /"</span>
                                </div>
                                <div class="px-4 py-3 flex justify-between">
                                    <span class="text-zinc-800 dark:text-zinc-200">"src/routes/about.rs"</span>
                                    <span class="text-amber-600 dark:text-amber-400 font-bold">"GET /about"</span>
                                </div>
                                <div class="px-4 py-3 flex justify-between">
                                    <span class="text-zinc-800 dark:text-zinc-200">"src/routes/users/[id].rs"</span>
                                    <span class="text-amber-600 dark:text-amber-400 font-bold">"GET /users/{id}"</span>
                                </div>
                                <div class="px-4 py-3 flex justify-between">
                                    <span class="text-zinc-800 dark:text-zinc-200">"src/routes/api/health.rs"</span>
                                    <span class="text-amber-600 dark:text-amber-400 font-bold">"GET /api/health"</span>
                                </div>
                            </div>
                        </section>

                        // Section 5: Declarative Motion & Physics (yav-motion)
                        <section id="yav-motion" class="space-y-6 pt-6 border-t border-zinc-200 dark:border-zinc-800">
                            <h2 class="text-2xl font-bold tracking-tight text-zinc-900 dark:text-white flex items-center gap-2">
                                <span class="text-amber-500">"#"</span>
                                <span>"Declarative Motion &amp; Spring Physics (yav-motion)"</span>
                            </h2>
                            <p class="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed font-sans">
                                "Yav provides a declarative animation system inspired by Framer Motion. Components animate smoothly using CSS transforms, spring dynamics, and scroll triggers without pulling in hundreds of kilobytes of client JavaScript libraries."
                            </p>

                            <div class="space-y-4">
                                <h3 class="text-base font-bold text-zinc-900 dark:text-white">"1. Declarative Motion Component"</h3>
                                <div class="p-4 rounded-xl bg-zinc-950 text-zinc-200 font-mono text-xs overflow-x-auto leading-relaxed shadow-inner">
                                    <div><span class="token-keyword">"use"</span><span> " yav::prelude::*;"</span></div>
                                    <div class="pt-2"><span class="token-macro">"rsx!"</span><span> " {"</span></div>
                                    <div class="pl-4"><span class="token-tag">"&lt;Motion"</span></div>
                                    <div class="pl-8"><span class="token-attr">"initial="</span><span class="token-val">"{MotionState::new().opacity(0.0).y(24.0)}"</span></div>
                                    <div class="pl-8"><span class="token-attr">"animate="</span><span class="token-val">"{MotionState::new().opacity(1.0).y(0.0)}"</span></div>
                                    <div class="pl-8"><span class="token-attr">"while_hover="</span><span class="token-val">"{MotionState::new().scale(1.02).y(-2.0)}"</span></div>
                                    <div class="pl-8"><span class="token-attr">"while_tap="</span><span class="token-val">"{MotionState::new().scale(0.98)}"</span></div>
                                    <div class="pl-8"><span class="token-attr">"transition="</span><span class="token-val">"{Transition::spring().stiffness(260.0).damping(20.0)}"</span></div>
                                    <div class="pl-8"><span class="token-attr">"in_view="</span><span class="token-val">"{true}"</span></div>
                                    <div class="pl-4"><span class="token-tag">"&gt;"</span></div>
                                    <div class="pl-8"><span class="token-tag">"&lt;Card&gt;"</span><span>"\"Physical spring animation\""</span><span class="token-tag">"&lt;/Card&gt;"</span></div>
                                    <div class="pl-4"><span class="token-tag">"&lt;/Motion&gt;"</span></div>
                                    <div><span>"}"</span></div>
                                </div>
                            </div>

                            <div class="space-y-4 pt-2">
                                <h3 class="text-base font-bold text-zinc-900 dark:text-white">"2. Top Scroll Reading Indicator"</h3>
                                <p class="text-xs text-zinc-600 dark:text-zinc-400 font-sans leading-relaxed">
                                    "Place &lt;ScrollProgressBar /&gt; inside your top-level layout to automatically track scroll percentage from 0% to 100% across the viewport."
                                </p>
                                <div class="p-4 rounded-xl bg-zinc-950 text-zinc-200 font-mono text-xs overflow-x-auto leading-relaxed shadow-inner">
                                    <div><span class="token-tag">"&lt;ScrollProgressBar"</span></div>
                                    <div class="pl-4"><span class="token-attr">"class="</span><span class="token-string">"\"fixed top-0 left-0 right-0 h-[2.5px] bg-gradient-to-r from-amber-500 to-orange-500 z-50 origin-left\""</span></div>
                                    <div><span class="token-tag">"/&gt;"</span></div>
                                </div>
                            </div>
                        </section>
                    </main>
                </div>
            </div>
        </AppLayout>
    };
    HtmlPage::new(page)
        .title("Documentation — Yav.rs")
        .favicon("/public/favicon.svg")
}
