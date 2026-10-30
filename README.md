# Nexus Flow

Nexus Flow is a local dream reflection app. A Leptos WebAssembly frontend talks to a Rust API, which stores journal entries in PostgreSQL. When enabled, the API starts the independent [Symbols Awakening MCP](https://github.com/yogimathius/symbols-awakening-mcp) Node process over stdio to add symbol context to an interpretation prompt.

Dream meanings are prompts for reflection, not diagnoses or fixed claims about a person.

## What works now

- Write a dream and request an AI interpretation from a configured OpenRouter or GitHub Models provider.
- Choose whether to save an interpreted dream. Saving is off by default.
- Browse, search, and open saved entries, including direct entry links.
- Use the separate TypeScript MCP locally in demo mode, or connect it to its own database.
- Add or remove dream tags through the Rust API. The web journal currently exposes text search and refresh.

The symbol library page is a placeholder. The older Rust `symbol-ontology/` MCP remains in the repository for reference and optional SSE use.

## Local setup

You need Rust 1.97.1, the `wasm32-unknown-unknown` target, Trunk, Node.js, pnpm, and PostgreSQL. On macOS with Homebrew:

```bash
brew install postgresql@16 trunk
rustup target add wasm32-unknown-unknown --toolchain 1.97.1
/opt/homebrew/opt/postgresql@16/bin/pg_ctl -D /opt/homebrew/var/postgresql@16 -l /tmp/nexus-flow-postgres.log start
```

Clone `nexus-flow-capstone` and `symbols-awakening-mcp` beside each other. Build the MCP once:

```bash
cd ../symbols-awakening-mcp
pnpm install --frozen-lockfile
pnpm build
```

From the Nexus Flow repository root, start the API:

```bash
export DATABASE_URL="postgresql://${USER}@127.0.0.1:5432/nexus_core"
export ENABLE_MCP=true
export MCP_TRANSPORT=stdio
export SYMBOLS_MCP_ENTRY="$(realpath ../symbols-awakening-mcp/dist/index.js)"
export SYMBOLS_MCP_DEMO=true
# Set one real provider key to enable interpretation:
# export OPENROUTER_API_KEY=...
# export GITHUB_TOKEN=...
cd nexus-core
cargo run -- --port 3003
```

The API creates the `nexus_core` database if the PostgreSQL role has permission and applies its migrations at startup. It listens on `127.0.0.1:3003`. Without an AI provider key, interpretation requests return HTTP 503; the journal and MCP still run.

In another terminal, from the repository root:

```bash
cd nexus-web
trunk serve --port 3004
```

Open [the local app](http://localhost:3004). The frontend defaults to `http://localhost:3003/api`. Set `API_URL` before `trunk build` or `trunk serve` to use another API origin or API root. The health endpoint is `/health` at that origin.

For a persistent Symbols Awakening dataset, set `SYMBOLS_MCP_DEMO=false` and `SYMBOLS_MCP_DATABASE_URL` to that project's PostgreSQL URL. The Rust API still launches it as a separate local child process; no MCP port or deployment is required.

## Data and access

The local API has no user authentication and is bound to loopback. Do not expose it publicly without adding access control. An interpretation sends the dream text and symbol context to the configured AI provider. Saving stores the text and interpretation in PostgreSQL; the web journal also caches fetched entries in this browser's local storage for offline reading. Clearing browser storage removes that cache, not the database records.

## Verify

```bash
cd nexus-core && cargo test
cd ../nexus-web && cargo test --lib && trunk build
cd ../symbols-awakening-mcp && pnpm test
```

With PostgreSQL running and `DATABASE_URL` set, the ignored database security test can be run explicitly:

```bash
cd nexus-core
cargo test --test dream_model_security -- --ignored
```

The optional cross-language MCP test also requires `SYMBOLS_MCP_ENTRY` and `SYMBOLS_MCP_DEMO=true`:

```bash
cd nexus-core
cargo test connects_to_local_symbols_awakening_server --lib -- --ignored
```

API routes include `GET /health`, `POST /api/interpret_dream`, `GET` and `POST /api/dreams`, `GET /api/dreams/{id}`, `GET /api/dreams/search`, and dream tag routes. See [Nexus Core's README](nexus-core/README.md) for request examples.
