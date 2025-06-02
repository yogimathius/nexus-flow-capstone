## 📁 Project Structure with `mod.rs` (Scalable + Clean)

```
.
├── src/
│   ├── main.rs               # Launch Axum server (only setup + boot)
│   ├── lib.rs                # AppState, router aggregation
│
│   ├── api/                  # Axum route handlers
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   └── dream.rs
│
│   ├── services/             # Core business logic
│   │   ├── mod.rs
│   │   ├── ai_client.rs
│   │   └── mcp_client.rs
│
│   ├── models/               # DB structs + SQLx ops
│   │   ├── mod.rs
│   │   └── dream.rs
│
│   ├── db/                   # DB connection and migration logic
│   │   ├── mod.rs
│   │   └── migrate.rs
│
│   └── config.rs             # Loads and validates .env variables
│
├── migrations/              # sqlx migrations (CLI-compatible)
├── tests/                   # Integration tests (Router or app-level)
│   └── dream_routes.rs
├── .env
├── Cargo.toml
└── README.md
```

## ✅ Dream Interpreter Server — v1.5 Completion Checklist

### 🧠 Core Functionality

- [ ] `POST /api/interpret_dream`  
      Accepts raw dream text, returns structured AI interpretation  
      ✅ Handles both OpenRouter and Azure fallback  
      ✅ Uses Jungian-style symbolic interpretation prompt  
      ✅ Returns `interpretation` and `status` fields in response

- [ ] `POST /api/dreams`  
      Saves a dream + its AI interpretation to the database  
      ✅ Returns saved dream ID + confirmation status

- [ ] `GET /api/dreams`  
      Returns all dreams in reverse chronological order  
      ✅ Includes `id`, `text`, `interpretation`, and `created_at`

- [ ] `GET /api/dreams/:id`  
      Returns a specific dream and its interpretation  
      ✅ Returns 404 if not found

- [ ] `GET /health`  
      Returns 200 with `"ok"` for basic service health check

---

### 📦 Database Requirements (SQLx)

- [ ] `Dream` table migration is created and applied via `sqlx migrate run`

  ```sql
  CREATE TABLE dreams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    text TEXT NOT NULL,
    interpretation TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
  );
  ```

- [ ] SQLx model exists for `Dream` with `FromRow` and `Insertable` support

- [ ] Functions implemented:
  - [ ] `create_dream(pool, text, interpretation)`
  - [ ] `get_dream_by_id(pool, id)`
  - [ ] `list_dreams(pool)`

---

### 🔁 AI + MCP Integration

- [ ] AI prompt includes:

  - Archetypal analysis
  - Symbol meaning
  - Emotional resonance
  - Jungian tone

- [ ] Azure fallback triggered on:

  - Rate limiting
  - Model unavailability

- [ ] MCP client integrated (placeholder or reused from existing ontology server)
  - [ ] Optional: `/api/process_text` with symbolic MCP operation support

---

### 🧪 Minimum Test Coverage

- [ ] Unit test for AI client with mock JSON
- [ ] DB insert + fetch round-trip tested with `sqlx::test`
- [ ] Integration test for `/api/interpret_dream` (Leptos-compatible)
- [ ] At least one test using a real `.env` loaded with `dotenvy`

---

### 🔐 Environment & Config

- [ ] `.env` contains:

  - `OPENROUTER_API_KEY`
  - `DATABASE_URL`
  - Optional: `GITHUB_TOKEN`

- [ ] Config is loaded via a centralized `config.rs` file

- [ ] `AppState` holds:
  - SQLx `PgPool`
  - `AiClient`

---

### 🧩 Leptos CSR Compatibility

- [ ] All endpoints return JSON
- [ ] CORS headers allow requests from frontend dev server (`localhost:3000`)
- [ ] Errors return with `{ status: "error", message: "..."}`
- [ ] Loading states and empty states are supported (`[]` on no dreams)

---

### 🧹 Final Touches

- [ ] `cargo check`, `fmt`, and `clippy` all clean
- [ ] `README.md` includes:
  - Install instructions
  - How to run locally
  - Example `curl` commands
- [ ] `sqlx` migrations committed
- [ ] One sample dream seeded (optional for frontend demo)
