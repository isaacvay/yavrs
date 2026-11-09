//! Cryptographic Sessions & Security Architecture Component (Airy & Generous).

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};

#[component]
pub fn ArchSecurity() -> Node {
    rsx! {
        <section id="security" class="wrap pt-12 sm:pt-16">
            <div class="sec-head reveal">
                <span class="n">"2.4"</span>
                <span class="t">"Security Architecture"</span>
                <span class="sec-rule"></span>
                <span class="r">"zero external state"</span>
            </div>

            <div class="pt-6 grid lg:grid-cols-12 gap-10 lg:gap-14 items-start pb-6">
                <div class="lg:col-span-5 space-y-8 reveal pl-2 sm:pl-4">
                    <div class="space-y-4 pl-5 sm:pl-7 border-l-2 border-oxide/50">
                        <h2 class="h-section text-paper leading-[1.2]">
                            "Cryptographic sessions. "<span class="serif text-oxide">"Zero Redis dependency."</span>
                        </h2>
                        <p class="text-[15.5px] text-mute leading-[1.8]">
                            "Traditional frameworks query a Redis cluster or database on every single incoming HTTP request just to validate the user's session ID. This introduces 2 to 8 ms of network round-trip latency, connection pool limits, and a critical single point of failure."
                        </p>
                    </div>

                    <p class="text-[15.5px] text-mute leading-[1.8] pl-5 sm:pl-7">
                        "Yav employs <strong class=\"text-paper font-semibold\">tamper-proof cryptographic cookie sessions</strong> signed with HMAC-SHA256 (or encrypted with AES-256-GCM). Validation happens completely in-memory via constant-time comparison, yielding microsecond validation with zero external infrastructure."
                    </p>
                </div>

                <div class="lg:col-span-7 reveal" style="--d:120ms">
                    <div class="cells grid-cols-1 sm:grid-cols-2">
                        <div class="cell p-8 sm:p-10 pl-8 sm:pl-10 space-y-4">
                            <div class="flex items-center gap-2.5 text-oxide font-mono text-[14px] font-semibold">
                                {render_icon("shield", IconSize::Sm, Some("w-4 h-4"), None)}
                                <span>"HMAC-SHA256 Signing"</span>
                            </div>
                            <p class="text-[14px] text-mute leading-[1.75]">
                                "Payload is signed with a secret master key. Signatures are verified in constant time via <code class=\"text-paper\">subtle::ConstantTimeEq</code>, completely immune to timing side-channel attacks."
                            </p>
                            <div class="dim-line pt-4 font-mono text-[11.5px]">
                                <span>"crypto"</span><span class="bar"></span><span>"constant-time"</span>
                            </div>
                        </div>

                        <div class="cell p-8 sm:p-10 pl-8 sm:pl-10 space-y-4">
                            <div class="flex items-center gap-2.5 text-oxide font-mono text-[14px] font-semibold">
                                {render_icon("lock", IconSize::Sm, Some("w-4 h-4"), None)}
                                <span>"Double-Submit CSRF"</span>
                            </div>
                            <p class="text-[14px] text-mute leading-[1.75]">
                                "Every mutating form is automatically supplied with a cryptographic nonce. The server validates that the cookie nonce and the form payload match byte-for-byte."
                            </p>
                            <div class="dim-line pt-4 font-mono text-[11.5px]">
                                <span>"protection"</span><span class="bar"></span><span>"automatic"</span>
                            </div>
                        </div>

                        <div class="cell p-8 sm:p-10 pl-8 sm:pl-10 space-y-4">
                            <div class="flex items-center gap-2.5 text-oxide font-mono text-[14px] font-semibold">
                                {render_icon("server", IconSize::Sm, Some("w-4 h-4"), None)}
                                <span>"Zero-Infrastructure Auth"</span>
                            </div>
                            <p class="text-[14px] text-mute leading-[1.75]">
                                "Deploy across 20 global regions on bare metal or edge containers without maintaining a distributed Redis cache. Session state travels safely with the user."
                            </p>
                            <div class="dim-line pt-4 font-mono text-[11.5px]">
                                <span>"state"</span><span class="bar"></span><span>"stateless edge"</span>
                            </div>
                        </div>

                        <div class="cell p-8 sm:p-10 pl-8 sm:pl-10 space-y-4">
                            <div class="flex items-center gap-2.5 text-oxide font-mono text-[14px] font-semibold">
                                {render_icon("check", IconSize::Sm, Some("w-4 h-4"), None)}
                                <span>"Hardened Security Headers"</span>
                            </div>
                            <p class="text-[14px] text-mute leading-[1.75]">
                                "Automatic injection of <code class=\"text-paper\">SameSite=Lax</code>, <code class=\"text-paper\">HttpOnly</code>, <code class=\"text-paper\">Secure</code>, strict Content-Security-Policy, and HSTS."
                            </p>
                            <div class="dim-line pt-4 font-mono text-[11.5px]">
                                <span>"score"</span><span class="bar"></span><span>"A+ Observatory"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
