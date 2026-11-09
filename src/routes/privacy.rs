//! Privacy Policy & Technical Security Specifications — Editorial Blueprint.

use yav::prelude::*;
use yav::icons::{render_icon, IconSize};
use axum::extract::Request;
use crate::layouts::*;

pub async fn privacy(_req: Request) -> yav::server::HtmlPage {
    let page = rsx! {
        <AppLayout>
            <div class="wrap py-12 sm:py-16 max-w-4xl space-y-12">
                // Header
                <div class="space-y-4">
                    <div class="inline-flex items-center gap-2 px-3 py-1 bg-ink-2 border border-rule font-mono text-[11px] text-oxide">
                        {render_icon("shield-check", IconSize::Sm, Some("w-3.5 h-3.5 text-ok"), None)}
                        <span data-i18n="privacy.badge">"DATA ETHICS & TECHNICAL SECURITY SPECIFICATION"</span>
                    </div>
                    <h1 class="text-[36px] sm:text-[48px] font-bold text-paper tracking-[-0.03em] leading-tight" data-i18n="privacy.title">
                        "Privacy Policy & Security Model"
                    </h1>
                    <p class="text-[16px] text-mute leading-relaxed max-w-2xl" data-i18n="privacy.subtitle">
                        "Yav is built on the principle of minimal data footprint. We collect zero tracking analytics, transmit no client-side telemetry, and enforce cryptographic security by default."
                    </p>
                </div>

                <div class="space-y-10 font-sans text-mute leading-relaxed text-[15px]">
                    // 1. Zero Tracking
                    <section class="plate p-6 bg-ink-2 space-y-3">
                        <div class="text-paper font-semibold text-lg flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-sm">"01."</span>
                            "Zero Tracking Pixels & No Third-Party Telemetry"
                        </div>
                        <p>
                            "This website does not load Google Analytics, Meta Pixel, Hotjar, Mixpanel, or any third-party marketing trackers. There are no external scripts or ad networks loaded into your browser context. When you inspect network traffic during page navigation, you will find 100% first-party requests served strictly from our edge binary."
                        </p>
                    </section>

                    // 2. Cookie Hardening
                    <section class="plate p-6 bg-ink-2 space-y-3">
                        <div class="text-paper font-semibold text-lg flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-sm">"02."</span>
                            "Cryptographic Sessions & Cookie Architecture"
                        </div>
                        <p>
                            "When interactive server sessions or authentication flows are engaged, Yav employs stateless HMAC-SHA256 cookies. Every cookie adheres strictly to modern web safety flags:"
                        </p>
                        <ul class="list-disc pl-5 space-y-1.5 font-mono text-xs text-paper/90">
                            <li><strong class="text-oxide">"HttpOnly"</strong>" — Inaccessible to JavaScript, eliminating XSS token theft."</li>
                            <li><strong class="text-oxide">"Secure"</strong>" — Transmitted strictly over encrypted TLS/HTTPS connections."</li>
                            <li><strong class="text-oxide">"SameSite=Lax"</strong>" — Enforces cross-site request isolation against CSRF exploits."</li>
                            <li><strong class="text-oxide">"CRLF Injection Guard"</strong>" — Cookie names and values are rejected if containing ;, \r, \n, or \0."</li>
                        </ul>
                    </section>

                    // 3. Flash Messages & Replay Attacks
                    <section class="plate p-6 bg-ink-2 space-y-3">
                        <div class="text-paper font-semibold text-lg flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-sm">"03."</span>
                            "Flash Notifications & Replay Prevention"
                        </div>
                        <p>
                            "One-time flash messages (e.g. form success notifications or error alerts) embed cryptographically signed 5-minute UNIX timestamps (DEFAULT_FLASH_TTL_SECS = 300). Expired or tampered flash tokens are discarded in constant time before rendering."
                        </p>
                    </section>

                    // 4. Local Storage & Client State
                    <section class="plate p-6 bg-ink-2 space-y-3">
                        <div class="text-paper font-semibold text-lg flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-sm">"04."</span>
                            "Local Storage Usage"
                        </div>
                        <p>
                            "Client-side storage is restricted strictly to user interface ergonomics: your preferred language locale (key: "
                            <code class="inline">"yav-lang"</code>
                            ") and system dark mode state. No personally identifiable information (PII) is ever written to localStorage."
                        </p>
                    </section>

                    // 5. Open-Source Licensing & Sovereignty
                    <section class="plate p-6 bg-ink-2 space-y-3">
                        <div class="text-paper font-semibold text-lg flex items-center gap-2.5">
                            <span class="text-oxide font-mono text-sm">"05."</span>
                            "Open-Source Licensing (MIT & Apache-2.0)"
                        </div>
                        <p>
                            "The Yav framework, its core crates, CLI toolbelt, and this documentation website are dual-licensed under MIT and Apache-2.0. You have complete legal freedom to inspect the source code, compile the binary locally, audit dependencies, self-host on private infrastructure, and verify runtime behavior."
                        </p>
                        <div class="pt-2 text-xs font-mono text-dim">
                            "Last revised: October 25, 2026 · Maintained by Isaac Yav and contributors."
                        </div>
                    </section>
                </div>
            </div>
        </AppLayout>
    };

    website_page(
        "Privacy & Security Specifications — Yav.rs",
        "Technical privacy policy, zero-tracking guarantee, cryptographic cookie handling, and MIT/Apache-2.0 open-source licensing.",
        "/privacy",
        page,
    )
}
