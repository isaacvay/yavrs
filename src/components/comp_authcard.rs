//! 5.6 Auth Card & Credentials Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentAuthCard() -> Node {
    rsx! {
        <section id="authcard" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.6"</span>
                <span class="t" data-i18n="comp.auth.sec">"Auth Card & Credentials"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add auth-card"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight" data-i18n="comp.auth.title">"Security-first auth card."</h2>
                <button
                    onclick="copyText(this, 'yav add auth-card')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add auth-card"</span>
                    <span data-copy-label="" data-i18n="common.copy" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed" data-i18n="comp.auth.lede">
                "Complete authentication panel for engineering consoles and SaaS dashboards. Integrates single-sign-on OAuth providers, Argon2 password hashing hooks, and strict HMAC CSRF tokens right out of the box."
            </p>

            // Live Interactive Preview Plate
            <div class="plate max-w-lg mx-auto">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-ok"></span>
                        <span data-i18n="comp.auth.preview_head">"AUTH CARD PREVIEW"</span>
                    </span>
                    <span class="text-faint font-mono text-[11px]" data-i18n="comp.auth.preview_sub">"HMAC verified"</span>
                </div>
                <div class="p-8 space-y-6 bg-ink-2">
                    // Card Title
                    <div class="space-y-1 text-center">
                        <div class="text-[20px] font-semibold text-paper" data-i18n="comp.auth.card_title">"Sign in to Console"</div>
                        <p class="text-xs text-mute" data-i18n="comp.auth.card_sub">"Enter your engineer credentials to access the cluster"</p>
                    </div>

                    // OAuth Button
                    <button class="w-full btn btn-line flex items-center justify-center gap-2.5 py-2.5 text-xs font-mono">
                        {render_icon("git-branch", IconSize::Sm, Some("w-4 h-4 text-paper"), None)}
                        <span data-i18n="comp.auth.oauth">"Continue with GitHub OAuth"</span>
                    </button>

                    // Divider
                    <div class="relative flex items-center justify-center">
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full border-t border-[rgba(236,233,227,0.08)]"></div>
                        </div>
                        <span class="relative px-3 bg-ink-2 text-[10px] font-mono uppercase tracking-widest text-dim" data-i18n="comp.auth.or">
                            "or credentials"
                        </span>
                    </div>

                    // Form inputs
                    <form class="space-y-4" onsubmit="event.preventDefault();">
                        <div class="space-y-1.5">
                            <label class="label block text-[11px]" data-i18n="comp.auth.email">"Email Address"</label>
                            <input
                                type="email"
                                class="input-field text-xs font-mono"
                                placeholder="engineer@domain.rs"
                                value="admin@cluster.internal"
                            />
                        </div>

                        <div class="space-y-1.5">
                            <div class="flex items-center justify-between">
                                <label class="label text-[11px]" data-i18n="comp.auth.pw">"Password"</label>
                                <a href="#authcard" class="text-[11px] font-mono text-oxide hover:underline" data-i18n="comp.auth.forgot">"Forgot?"</a>
                            </div>
                            <div class="relative flex items-center">
                                <input
                                    type="password"
                                    id="demo-password-field"
                                    class="input-field pr-16 text-xs font-mono"
                                    value="super-secret-lto-key"
                                />
                                <button
                                    type="button"
                                    id="demo-pw-toggle"
                                    data-i18n="comp.auth.show"
                                    onclick="toggleDemoPassword(this)"
                                    class="absolute right-3 font-mono text-[10px] text-dim hover:text-paper uppercase tracking-wider"
                                >
                                    "Show"
                                </button>
                            </div>
                        </div>

                        <div class="flex items-center justify-between pt-1">
                            <label class="flex items-center gap-2 cursor-pointer text-xs font-mono text-dim">
                                <input type="checkbox" checked="" class="accent-[#e0592f]" />
                                <span data-i18n="comp.auth.remember">"Remember 30 days"</span>
                            </label>
                            <span class="text-[10px] font-mono text-ok flex items-center gap-1">
                                <span class="dot dot-ok"></span>
                                <span data-i18n="comp.auth.aes">"256-bit AES"</span>
                            </span>
                        </div>

                        <button type="submit" class="w-full btn btn-oxide justify-center py-2.5 mt-2">
                            <span data-i18n="comp.auth.submit">"Authenticate Session"</span>
                            <span class="arr">"→"</span>
                        </button>
                    </form>

                    // Security Footer Note
                    <div class="pt-3 border-t border-[rgba(236,233,227,0.08)] flex items-center justify-between text-[11px] font-mono text-dim">
                        <span data-i18n="comp.auth.samesite">"Strict SameSite=Lax"</span>
                        <span class="text-faint" data-i18n="comp.auth.csrf">"CSRF token active"</span>
                    </div>
                </div>
            </div>

            // Code Preview Windows: Tabbed IDE + Terminal
            <div class="space-y-3 pt-2">
                <div class="ide-window">
                    <div class="ide-head">
                        <div class="flex items-center h-full">
                            <div class="ide-tab">
                                {render_icon("code", IconSize::Sm, Some("w-3.5 h-3.5 text-oxide"), None)}
                                "src/components/auth_card.rs"
                            </div>
                        </div>
                        <button onclick="copyText(this, 'auth_card(AuthCardProps { oauth_github: true, csrf_token })')" class="text-dim hover:text-oxide transition-colors text-[11px] flex items-center gap-1.5 font-mono cursor-pointer">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_rsx" class="text-oxide font-medium">"copy rsx"</span>
                        </button>
                    </div>
                    <div class="flex bg-[#0b0b0d]">
                        <div class="select-none text-right pr-4 text-faint border-r border-rule font-mono text-[12px] leading-[1.75] py-4 pl-4">
                            "01"<br/>"02"<br/>"03"<br/>"04"<br/>"05"<br/>"06"<br/>"07"
                        </div>
                        <div class="code flex-1 pl-4 py-4 overflow-x-auto text-[13px] leading-[1.75]">
                            <div class="token-comment">"// Scaffolds secure authentication component with CSRF handling"</div>
                            <div><span class="token-tag">"<div "</span><span class="token-attr">"class="</span><span class="token-string">"\"plate max-w-lg\""</span><span class="token-tag">">"</span></div>
                            <div>"    "<span class="token-tag">"<form "</span><span class="token-attr">"method="</span><span class="token-string">"\"POST\""</span>" "<span class="token-attr">"action="</span><span class="token-string">"\"/api/login\""</span><span class="token-tag">">"</span></div>
                            <div>"        "<span class="token-tag">"<input "</span><span class="token-attr">"type="</span><span class="token-string">"\"hidden\""</span>" "<span class="token-attr">"name="</span><span class="token-string">"\"_csrf\""</span>" "<span class="token-attr">"value="</span>"{csrf_token}"<span class="token-tag">" />"</span></div>
                            <div>"        "<span class="token-tag">"<button "</span><span class="token-attr">"class="</span><span class="token-string">"\"btn btn-oxide\""</span><span class="token-tag">">"</span>"\"Sign in\""<span class="token-tag">"</button>"</span></div>
                            <div>"    "<span class="token-tag">"</form>"</span></div>
                            <div><span class="token-tag">"</div>"</span></div>
                        </div>
                    </div>
                    <div class="ide-status">
                        <span data-i18n="comp.auth.ide_l">"Strict compile-time template · Zero runtime eval"</span>
                        <span class="flex items-center gap-1.5"><span class="dot dot-ok"></span>"Argon2id"</span>
                    </div>
                </div>

                // Terminal Scaffolding Command
                <div class="term-window">
                    <div class="term-head">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-1.5">
                                <span class="w-2.5 h-2.5 rounded-full bg-[#e0592f]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#eab308]/70 inline-block"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-[#22c55e]/70 inline-block"></span>
                            </div>
                            <span class="text-faint text-[11px] ml-2" data-i18n="comp.common.term_head">"bash — yav-cli scaffold"</span>
                        </div>
                        <button onclick="copyText(this, 'yav add auth-card')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" data-i18n="common.copy_cmd" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add auth-card"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/auth_card.rs [3.1 KB pure RSX]"</div>
                    </div>
                </div>
            </div>

            <script>
                {r#"
                function toggleDemoPassword(btn) {
                    var input = document.getElementById('demo-password-field');
                    if (!input || !btn) return;
                    input.type = input.type === 'password' ? 'text' : 'password';
                    var isHidden = input.type === 'password';
                    var key = isHidden ? 'comp.auth.show' : 'comp.auth.hide';
                    var fb = isHidden ? 'Show' : 'Hide';
                    btn.setAttribute('data-i18n', key);
                    btn.textContent = window.yavT ? window.yavT(key, fb) : fb;
                }
                "#}
            </script>
        </section>
    }
}
