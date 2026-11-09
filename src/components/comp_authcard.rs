//! 5.5 Auth Card & Credentials Component Showroom.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ComponentAuthCard() -> Node {
    rsx! {
        <section id="authcard" class="space-y-6 scroll-mt-24">
            <div class="sec-head">
                <span class="n">"5.5"</span>
                <span class="t">"Auth Card & Credentials"</span>
                <span class="sec-rule"></span>
                <span class="r">"yav add auth-card"</span>
            </div>
            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-2 pt-2">
                <h2 class="text-[30px] sm:text-[34px] text-paper leading-tight">"Security-first auth card."</h2>
                <button
                    onclick="copyText(this, 'yav add auth-card')"
                    class="self-start sm:self-auto font-mono text-[11px] text-dim hover:text-paper bg-ink-2 border border-rule px-2.5 py-1 rounded flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                    <span class="text-oxide">"$"</span>
                    <span>"yav add auth-card"</span>
                    <span data-copy-label="" class="text-oxide font-medium border-l border-rule pl-1.5">"copy"</span>
                </button>
            </div>
            <p class="text-mute text-[15.5px] leading-relaxed">
                "Complete authentication panel for engineering consoles and SaaS dashboards. Integrates single-sign-on OAuth providers, Argon2 password hashing hooks, and strict HMAC CSRF tokens right out of the box."
            </p>

            // Live Interactive Preview Plate
            <div class="plate max-w-lg mx-auto">
                <span class="tick tl"></span><span class="tick tr"></span><span class="tick bl"></span><span class="tick br"></span>
                <div class="plate-head">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-ok"></span>
                        "AUTH CARD PREVIEW"
                    </span>
                    <span class="text-faint font-mono text-[11px]">"HMAC verified"</span>
                </div>
                <div class="p-8 space-y-6 bg-ink-2">
                    // Card Title
                    <div class="space-y-1 text-center">
                        <div class="text-[20px] font-semibold text-paper">"Sign in to Console"</div>
                        <p class="text-xs text-mute">"Enter your engineer credentials to access the cluster"</p>
                    </div>

                    // OAuth Button
                    <button class="w-full btn btn-line flex items-center justify-center gap-2.5 py-2.5 text-xs font-mono">
                        {render_icon("git-branch", IconSize::Sm, Some("w-4 h-4 text-paper"), None)}
                        <span>"Continue with GitHub OAuth"</span>
                    </button>

                    // Divider
                    <div class="relative flex items-center justify-center">
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full border-t border-[rgba(236,233,227,0.08)]"></div>
                        </div>
                        <span class="relative px-3 bg-ink-2 text-[10px] font-mono uppercase tracking-widest text-dim">
                            "or credentials"
                        </span>
                    </div>

                    // Form inputs
                    <form class="space-y-4" onsubmit="event.preventDefault();">
                        <div class="space-y-1.5">
                            <label class="label block text-[11px]">"Email Address"</label>
                            <input
                                type="email"
                                class="input-field text-xs font-mono"
                                placeholder="engineer@domain.rs"
                                value="admin@cluster.internal"
                            />
                        </div>

                        <div class="space-y-1.5">
                            <div class="flex items-center justify-between">
                                <label class="label text-[11px]">"Password"</label>
                                <a href="#authcard" class="text-[11px] font-mono text-oxide hover:underline">"Forgot?"</a>
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
                                    onclick="let input = document.getElementById('demo-password-field'); input.type = input.type === 'password' ? 'text' : 'password'; this.textContent = input.type === 'password' ? 'Show' : 'Hide';"
                                    class="absolute right-3 font-mono text-[10px] text-dim hover:text-paper uppercase tracking-wider"
                                >
                                    "Show"
                                </button>
                            </div>
                        </div>

                        <div class="flex items-center justify-between pt-1">
                            <label class="flex items-center gap-2 cursor-pointer text-xs font-mono text-dim">
                                <input type="checkbox" checked="" class="accent-[#e0592f]" />
                                <span>"Remember 30 days"</span>
                            </label>
                            <span class="text-[10px] font-mono text-ok flex items-center gap-1">
                                <span class="dot dot-ok"></span>
                                "256-bit AES"
                            </span>
                        </div>

                        <button type="submit" class="w-full btn btn-oxide justify-center py-2.5 mt-2">
                            <span>"Authenticate Session"</span>
                            <span class="arr">"→"</span>
                        </button>
                    </form>

                    // Security Footer Note
                    <div class="pt-3 border-t border-[rgba(236,233,227,0.08)] flex items-center justify-between text-[11px] font-mono text-dim">
                        <span>"Strict SameSite=Lax"</span>
                        <span class="text-faint">"CSRF token active"</span>
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
                            <span data-copy-label="" class="text-oxide font-medium">"copy rsx"</span>
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
                        <span>"Strict compile-time template · Zero runtime eval"</span>
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
                            <span class="text-faint text-[11px] ml-2">"bash — yav-cli scaffold"</span>
                        </div>
                        <button onclick="copyText(this, 'yav add auth-card')" class="text-dim hover:text-oxide transition-colors cursor-pointer text-[11px] flex items-center gap-1.5 font-mono">
                            {render_icon("copy", IconSize::Sm, Some("w-3 h-3 text-oxide"), None)}
                            <span data-copy-label="" class="text-oxide font-medium">"copy command"</span>
                        </button>
                    </div>
                    <div class="p-4 code space-y-1 text-[13px] bg-[#070709]">
                        <div><span class="text-oxide">"$ "</span>"yav add auth-card"</div>
                        <div class="text-faint text-[11px]">"✓ Generated src/components/auth_card.rs [3.1 KB pure RSX]"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
