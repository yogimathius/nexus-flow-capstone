# Nexus Flow frontend framework review

Date: 2026-10-29

Local stack validation updated: 2026-10-30.

## Decision

Keep Leptos for the next capstone stabilization pass. Do not rewrite the frontend merely to get a faster or more polished interface. If the UI needs a substantial redesign after the API contract and browser tests are sound, prototype one complete flow in SvelteKit before choosing a migration. Dioxus is the strongest Rust alternative for a cross-platform product, but Nexus Flow is currently a browser application.

This is a project-fit judgment, not a claim that one framework is universally faster or better for coding agents.

## Current framework landscape

| Option | Current evidence | Fit for Nexus Flow |
| --- | --- | --- |
| Leptos | Active 0.8 patch releases and a 0.9 release candidate; supports CSR, SSR, and islands. Its creator said in May 2026 that it is feature complete and will be lightly maintained. [Releases](https://github.com/leptos-rs/leptos/releases), [maintainer update](https://github.com/leptos-rs/leptos/issues/4707), [book](https://book.leptos.dev/) | Strong fit for a Rust-centered, web-only capstone. Maintenance pace is a long-term consideration. |
| Dioxus | Stable 0.7 line, 0.8 prereleases, integrated CLI, hot reload and web/desktop/mobile targets. [Releases](https://github.com/DioxusLabs/dioxus/releases), [hot reload](https://dioxuslabs.com/learn/0.7/essentials/ui/hotreload/), [platforms](https://dioxuslabs.com/learn/0.7/guides/platforms/) | Worth considering if desktop or mobile become real requirements. A web-only rewrite would have little direct benefit. |
| Yew | Active 0.23 release; CSR by default with SSR available. [Releases](https://github.com/yewstack/yew/releases), [SSR docs](https://yew.rs/docs/advanced-topics/server-side-rendering) | Mature option, but no clear advantage over the existing Leptos code for this project. |
| Sycamore | Active 0.9.4 release; fine-grained reactivity, SSR and routing. [Project](https://sycamore.dev/), [releases](https://github.com/sycamore-rs/sycamore/releases) | Similar conceptual model to Leptos, so migration cost is hard to justify. |
| SvelteKit (TypeScript) | Official support for SSR, CSR, prerendering, and adapters including Netlify and static output. [Introduction](https://svelte.dev/docs/kit/introduction), [adapters](https://svelte.dev/docs/kit/adapters) | Best candidate for a non-Rust UI pilot if frontend iteration becomes the bottleneck. Rust API and the separate TypeScript MCP can remain as they are. |

## Stabilization evidence

1. Leptos 0.8.2 now builds on pinned stable Rust 1.97.1 with Trunk; the moving nightly requirement was removed.
2. The API uses port 3003, migrations run on startup, and the frontend's API and health URLs now match the Rust routes.
3. Browser checks covered the home page, interpretation unavailable state, a successful mock interpretation and save, journal search, direct entry links, and a 390 px viewport. The mobile navigation overflow was fixed. A real provider response remains unverified because no provider key was supplied.
4. Saving is opt-in. The UI now explains that configured AI providers receive dream text and that PostgreSQL and browser storage retain saved entries. The local API is bound to loopback and has no account authentication. Retention and deletion controls remain product work before any public deployment.
5. Fabricated testimonials, fixed symbolic claims, and unsupported journal controls were removed from the active UI. The current Leptos stack has not been a demonstrated bottleneck, so a rewrite is not justified yet.

## Migration threshold

Consider SvelteKit only if, after the above work, Leptos repeatedly blocks UI changes, browser integration, or deployment. Compare a single interpretation-and-journal slice on measured build time, accessibility, testability, bundle/first-load behavior, and maintenance effort. Preserve the Rust API and the isolated Symbols Awakening MCP either way.

## If Nexus Flow becomes a mobile app

| Route | What it preserves | Main tradeoff |
| --- | --- | --- |
| Tauri 2 + current Leptos CSR app | Reuses most web UI and adds a Rust mobile shell for iOS and Android. [Tauri frontend guide](https://v2.tauri.app/start/frontend/), [Leptos guide](https://v2.tauri.app/start/frontend/leptos/) | WebView UI; the Leptos-specific guide is written for 0.6 and needs validation against this 0.8 app. Tauri does not package the separate Axum API or Node MCP automatically. |
| Dioxus | A Rust UI codebase can target web and mobile. [Mobile guide](https://dioxuslabs.com/learn/0.7/guides/platforms/mobile/) | Rewrites the existing Leptos UI; current mobile rendering uses WebView, while its native renderer is experimental. |
| Slint | Rust UI toolkit with Android and iOS support. [Mobile guide](https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/general/) | Native-style UI would require a new presentation layer. |
| SwiftUI/Kotlin + UniFFI, or Flutter + flutter_rust_bridge | Keeps shared Rust domain logic while using platform-native UI or Flutter. [UniFFI](https://mozilla.github.io/uniffi-rs/latest/), [Flutter Rust Bridge](https://cjycode.com/flutter_rust_bridge/) | More UI stacks and binding/build work. |

For a first mobile experiment, Tauri + the current Leptos CSR app has the smallest UI rewrite. It is a packaging and device-experience experiment, not an automatic way to run the current local stack on a phone. The current Nexus Core integration launches `node dist/index.js` as a child process; none of these mobile UI routes supplies that Node process. A self-contained offline mobile design would need a separate plan for symbol data and interpretation, such as moving portable domain logic into a Rust library. If the phone calls the existing Rust API instead, that API must be reachable from the phone.
