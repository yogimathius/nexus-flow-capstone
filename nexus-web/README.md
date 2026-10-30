# Nexus Web

Nexus Web is the Leptos WebAssembly frontend for [Nexus Flow](../README.md). It provides dream interpretation, an opt-in saved journal, text search, and direct journal entry links.

## Run locally

The repository pins Rust 1.97.1 and the `wasm32-unknown-unknown` target. Install [Trunk](https://trunkrs.dev/) and start Nexus Core on port 3003, then run:

```bash
trunk serve --port 3004
```

Open `http://localhost:3004`. The default API root is `http://localhost:3003/api`. To point at another API, set `API_URL` before building or serving. Both an origin (`http://localhost:3003`) and an API root (`http://localhost:3003/api`) work.

```bash
API_URL=http://localhost:3003/api trunk build
cargo test --lib
```

Interpretation requires a provider key on Nexus Core. With no key configured, the form shows an unavailable error. Saving is off by default. Saved dreams live in the API's PostgreSQL database; the journal caches fetched pages in browser local storage for offline reading. This local app has no account login, so browser storage and database access should be treated accordingly.

The symbol library route is a placeholder. Category, date, and favorite controls are withheld from the journal until the API can support them consistently.
