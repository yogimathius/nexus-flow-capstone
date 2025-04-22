## 🧭 App Layout Architecture (Leptos)

```
.
├── Layout
│   ├── <Header />         ⟶ Logo, nav links, dark mode toggle
│   ├── <RouterOutlet />   ⟶ Page content (SymbolList, SymbolDetail, etc)
│   └── <Footer />         ⟶ Simple credits or links (low footprint)
```

### ⚙️ Recommended File/Component Structure

```rust
components/
├── layout/
│   ├── header.rs
│   ├── footer.rs
│   └── layout.rs     // Wraps header, slot, footer
├── symbol/
│   ├── list.rs       // SymbolList view
│   ├── detail.rs     // SymbolDetail view
│   └── card.rs       // Reusable UI for symbol preview
├── search.rs         // Search bar + category filter
├── interpret.rs      // /interpret mock matching logic
routes/
├── index.rs          // Home: welcome + recent symbols
├── symbols.rs        // Route for list view
├── symbols/[id].rs   // Route for symbol detail
├── interpret.rs      // MVP mock symbol matcher
```

---

## 🧱 Layout Design Spec

### 🔝 Header

| Element          | Purpose                                |
| ---------------- | -------------------------------------- |
| App Logo/Name    | Clickable → Home (`/`)                 |
| Nav Links        | `/symbols`, `/interpret`               |
| Dark Mode Toggle | Toggle `data-theme="dark/light"` class |
| Optional AI CTA  | "Talk to AI" → Later release           |

```rust
view! {
  <header class="header">
    <div class="logo">"🧠 Dream AI"</div>
    <nav>
      <a href="/">"Home"</a>
      <a href="/symbols">"Symbols"</a>
      <a href="/interpret">"Interpret"</a>
    </nav>
    <button on:click=toggle_dark_mode>"🌓"</button>
  </header>
}
```

### 🔻 Footer

Minimalist:

```rust
view! {
  <footer class="footer">
    <p>"© 2025 Dream AI · Powered by MCP"</p>
  </footer>
}
```

---

## 📄 Pages in v1.5

| Route          | View Component  | Description                                                    |
| -------------- | --------------- | -------------------------------------------------------------- |
| `/`            | `Home`          | Welcome, intro, call to action + featured symbols              |
| `/symbols`     | `SymbolList`    | Browse all symbols, optionally filter by category or query     |
| `/symbols/:id` | `SymbolDetail`  | Full view of a symbol’s meaning, properties, and relationships |
| `/interpret`   | `InterpretMock` | Paste a dream → show matched symbols + mock interpretations    |
| `404`          | `NotFound`      | Soft fallback for unknown routes                               |

---

## 🪄 UI Visual Guidance

### Symbol Card (`SymbolList`)

| Element           | Content                              |
| ----------------- | ------------------------------------ |
| Name              | Big bold heading                     |
| Category Chip     | Color-coded badge (e.g. 🌿 `nature`) |
| Short Description | Subtle muted font below title        |
| Clickable         | Takes user to `/symbols/:id`         |

### Symbol Detail

| Section             | Content                                         |
| ------------------- | ----------------------------------------------- |
| Symbol Name         | H1                                              |
| Description         | Paragraph                                       |
| Interpretations     | Quote-style blocks or cards                     |
| Related Symbols     | Grid of linked symbol previews                  |
| Metadata (optional) | Archetype, pantheon, etc. styled as info badges |

---

## 🌘 Styling + UX Enhancements (for Premium Feel)

| Element          | Touch                                                       |
| ---------------- | ----------------------------------------------------------- |
| Hover Effects    | Soft scale + shadow                                         |
| Transitions      | `ease-out 150ms` for all                                    |
| Cards            | Rounded-xl, slight blur                                     |
| Font             | `Inter` + `JetBrains Mono` for AI data                      |
| Layout Max Width | `768px` or `container mx-auto`                              |
| Color Theme      | Based on your premium `#7C3AED` violet + deep neutral darks |

---

## 📦 Example Leptos Layout Integration

```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <RouterOutlet />
        <Footer />
    }
}
```

---

## 🧑‍💻 Bonus: Cursor Prompts for Consistent Components

```txt
Create a Leptos component called `SymbolCard` that takes `name`, `category`, and `description` as props and renders a clickable preview box. The component should use modern styling with Tailwind or inline styles if Stylance is disabled.
```
