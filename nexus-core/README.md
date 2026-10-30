# Nexus Core

Nexus Core is Nexus Flow's Rust API. It stores journal entries in PostgreSQL, calls a configured AI provider for interpretations, and can launch the separate Symbols Awakening TypeScript MCP server as a local child process over stdio. See the [repository README](../README.md) for full local setup.

## Configuration

`DATABASE_URL` is required. The API creates the target database when the PostgreSQL role permits it, then applies migrations from `migrations/` before listening. `PORT` defaults to `3003`; `cargo run -- --port 3003` overrides it. The listener binds to `127.0.0.1` for local use.

Set `OPENROUTER_API_KEY` or `GITHUB_TOKEN` to enable interpretations. With neither configured, `POST /api/interpret_dream` returns HTTP 503. A provider failure is returned as an error rather than a successful interpretation.

For local MCP integration:

```bash
ENABLE_MCP=true
MCP_TRANSPORT=stdio
SYMBOLS_MCP_ENTRY=/absolute/path/to/symbols-awakening-mcp/dist/index.js
SYMBOLS_MCP_DEMO=true
```

Use `SYMBOLS_MCP_DEMO=false` plus `SYMBOLS_MCP_DATABASE_URL` for that server's persistent dataset. For the legacy Rust MCP, set `MCP_TRANSPORT=sse` and `MCP_SERVER_URL` to its SSE endpoint. The API continues without symbol enrichment if the MCP cannot initialize.

## Routes

| Method | Route | Purpose |
| --- | --- | --- |
| GET | `/health` | API process health |
| POST | `/api/interpret_dream` | Interpret a dream; `save` defaults to false |
| GET, POST | `/api/dreams` | List or create journal entries |
| GET | `/api/dreams/{id}` | Read an entry |
| GET | `/api/dreams/search?search=...` | Full text search |
| POST | `/api/dreams/{id}/tags` | Add a tag |
| DELETE | `/api/dreams/{id}/tags/{tag}` | Remove a tag |

Example request:

```bash
curl -i http://127.0.0.1:3003/api/interpret_dream \
  -H 'Content-Type: application/json' \
  -d '{"dream_text":"I walked beside a quiet ocean under a bright moon.","save":false}'
```

The API has no authentication and is intended for local use. Interpretation text is sent to the configured AI provider. Raw dream text is not written to the API logs.

## Verify

```bash
cargo test
DATABASE_URL=postgresql://USER@127.0.0.1:5432/nexus_core \
  cargo test --test dream_model_security -- --ignored
```

The MCP integration test is also ignored by default; run `cargo test connects_to_local_symbols_awakening_server --lib -- --ignored` after setting `SYMBOLS_MCP_ENTRY` and `SYMBOLS_MCP_DEMO=true`.
