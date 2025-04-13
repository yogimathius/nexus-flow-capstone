## 🗂 Folder Structure (MVP)

```
mcp-insight/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── server.rs             # Axum routes and handlers
│   ├── symbols/
│   │   ├── mod.rs
│   │   ├── model.rs          # Symbol, SymbolSet, etc.
│   │   ├── loader.rs         # Load from JSON or DB
│   │   ├── mcp.rs            # MCP server implementation for symbols
│   ├── interpretation/
│   │   ├── mod.rs
│   │   ├── engine.rs         # Runs LLM prompt logic
│   │   ├── prompt.rs         # Templates for LLM
│   ├── clients/
│   │   ├── mod.rs
│   │   ├── llm.rs            # Reqwest-based OpenRouter LLM client
│   ├── ontologies/
│   │   ├── jungian.json      # Initial symbol sets
│   │   ├── mythic.json
│   └── utils.rs
```

---

## 🧱 Core Traits + Structs

### 🔤 `symbols/model.rs`

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub meanings: Vec<String>,
    pub related: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSet {
    pub source: String,
    pub symbols: Vec<Symbol>,
}
```

---

### 🔌 `symbols/loader.rs`

```rust
use crate::symbols::model::{SymbolSet};
use std::{collections::HashMap, fs};

pub fn load_symbol_set_from_file(path: &str) -> anyhow::Result<SymbolSet> {
    let content = fs::read_to_string(path)?;
    let set: SymbolSet = serde_json::from_str(&content)?;
    Ok(set)
}
```

---

### 🔗 `symbols/mcp.rs` (Basic MCP Handler)

```rust
use rmcp::{Handler, Request, Response};
use crate::symbols::model::SymbolSet;
use std::sync::Arc;

pub struct SymbolMcp {
    pub sets: Arc<Vec<SymbolSet>>, // Shared storage
}

#[async_trait::async_trait]
impl Handler for SymbolMcp {
    async fn handle(&self, req: Request) -> Response {
        match req.method.as_str() {
            "get_symbols" => {
                let query = req.params["query"].as_array().unwrap();
                let symbols: Vec<_> = self.sets.iter()
                    .flat_map(|set| set.symbols.iter())
                    .filter(|s| query.iter().any(|q| q == &s.name))
                    .cloned()
                    .collect();

                Response::success(serde_json::json!({ "symbols": symbols }))
            }
            _ => Response::error("Unknown method".to_string()),
        }
    }
}
```

---

### 🧠 `interpretation/engine.rs`

```rust
use crate::symbols::model::Symbol;
use crate::clients::llm::interpret_with_llm;

pub async fn interpret(symbols: Vec<Symbol>) -> anyhow::Result<String> {
    let prompt = super::prompt::build_prompt(&symbols);
    let interpretation = interpret_with_llm(&prompt).await?;
    Ok(interpretation)
}
```

---

### 🧾 `interpretation/prompt.rs`

```rust
use crate::symbols::model::Symbol;

pub fn build_prompt(symbols: &[Symbol]) -> String {
    let mut out = String::from("Given the dream symbols:\n");
    for sym in symbols {
        out.push_str(&format!(
            "- {}: {}\n",
            sym.name,
            sym.meanings.join(", ")
        ));
    }
    out.push_str("\nGenerate an interpretation.\n");
    out
}
```

---

### 🤖 `clients/llm.rs`

```rust
use reqwest::Client;

pub async fn interpret_with_llm(prompt: &str) -> anyhow::Result<String> {
    let api_url = "https://openrouter.ai/api/..."; // Adjust for DeepSeek
    let payload = serde_json::json!({
        "model": "deepseek-coder",
        "prompt": prompt,
    });

    let client = Client::new();
    let res = client.post(api_url)
        .header("Authorization", "Bearer YOUR_KEY")
        .json(&payload)
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;
    Ok(body["response"].as_str().unwrap_or("").to_string())
}
```

---

## 🛠 `main.rs` – bootstraps it all

```rust
mod symbols;
mod interpretation;
mod clients;

use axum::{Router, routing::post};
use symbols::loader::load_symbol_set_from_file;
use std::sync::Arc;
use symbols::mcp::SymbolMcp;

#[tokio::main]
async fn main() {
    let jungian = load_symbol_set_from_file("ontologies/jungian.json").unwrap();
    let sets = Arc::new(vec![jungian]);

    let mcp = SymbolMcp { sets: Arc::clone(&sets) };
    rmcp::serve("localhost:9000", mcp); // Serve MCP

    let app = Router::new().route("/interpret", post(handlers::interpret));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```
