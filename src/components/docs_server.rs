//! Documentation Section 3: Routing & Server (Radix File Routing, Dynamic Routes, Axum 0.8, HMAC Sessions, CSRF & Rate Limiting)

use yav::prelude::*;

#[component]
pub fn DocsServer() -> Node {
    rsx! {
        <div class="space-y-20">
            // =========================================================================
            // 3.1 RADIX FILE ROUTING
            // =========================================================================
            <section id="filesystem-routing" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.1"</span>
                    <span class="t" data-i18n="docs.srv.3_1_sec">"File-System Routing & Radix Trees"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"auto_routes!"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_1_title">"The directory structure is the router."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_1_p1">"The "</span>
                    <code class="inline">"auto_routes!(app)"</code>
                    <span data-i18n="docs.srv.3_1_p2">" macro inspects your "</span>
                    <code class="inline">"src/routes/"</code>
                    <span data-i18n="docs.srv.3_1_p3">" directory at compile time, discovers all page handlers, and compiles them into constant-time O(k) Radix routing trees via "</span>
                    <code class="inline">"matchit"</code>
                    <span data-i18n="docs.srv.3_1_p4">"."</span>
                </p>

                <div class="table-scroll pb-2">
                    <table class="dtable min-w-[620px]">
                        <thead>
                            <tr>
                                <th data-i18n="docs.srv.th_path">"File Path"</th>
                                <th data-i18n="docs.srv.th_pat">"Pattern"</th>
                                <th data-i18n="docs.srv.th_meth">"HTTP Method & Path"</th>
                                <th data-i18n="docs.srv.th_ext">"Parameter Extractor"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="us">
                                <td class="text-paper font-semibold">"src/routes/index.rs"</td>
                                <td data-i18n="docs.srv.pat_root">"Root Index"</td>
                                <td class="text-paper font-semibold">"GET /"</td>
                                <td class="text-dim">"—"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"src/routes/about.rs"</td>
                                <td data-i18n="docs.srv.pat_static">"Static Endpoint"</td>
                                <td class="text-paper font-semibold">"GET /about"</td>
                                <td class="text-dim">"—"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"src/routes/users/_id_.rs"</td>
                                <td data-i18n="docs.srv.pat_dyn">"Dynamic parameter"</td>
                                <td class="text-paper font-semibold">"GET /users/{id}"</td>
                                <td class="text-oxide">"Path(id): Path<u64>"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"src/routes/docs/__slug__.rs"</td>
                                <td data-i18n="docs.srv.pat_wild">"Catch-all wildcard"</td>
                                <td class="text-paper font-semibold">"GET /docs/{*slug}"</td>
                                <td class="text-oxide">"Path(slug): Path<String>"</td>
                            </tr>
                            <tr class="us">
                                <td class="text-paper font-semibold">"src/routes/(auth)/login.rs"</td>
                                <td data-i18n="docs.srv.pat_grp">"Route grouping"</td>
                                <td class="text-paper font-semibold">"GET /login"</td>
                                <td class="text-dim" data-i18n="docs.srv.ext_grp">"— (folder excluded from URL)"</td>
                            </tr>
                            <tr>
                                <td class="text-paper font-semibold">"src/routes/api/health.rs"</td>
                                <td data-i18n="docs.srv.pat_rest">"REST API"</td>
                                <td class="text-paper font-semibold">"GET /api/health"</td>
                                <td class="text-paper">"Json<HealthResponse>"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>

            // =========================================================================
            // 3.2 DYNAMIC PARAMS & SLUGS
            // =========================================================================
            <section id="dynamic-routes" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.2"</span>
                    <span class="t" data-i18n="docs.srv.3_2_sec">"Dynamic Parameters & Catch-All Slugs"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"Path"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_2_title">"Strongly-typed URL parameter extraction."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_2_p1">"Dynamic parameters in filenames (either "</span>
                    <code class="inline">"_id_.rs"</code>
                    <span data-i18n="docs.srv.3_2_p2">" or "</span>
                    <code class="inline">"[id].rs"</code>
                    <span data-i18n="docs.srv.3_2_p3">") are validated and converted into typed Rust structs at runtime with zero boilerplate."</span>
                </p>

                // IDE LOOK WINDOW
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("file", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/routes/users/_id_.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-dim font-mono" data-i18n="docs.srv.3_2_badge">"Dynamic Handler"</span>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"use "</span>"axum::extract::"<span class="token-type">"Path"</span>";"</div>
                            <div><span class="token-keyword">"use "</span>"yav::server::"<span class="token-type">"HtmlPage"</span>";"</div>
                            <div class="pt-1"><span class="token-keyword">"pub async fn "</span><span class="token-fn">"user_detail"</span>"(Path(id): "<span class="token-type">"Path"</span>"&lt;"<span class="token-type">"u64"</span>"&gt;) -> "<span class="token-type">"HtmlPage"</span>" {"</div>
                            <div>"    "<span class="token-type">"HtmlPage"</span>"::new("<span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"        "<span class="token-tag">"<AppLayout>"</span></div>
                            <div>"            "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"wrap py-12\""</span><span class="token-tag">">"</span></div>
                            <div>"                "<span class="token-tag">"<h1 "</span><span class="token-attr">"class="</span><span class="token-string">"\"text-3xl font-bold\""</span><span class="token-tag">">"</span>"User Profile #{id}"<span class="token-tag">"</h1>"</span></div>
                            <div>"            "<span class="token-tag">"</div>"</span></div>
                            <div>"        "<span class="token-tag">"</AppLayout>"</span></div>
                            <div>"    }))"</div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.srv.3_2_st_l">"Rust · Path<u64> extractor"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span><span data-i18n="docs.srv.3_2_st_r">"Radix fast path"</span></span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.3 AXUM 0.8 & TOKIO ENGINE
            // =========================================================================
            <section id="server-architecture" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.3"</span>
                    <span class="t" data-i18n="docs.srv.3_3_sec">"Axum 0.8 & Tokio Server Foundation"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"hyper 1.0"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_3_title">"No black boxes. Native Tokio asynchronous stack."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="docs.srv.3_3_lede">
                    "Yav is built directly on top of Axum 0.8 and Tokio, retaining full access to Tower middleware services, WebSockets, Server-Sent Events, connection pools, and hyper 1.0 high-throughput primitives."
                </p>

                // IDE LOOK WINDOW
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/main.rs"
                            </div>
                        </div>
                        <button data-copy="use yav::server::{YavApp, RateLimitConfig};\nuse std::time::Duration;\n\n#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {\n    let app = YavApp::new()\n        .rate_limit(RateLimitConfig::new().limit_per_window(120).window_duration(Duration::from_secs(60)))\n        .static_dir(&quot;public&quot;);\n    auto_routes!(app);\n    app.run().await\n}" onclick="copyText(this)" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium">"copy"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"use "</span>"yav::server::{"<span class="token-type">"YavApp"</span>", "<span class="token-type">"RateLimitConfig"</span>"};"</div>
                            <div><span class="token-keyword">"use "</span>"std::time::"<span class="token-type">"Duration"</span>";"</div>
                            <div class="pt-1"><span class="token-macro">"#[tokio::main]"</span></div>
                            <div><span class="token-keyword">"async fn "</span><span class="token-fn">"main"</span>"() -> "<span class="token-type">"Result"</span>"&lt;(), "<span class="token-type">"Box"</span>"&lt;"<span class="token-keyword">"dyn "</span><span class="token-type">"std::error::Error"</span>"&gt;&gt; {"</div>
                            <div>"    "<span class="token-keyword">"let "</span>"app = "<span class="token-type">"YavApp"</span>"::new()"</div>
                            <div>"        .rate_limit("<span class="token-type">"RateLimitConfig"</span>"::new().limit_per_window("<span class="token-num">"120"</span>").window_duration("<span class="token-type">"Duration"</span>"::from_secs("<span class="token-num">"60"</span>")))"</div>
                            <div>"        .static_dir("<span class="token-string">"\"public\""</span>");"</div>
                            <div class="pt-1 token-comment">"    // auto_routes registers all files under src/routes/"</div>
                            <div>"    "<span class="token-macro">"auto_routes!"</span>"(app);"</div>
                            <div class="pt-1">"    app.run()."<span class="token-keyword">"await"</span></div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.srv.3_3_st_l">"Axum 0.8.1 · Tokio Multi-threaded Worker Pool"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"124,500 req/s"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.4 HMAC CRYPTOGRAPHIC SESSIONS
            // =========================================================================
            <section id="security-sessions" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.4"</span>
                    <span class="t" data-i18n="docs.srv.3_4_sec">"Cryptographic Sessions & Signed Flash Cookies"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"HMAC-SHA256"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_4_title">"Tamper-proof stateless cookies with constant-time verification."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="docs.srv.3_4_lede">
                    "Sessions and one-time notifications in Yav are cryptographically signed using HMAC-SHA256. Signatures are verified in constant time to eliminate timing side-channel attacks."
                </p>

                // IDE LOOK WINDOW
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("lock", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/routes/dashboard.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono" data-i18n="docs.srv.3_4_badge">"HMAC Signed"</span>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"<br/>"13"<br/>"14"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"use "</span>"yav::server::{"<span class="token-type">"HtmlPage"</span>", "<span class="token-type">"Session"</span>", "<span class="token-type">"Flash"</span>"};"</div>
                            <div class="pt-1"><span class="token-keyword">"pub async fn "</span><span class="token-fn">"dashboard"</span>"(session: "<span class="token-type">"Session"</span>", flash: "<span class="token-type">"Flash"</span>") -> "<span class="token-type">"Result"</span>"&lt;"<span class="token-type">"HtmlPage"</span>", "<span class="token-type">"StatusCode"</span>"&gt; {"</div>
                            <div>"    "<span class="token-keyword">"let "</span>"user = session.get::&lt;"<span class="token-type">"User"</span>"&gt;("<span class="token-string">"\"user\""</span>")"</div>
                            <div>"        .ok_or("<span class="token-type">"StatusCode"</span>"::UNAUTHORIZED)?;"</div>
                            <div class="pt-1">"    Ok("<span class="token-type">"HtmlPage"</span>"::new("<span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"        "<span class="token-tag">"<AppLayout>"</span></div>
                            <div>"            {if let Some(msg) = flash.get() {"</div>
                            <div>"                "<span class="token-macro">"rsx!"</span>" { "<span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"alert-box alert-ok\""</span><span class="token-tag">">"</span>"{msg}"<span class="token-tag">"</div>"</span>" }"</div>
                            <div>"            }}"</div>
                            <div>"            "<span class="token-tag">"<h2>"</span>"Welcome back, {user.name}"<span class="token-tag">"</h2>"</span></div>
                            <div>"        "<span class="token-tag">"</AppLayout>"</span></div>
                            <div>"    })))"</div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span>"constant_time_eq verification · HttpOnly; SameSite=Lax; Secure"</span>
                        <span data-i18n="docs.srv.3_4_st_r">"5-min Flash TTL"</span>
                    </div>
                </div>

                <div class="grid sm:grid-cols-2 gap-4 text-xs font-mono">
                    <div class="plate p-4 bg-ink-2 space-y-1.5">
                        <div class="text-paper font-semibold" data-i18n="docs.srv.3_4_c1_t">"Cookie Injection Hardening"</div>
                        <p class="text-mute text-[11.5px] leading-relaxed" data-i18n="docs.srv.3_4_c1_d">"Cookie names, paths, and domains are sanitized against CRLF injection (; \\r \\n \\0), and payloads > 4 KB are discarded before HMAC processing."</p>
                    </div>
                    <div class="plate p-4 bg-ink-2 space-y-1.5">
                        <div class="text-paper font-semibold" data-i18n="docs.srv.3_4_c2_t">"Flash Replay Prevention"</div>
                        <p class="text-mute text-[11.5px] leading-relaxed" data-i18n="docs.srv.3_4_c2_d">"Flash messages embed signed 5-minute timestamps (DEFAULT_FLASH_TTL_SECS = 300) preventing replay attacks across browser restarts."</p>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.5 CSRF & SLIDING RATE LIMITER
            // =========================================================================
            <section id="security-hardening" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.5"</span>
                    <span class="t" data-i18n="docs.srv.3_5_sec">"CSRF Protection & Sliding-Window Rate Limiting"</span>
                    <span class="sec-rule"></span>
                    <span class="r" data-i18n="docs.srv.3_5_r">"security"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_5_title">"Architectural defense against brute force and forgery."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="docs.srv.3_5_lede">
                    "Yav provides built-in cryptographic CSRF token generation and sliding-window rate limiting to protect endpoints against automated bots and cross-site request attacks."
                </p>

                // IDE LOOK WINDOW
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("shield", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/routes/auth.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-oxide font-mono">"CsrfToken"</span>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"<br/>"08"<br/>"09"<br/>"10"<br/>"11"<br/>"12"<br/>"13"<br/>"14"<br/>"15"<br/>"16"<br/>"17"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div><span class="token-keyword">"use "</span>"yav::server::{"<span class="token-type">"CsrfToken"</span>", "<span class="token-type">"FormData"</span>", "<span class="token-type">"Session"</span>", "<span class="token-type">"YavError"</span>"};"</div>
                            <div class="pt-1 token-comment">"// 1. GET handler: bind cryptographic CSRF token to user session"</div>
                            <div><span class="token-keyword">"pub async fn "</span><span class="token-fn">"login_form"</span>"(session: "<span class="token-type">"Session"</span>") -> "<span class="token-type">"HtmlPage"</span>" {"</div>
                            <div>"    "<span class="token-keyword">"let "</span>"csrf = "<span class="token-type">"CsrfToken"</span>"::from_session(&session);"</div>
                            <div>"    "<span class="token-type">"HtmlPage"</span>"::new("<span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"        "<span class="token-tag">"<form "</span><span class="token-attr">"method="</span><span class="token-string">"\"POST\" "</span><span class="token-attr">"action="</span><span class="token-string">"\"/login\""</span><span class="token-tag">">"</span></div>
                            <div>"            {csrf.hidden_input()}"</div>
                            <div>"            "<span class="token-tag">"<input "</span><span class="token-attr">"name="</span><span class="token-string">"\"email\" "</span><span class="token-attr">"type="</span><span class="token-string">"\"email\" "</span><span class="token-attr">"required"</span><span class="token-tag">" />"</span></div>
                            <div>"            "<span class="token-tag">"<button "</span><span class="token-attr">"type="</span><span class="token-string">"\"submit\""</span><span class="token-tag">">"</span>"Log In"<span class="token-tag">"</button>"</span></div>
                            <div>"        "<span class="token-tag">"</form>"</span></div>
                            <div>"    })"</div>
                            <div>"}"</div>
                            <div class="pt-1 token-comment">"// 2. POST handler: constant-time token verification"</div>
                            <div><span class="token-keyword">"pub async fn "</span><span class="token-fn">"login_action"</span>"(session: "<span class="token-type">"Session"</span>", form: "<span class="token-type">"FormData"</span>") -> "<span class="token-type">"Response"</span>" {"</div>
                            <div>"    "<span class="token-keyword">"let "</span>"csrf = "<span class="token-type">"CsrfToken"</span>"::from_session(&session);"</div>
                            <div>"    "<span class="token-keyword">"if "</span>"!csrf.verify(&form) {"</div>
                            <div>"        "<span class="token-keyword">"return "</span><span class="token-type">"YavError"</span>"::Forbidden("<span class="token-string">"\"Invalid CSRF token\""</span>".into()).into_response();"</div>
                            <div>"    }"</div>
                            <div>"    redirect("<span class="token-string">"\"/dashboard\""</span>")"</div>
                            <div>"}"</div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.srv.3_5_st_l">"CsrfToken::verify(&form) constant-time check"</span>
                        <span class="text-ok" data-i18n="docs.srv.3_5_st_r">"Protected"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.6 IN-MEMORY SSR CACHING & BLAKE3 ETAGS
            // =========================================================================
            <section id="ssr-caching" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.6"</span>
                    <span class="t" data-i18n="docs.srv.3_6_sec">"In-Memory SSR Route Caching & Blake3 ETags"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"get_cached"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_6_title">"Sub-millisecond responses without template re-execution."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_6_p1">"For dynamic content under high load, "</span>
                    <code class="inline">"get_cached"</code>
                    <span data-i18n="docs.srv.3_6_p2">" caches rendered HTML in memory. When incoming requests include "</span>
                    <code class="inline">"If-None-Match"</code>
                    <span data-i18n="docs.srv.3_6_p3">", Yav computes Blake3/SHA-256 ETags and returns 304 Not Modified instantly, saving CPU and network bandwidth."</span>
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/main.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono" data-i18n="docs.srv.3_6_badge">"In-Memory Cache"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// Cache route HTML for 120 seconds with automatic ETag generation"</div>
                        <div><span class="token-keyword">"let "</span>"app = "<span class="token-type">"YavApp"</span>"::new()"</div>
                        <div>"    .etag("<span class="token-bool">"true"</span>") "<span class="token-comment">"// Enables Blake3 / SHA-256 ETags globally"</span></div>
                        <div>"    .get_cached("<span class="token-string">"\"/blog\""</span>", "<span class="token-type">"Duration"</span>"::from_secs("<span class="token-num">"120"</span>"), blog_index_handler)"</div>
                        <div>"    .static_dir("<span class="token-string">"\"public\""</span>");"</div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.srv.3_6_st_l">"Bypasses template rendering & database queries"</span>
                        <span class="text-ok" data-i18n="docs.srv.3_6_st_r">"304 Not Modified Fast-Path"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.7 PROGRESSIVE NAVIGATION & VIEW TRANSITIONS
            // =========================================================================
            <section id="view-transitions" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.7"</span>
                    <span class="t" data-i18n="docs.srv.3_7_sec">"View Transitions API & Type-Safe Reverse Routing"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"url_for!"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_7_title">"SPA fluid transitions with SSR initial load speed."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_7_p1">"Yav's "</span>
                    <code class="inline">"<Link />"</code>
                    <span data-i18n="docs.srv.3_7_p2">" component automatically prefetches target pages in the background and morphs views using the native browser View Transitions API ("</span>
                    <code class="inline">"document.startViewTransition"</code>
                    <span data-i18n="docs.srv.3_7_p3">"), while "</span>
                    <code class="inline">"url_for!"</code>
                    <span data-i18n="docs.srv.3_7_p4">" ensures type-safe reverse routing without hardcoded URL strings."</span>
                </p>

                <div class="grid md:grid-cols-2 gap-6">
                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("link", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="docs.srv.3_7_tab1">"Link with Prefetch"</span>
                            </div>
                            <span class="text-[10px] text-oxide font-mono">"View Transitions"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Prefetches on hover, morphs with View Transitions"</div>
                            <div><span class="token-keyword">"use "</span>"yav::ui::"<span class="token-type">"Link"</span>";"</div>
                            <div class="pt-1"><span class="token-macro">"rsx!"</span>" {"</div>
                            <div>"    "<span class="token-tag">"<Link "</span><span class="token-attr">"href="</span><span class="token-string">"\"/components\" "</span><span class="token-attr">"prefetch="</span><span class="token-bool">"true"</span><span class="token-tag">">"</span></div>
                            <div>"        \"Explore components\""</div>
                            <div>"    "<span class="token-tag">"</Link>"</span></div>
                            <div>"}"</div>
                        </div>
                    </div>

                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("git-branch", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="docs.srv.3_7_tab2">"Reverse Routing"</span>
                            </div>
                            <span class="text-[10px] text-ok font-mono">"url_for!"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Interpolate typed route parameters safely"</div>
                            <div><span class="token-keyword">"use "</span>"yav::router::"<span class="token-fn">"url_for"</span>";"</div>
                            <div class="pt-1"><span class="token-keyword">"let "</span>"user_url = "<span class="token-fn">"url_for"</span>"("</div>
                            <div>"    "<span class="token-string">"\"/users/:id\""</span>","</div>
                            <div>"    &[("<span class="token-string">"\"id\""</span>", "<span class="token-string">"\"42\""</span>")],"</div>
                            <div>");"</div>
                            <div><span class="token-macro">"assert_eq!"</span>"(user_url, "<span class="token-string">"\"/users/42\""</span>");"</div>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.8 TYPED FORM VALIDATION & HARDENING
            // =========================================================================
            <section id="form-validation" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.8"</span>
                    <span class="t" data-i18n="docs.srv.3_8_sec">"Typed Form Validation & Asset Hardening"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"FromFormData"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_8_title">"Typed validation traits and strict path containment."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_8_p1">"Parse and validate multipart and URL-encoded forms with the "</span>
                    <code class="inline">"FromFormData"</code>
                    <span data-i18n="docs.srv.3_8_p2">" trait, while static file serving verifies canonical paths and serves SVGs with strict Content-Security-Policy headers."</span>
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("check-circle", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/forms.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-oxide font-mono">"FromFormData"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// Typed form validation with error aggregation"</div>
                        <div><span class="token-keyword">"impl "</span><span class="token-type">"FromFormData "</span><span class="token-keyword">"for "</span><span class="token-type">"RegisterForm"</span>" {"</div>
                        <div>"    "<span class="token-keyword">"fn "</span><span class="token-fn">"from_form"</span>"(form: &"<span class="token-type">"FormData"</span>") -> "<span class="token-type">"Result"</span>"&lt;"<span class="token-type">"Self"</span>", "<span class="token-type">"FormErrors"</span>"&gt; {"</div>
                        <div>"        "<span class="token-keyword">"let mut "</span>"errors = "<span class="token-type">"FormErrors"</span>"::new();"</div>
                        <div>"        errors.require("<span class="token-string">"\"email\""</span>", form.get_or_empty("<span class="token-string">"\"email\""</span>"), "<span class="token-string">"\"Email is required\""</span>");"</div>
                        <div>"        "<span class="token-keyword">"if "</span>"errors.has_errors() { "<span class="token-type">"Err"</span>"(errors) } "<span class="token-keyword">"else"</span>" { "<span class="token-type">"Ok"</span>"(Self { ... }) }"</div>
                        <div>"    }"</div>
                        <div>"}"</div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.srv.3_8_st_l">"Strict canonicalization against directory traversal"</span>
                        <span class="text-ok" data-i18n="docs.srv.3_8_st_r">"SVG CSP Sandbox Protection"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.9 NATIVE AI SDK, STREAMING UI & MODEL CONTEXT PROTOCOL (MCP)
            // =========================================================================
            <section id="ai-mcp" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.9"</span>
                    <span class="t" data-i18n="docs.srv.3_9_sec">"Native AI SDK, Streaming UI & MCP Server"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"McpServer & RAG"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_9_title">"Built-in Model Context Protocol & Streaming UI."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_9_p1">"Yav includes a native AI runtime: multi-turn "</span>
                    <code class="inline">"Conversation"</code>
                    <span data-i18n="docs.srv.3_9_p2">", JSON-Schema "</span>
                    <code class="inline">"ToolRegistry"</code>
                    <span data-i18n="docs.srv.3_9_p3">", streaming HTML fragments over SSE ("</span>
                    <code class="inline">"stream_ui_fragment"</code>
                    <span data-i18n="docs.srv.3_9_p4">"), dense cosine "</span>
                    <code class="inline">"VectorIndex"</code>
                    <span data-i18n="docs.srv.3_9_p5">" with Reciprocal Rank Fusion (RRF), and a full Model Context Protocol ("</span>
                    <code class="inline">"McpServer"</code>
                    <span data-i18n="docs.srv.3_9_p6">") endpoint exposing live application tools and prompt resources to Claude Desktop, Cursor, and Copilot."</span>
                </p>

                <div class="grid md:grid-cols-2 gap-6">
                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("cpu", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="docs.srv.3_9_tab1">"Model Context Protocol (MCP)"</span>
                            </div>
                            <span class="text-[10px] text-oxide font-mono">"JSON-RPC 2.0"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Expose application tools to AI agents via MCP"</div>
                            <div><span class="token-keyword">"use "</span>"yav::server::mcp::{"<span class="token-type">"McpServer"</span>", "<span class="token-type">"McpTool"</span>"};"</div>
                            <div class="pt-1"><span class="token-keyword">"let "</span>"server = "<span class="token-type">"McpServer"</span>"::new("<span class="token-string">"\"yav-backend\""</span>", "<span class="token-string">"\"1.0\""</span>")"</div>
                            <div>"    .tool("<span class="token-type">"McpTool"</span>"::new("<span class="token-string">"\"query_db\""</span>")"</div>
                            <div>"        .description("<span class="token-string">"\"Executes read-only SQL queries\""</span>")"</div>
                            <div>"        .handler(handle_sql_tool))"</div>
                            <div>"    .resource("<span class="token-string">"\"schema://postgres\""</span>", db_schema);"</div>
                            <div class="pt-1"><span class="token-comment">"// Mounts at POST /_yav/mcp"</span></div>
                            <div>"app.mount_mcp(server);"</div>
                        </div>
                    </div>

                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("zap", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                <span data-i18n="docs.srv.3_9_tab2">"Streaming UI Fragments"</span>
                            </div>
                            <span class="text-[10px] text-ok font-mono">"SSE Stream"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Stream AI-generated RSX fragments progressively"</div>
                            <div><span class="token-keyword">"use "</span>"yav::server::ai::"<span class="token-fn">"stream_ui_fragment"</span>";"</div>
                            <div class="pt-1"><span class="token-keyword">"pub async fn "</span><span class="token-fn">"ai_agent"</span>"(req: "<span class="token-type">"Request"</span>") -&gt; "<span class="token-type">"SseStream"</span>" {"</div>
                            <div>"    "<span class="token-fn">"stream_ui_fragment"</span>"(req, |chunk| {"</div>
                            <div>"        "<span class="token-macro">"rsx!"</span>" { \"&lt;p class='ai-token'&gt;\" {chunk} \"&lt;/p&gt;\" }"</div>
                            <div>"    })."<span class="token-keyword">"await"</span></div>
                            <div>"}"</div>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.10 PROACTIVE RESILIENCE & DISTRIBUTED COORDINATION
            // =========================================================================
            <section id="resilience" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.10"</span>
                    <span class="t" data-i18n="docs.srv.3_10_sec">"Proactive Resilience & Distributed Coordination"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"CircuitBreaker & XFetch"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_10_title">"Zero cascading failures. Automatic fail-safe."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_10_p1">"Built for mission-critical enterprise environments: "</span>
                    <code class="inline">"CircuitBreaker"</code>
                    <span data-i18n="docs.srv.3_10_p2">" isolates struggling microservices, "</span>
                    <code class="inline">"Bulkhead"</code>
                    <span data-i18n="docs.srv.3_10_p3">" limits concurrent execution slots, "</span>
                    <code class="inline">"TransactionalOutbox"</code>
                    <span data-i18n="docs.srv.3_10_p4">" guarantees dual-write consistency, "</span>
                    <code class="inline">"DbReplicaPool"</code>
                    <span data-i18n="docs.srv.3_10_p5">" steers reads to geographic replicas with read-your-own-writes session stickiness, and "</span>
                    <code class="inline">"DistributedCache"</code>
                    <span data-i18n="docs.srv.3_10_p6">" mitigates stampedes via the probabilistic XFetch algorithm."</span>
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("shield", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/resilience.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono">"CircuitBreaker & XFetch"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// Proactive resilience protecting downstream services"</div>
                        <div><span class="token-keyword">"use "</span>"yav::server::resilience::{"<span class="token-type">"CircuitBreaker"</span>", "<span class="token-type">"Bulkhead"</span>"};"</div>
                        <div><span class="token-keyword">"use "</span>"yav::server::distributed_cache::{"<span class="token-type">"DistributedCache"</span>", "<span class="token-type">"MemoryL1Store"</span>"};"</div>
                        <div class="pt-2"><span class="token-keyword">"let "</span>"cb = "<span class="token-type">"CircuitBreaker"</span>"::builder("<span class="token-string">"\"payment-api\""</span>")"</div>
                        <div>"    .failure_threshold(5)"</div>
                        <div>"    .recovery_timeout("<span class="token-type">"Duration"</span>"::from_secs(30))"</div>
                        <div>"    .build();"</div>
                        <div class="pt-2"><span class="token-comment">"// XFetch: probabilistic early recomputation eliminates cache stampedes"</span></div>
                        <div><span class="token-keyword">"let "</span>"cache = "<span class="token-type">"DistributedCache"</span>"::new("<span class="token-type">"MemoryL1Store"</span>"::new(), redis_pool)"</div>
                        <div>"    .xfetch_beta(1.2); "<span class="token-comment">"// probabilistic delta evaluation"</span></div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.srv.3_10_st_l">"Protects DB connection pools and API rate limits from collapse"</span>
                        <span class="text-ok" data-i18n="docs.srv.3_10_st_r">"XFetch Stampede Elimination"</span>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.11 WEBAUTHN / PASSKEYS & REBAC ZERO-TRUST SECURITY
            // =========================================================================
            <section id="passkeys-rebac" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.11"</span>
                    <span class="t" data-i18n="docs.srv.3_11_sec">"Passkeys / WebAuthn & ReBAC Zanzibar Authorization"</span>
                    <span class="sec-rule"></span>
                    <span class="r">"FIDO2 & Zanzibar"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_11_title">"Passwordless biometric auth and graph authorization."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_11_p1">"Native passwordless FIDO2 authentication via "</span>
                    <code class="inline">"PasskeyManager"</code>
                    <span data-i18n="docs.srv.3_11_p2">" supporting Apple Touch ID, Face ID, Windows Hello, and YubiKeys without external cloud auth vendors. For permissions, "</span>
                    <code class="inline">"RebacGraph"</code>
                    <span data-i18n="docs.srv.3_11_p3">" delivers Google Zanzibar-style relationship-based access control with transitive resolution and cycle-safe graph traversals."</span>
                </p>

                <div class="grid md:grid-cols-2 gap-6">
                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("lock", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "PasskeyManager (FIDO2)"
                            </div>
                            <span class="text-[10px] text-ok font-mono" data-i18n="docs.srv.3_11_b1">"WebAuthn Native"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Challenge generation & biometric signature validation"</div>
                            <div><span class="token-keyword">"use "</span>"yav::server::passkeys::"<span class="token-type">"PasskeyManager"</span>";"</div>
                            <div class="pt-1"><span class="token-keyword">"let "</span>"pm = "<span class="token-type">"PasskeyManager"</span>"::new("<span class="token-string">"\"yav.rs\""</span>", "<span class="token-string">"\"https://yav.rs\""</span>");"</div>
                            <div><span class="token-keyword">"let "</span>"challenge = pm.generate_register_challenge(user_id);"</div>
                            <div><span class="token-keyword">"let "</span>"credential = pm.verify_register(user_id, response)?;"</div>
                        </div>
                    </div>

                    <div class="ide-window">
                        <div class="ide-head">
                            <div class="ide-tab">
                                {render_icon("shield", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "ReBAC Zanzibar Graph"
                            </div>
                            <span class="text-[10px] text-oxide font-mono" data-i18n="docs.srv.3_11_b2">"Relationship-Based"</span>
                        </div>
                        <div class="p-4 code overflow-x-auto text-[12.5px] leading-[1.75] bg-[#0b0b0d]">
                            <div class="token-comment">"// Fine-grained graph authorization with transitive roles"</div>
                            <div><span class="token-keyword">"use "</span>"yav::server::rebac::"<span class="token-type">"RebacGraph"</span>";"</div>
                            <div class="pt-1"><span class="token-keyword">"let mut "</span>"graph = "<span class="token-type">"RebacGraph"</span>"::new();"</div>
                            <div>"graph.add_relation("<span class="token-string">"\"doc:42\""</span>", "<span class="token-string">"\"editor\""</span>", "<span class="token-string">"\"user:alice\""</span>");"</div>
                            <div>"graph.add_relation("<span class="token-string">"\"doc:42\""</span>", "<span class="token-string">"\"viewer\""</span>", "<span class="token-string">"\"team:frontend\""</span>");"</div>
                            <div><span class="token-macro">"assert!"</span>"(graph.check("<span class="token-string">"\"user:bob\""</span>", "<span class="token-string">"\"viewer\""</span>", "<span class="token-string">"\"doc:42\""</span>"));"</div>
                        </div>
                    </div>
                </div>
            </section>

            // =========================================================================
            // 3.12 MODERN PROTOCOLS: GRAPHQL, TURBOSTREAM, EVENT SOURCING & TUS
            // =========================================================================
            <section id="modern-protocols" class="space-y-6 scroll-mt-24">
                <div class="sec-head">
                    <span class="n">"3.12"</span>
                    <span class="t" data-i18n="docs.srv.3_12_sec">"GraphQL, TurboStream, Event Sourcing & Tus Uploads"</span>
                    <span class="sec-rule"></span>
                    <span class="r" data-i18n="docs.srv.3_12_r">"Modern Protocols"</span>
                </div>
                <h2 class="text-[30px] sm:text-[34px] text-paper pt-2 leading-tight" data-i18n="docs.srv.3_12_title">"All open standards natively implemented in pure Rust."</h2>
                <p class="text-mute text-[15.5px] leading-relaxed">
                    <span data-i18n="docs.srv.3_12_p1">"Connect any client architecture with zero external proxy dependencies: zero-dependency "</span>
                    <code class="inline">"GraphQlSchema"</code>
                    <span data-i18n="docs.srv.3_12_p2">" with embedded GraphiQL 3.x playground, "</span>
                    <code class="inline">"TurboStream"</code>
                    <span data-i18n="docs.srv.3_12_p3">" 7-action multi-zone DOM mutation streaming, "</span>
                    <code class="inline">"EventStore"</code>
                    <span data-i18n="docs.srv.3_12_p4">" CQRS aggregates with optimistic concurrency, and resumable file uploads conforming to the open "</span>
                    <code class="inline">"Tus Protocol v1.0.0"</code>
                    <span data-i18n="docs.srv.3_12_p5">" with disk spooling protection against out-of-memory denial of service."</span>
                </p>

                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("terminal", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/protocols.rs"
                            </div>
                        </div>
                        <span class="text-[11px] text-ok font-mono">"Tus v1.0.0 & TurboStream"</span>
                    </div>
                    <div class="p-6 code space-y-1.5 overflow-x-auto text-[13px] bg-[#0b0b0d]">
                        <div class="token-comment">"// Hotwire TurboStream multi-zone mutation & Tus resumable uploads"</div>
                        <div><span class="token-keyword">"use "</span>"yav::server::turbo_stream::{"<span class="token-type">"TurboStream"</span>", "<span class="token-type">"TurboAction"</span>"};"</div>
                        <div><span class="token-keyword">"use "</span>"yav::server::resumable_upload::"<span class="token-type">"ChunkedUploader"</span>";"</div>
                        <div class="pt-2"><span class="token-comment">"// 1. Stream HTML fragment targeting container ID without full reload"</span></div>
                        <div><span class="token-keyword">"let "</span>"mutation = "<span class="token-type">"TurboStream"</span>"::new("<span class="token-type">"TurboAction"</span>"::Prepend, "<span class="token-string">"\"messages\""</span>", "<span class="token-string">"\"&lt;div class='msg'&gt;New alert&lt;/div&gt;\""</span>");"</div>
                        <div class="pt-2"><span class="token-comment">"// 2. Resumable chunked file upload with disk spooling"</span></div>
                        <div><span class="token-keyword">"let "</span>"uploader = "<span class="token-type">"ChunkedUploader"</span>"::new("<span class="token-string">"\"uploads/\""</span>").max_file_size(10 * 1024 * 1024 * 1024);"</div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="docs.srv.3_12_st_l">"Full RFC compliance with pure Rust memory safety"</span>
                        <span class="text-ok" data-i18n="docs.srv.3_12_st_r">"Zero Node.js / Zero C Libraries"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
